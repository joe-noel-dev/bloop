use super::{
    process::Process,
    sampler_converter::{SampleConversionResult, SampleConverter},
    sequencer::Sequencer,
};
use crate::{
    api::Response,
    model::{PlaybackState, PlayingState, Progress, Project, ID},
    samples::SamplesCache,
};
use futures::StreamExt;
use futures_channel::mpsc;
use rawdio::{create_engine, Context, Gain, Sampler};
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    time::Duration,
};
use tokio::sync::broadcast;

pub trait Audio {
    fn play(&mut self);
    fn stop(&mut self);
    fn enter_loop(&mut self);
    fn exit_loop(&mut self);
    fn queue(&mut self, song_id: &ID, section_id: &ID);

    fn toggle_loop(&mut self);
    fn toggle_play(&mut self);
}

pub struct AudioManager {
    context: Box<dyn Context>,
    _process: Process,

    sample_converter: SampleConverter,
    conversion_rx: mpsc::Receiver<SampleConversionResult>,
    response_tx: broadcast::Sender<Response>,
    samples_being_converted: HashSet<ID>,
    playback_state: PlaybackState,
    progress: Progress,
    samplers: HashMap<ID, Sampler>,
    project: Project,
    output_gain: Gain,
    sequencer: Sequencer,
}

impl AudioManager {
    pub fn new(response_tx: broadcast::Sender<Response>, preferences_dir: &Path) -> Self {
        let sample_rate = 44_100;
        let (mut context, process) = create_engine(sample_rate);

        context.start();

        let (conversion_tx, conversion_rx) = mpsc::channel(64);

        let channel_count = 2;
        let gain = Gain::new(context.as_ref(), channel_count);
        gain.node.connect_to_output();

        Self {
            context,
            _process: Process::new(process, preferences_dir),
            sample_converter: SampleConverter::new(conversion_tx, sample_rate),
            conversion_rx,
            response_tx,
            samples_being_converted: HashSet::new(),
            playback_state: PlaybackState::default(),
            progress: Progress::default(),
            samplers: HashMap::new(),
            project: Project::empty(),
            output_gain: gain,
            sequencer: Sequencer::default(),
        }
    }

    pub async fn run(&mut self) {
        let mut interval = tokio::time::interval(Duration::from_secs_f64(1.0 / 60.0));

        loop {
            tokio::select! {
                Some(conversion_result) = self.conversion_rx.next() => {
                    self.on_sample_converted(conversion_result)
                },
                _ = interval.tick() => {
                    self.context.process_notifications();
                    self.sequencer.set_current_time(self.context.current_time());
                    self.notify_playback_state();
                },
            }
        }
    }

    fn notify_playback_state(&mut self) {
        let playback_state = self.sequencer.get_playback_state();
        if self.playback_state != playback_state {
            self.playback_state = playback_state;
            let _ = self
                .response_tx
                .send(Response::default().with_playback_state(&self.playback_state));
        }

        let progress = self.sequencer.get_progress();
        if self.progress != progress {
            self.progress = progress;
            let _ = self.response_tx.send(Response::default().with_progress(self.progress));
        }
    }

    pub fn playback_state(&self) -> &PlaybackState {
        &self.playback_state
    }

    fn on_sample_converted(&mut self, result: SampleConversionResult) {
        self.samples_being_converted.remove(&result.sample_id);

        let audio_data = match result.result {
            Ok(data) => data,
            Err(error) => {
                eprintln!("Error converting audio file {}: {}", result.sample_id, error);
                return;
            }
        };

        println!("Adding sample to the audio engine: {}", result.sample_id);

        let sampler = Sampler::new(self.context.as_ref(), audio_data);
        sampler.node.connect_to(&self.output_gain.node);
        self.samplers.insert(result.sample_id, sampler);
    }

    fn add_samples(&mut self, project: &Project, samples_cache: &SamplesCache) {
        for sample in project.samples.iter() {
            if self.samplers.contains_key(&sample.id) {
                continue;
            }

            let cached_sample = match samples_cache.get_sample(&sample.id) {
                Some(sample) => sample,
                None => continue,
            };

            if !cached_sample.is_cached() {
                continue;
            }

            self.add_sample(&sample.id, cached_sample.get_path().to_path_buf());
        }
    }

    fn add_sample(&mut self, sample_id: &ID, sample_path: PathBuf) {
        if self.samplers.contains_key(sample_id) {
            return;
        }

        if self.samples_being_converted.contains(sample_id) {
            return;
        }

        self.samples_being_converted.insert(*sample_id);
        self.sample_converter.convert(*sample_id, sample_path);
    }

    fn remove_samples(&mut self, project: &Project) {
        let samples_to_remove: HashSet<ID> = self
            .samplers
            .iter()
            .filter(|(sample_id, _)| !project.samples.iter().any(|sample| &sample.id == *sample_id))
            .map(|(sample_id, _)| sample_id)
            .copied()
            .collect();

        for sample_id in samples_to_remove {
            self.remove_sample(&sample_id);
        }
    }

    fn remove_sample(&mut self, sample_id: &ID) {
        if let Some(mut sampler) = self.samplers.remove(sample_id) {
            sampler.node.disconnect_from_node(&self.output_gain.node);
            sampler.stop_now();
        }
    }

    pub fn on_project_updated(&mut self, project: &Project, samples_cache: &SamplesCache) {
        self.add_samples(project, samples_cache);
        self.project = project.clone();
        self.remove_samples(project);
    }
}

impl Audio for AudioManager {
    fn play(&mut self) {
        self.sequencer.set_current_time(self.context.current_time());
        self.sequencer.play(self.project.clone(), &mut self.samplers);
    }

    fn stop(&mut self) {
        self.sequencer.set_current_time(self.context.current_time());
        self.sequencer.stop(&mut self.samplers);
    }

    fn enter_loop(&mut self) {
        self.sequencer.set_current_time(self.context.current_time());
        self.sequencer.enter_loop(&mut self.samplers);
    }

    fn exit_loop(&mut self) {
        self.sequencer.set_current_time(self.context.current_time());
        self.sequencer.exit_loop(&mut self.samplers);
    }

    fn queue(&mut self, song_id: &ID, section_id: &ID) {
        self.sequencer.set_current_time(self.context.current_time());
        self.sequencer.queue(song_id, section_id, &mut self.samplers);
    }

    fn toggle_loop(&mut self) {
        if self.playback_state.looping {
            self.exit_loop();
        } else {
            self.enter_loop();
        }
    }

    fn toggle_play(&mut self) {
        match self.playback_state.playing {
            PlayingState::Stopped => self.play(),
            PlayingState::Playing => self.stop(),
        }
    }
}
