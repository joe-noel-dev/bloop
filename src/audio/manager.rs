use super::{
    metronome::Metronome,
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
use rawdio::{create_engine_with_options, Context, EngineOptions, Mixer, Sampler, Timestamp};
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

#[allow(dead_code)]
pub struct AudioManager {
    context: Box<dyn Context>,
    realtime_process: Process,

    sample_converter: SampleConverter,
    conversion_rx: mpsc::Receiver<SampleConversionResult>,
    response_tx: broadcast::Sender<Response>,
    samples_being_converted: HashSet<ID>,
    playback_state: PlaybackState,
    progress: Progress,
    samplers: HashMap<ID, Sampler>,
    metronome: Metronome,
    project: Project,
    mixer: Mixer,
    sequencer: Sequencer,
}

impl AudioManager {
    pub fn new(response_tx: broadcast::Sender<Response>, preferences_dir: &Path) -> Self {
        let sample_rate = 44_100;
        let maximum_channel_count = 3;

        let (mut context, process) = create_engine_with_options(
            EngineOptions::default()
                .with_sample_rate(sample_rate)
                .with_maximum_channel_count(maximum_channel_count),
        );

        let metronome = Metronome::new(context.as_ref());

        context.start();

        let (conversion_tx, conversion_rx) = mpsc::channel(64);

        let realtime_process = Process::new(process, preferences_dir);

        let mixer = Mixer::unity(context.as_ref(), maximum_channel_count);

        metronome.output_node().connect_channels_to(&mixer.node, 0, 2, 1);

        mixer.node.connect_to_output();

        Self {
            context,
            realtime_process,
            sample_converter: SampleConverter::new(conversion_tx, sample_rate),
            conversion_rx,
            response_tx,
            samples_being_converted: HashSet::new(),
            playback_state: PlaybackState::default(),
            progress: Progress::default(),
            samplers: HashMap::new(),
            metronome,
            project: Project::empty(),
            mixer,
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
                _ = interval.tick() =>
                    self.interval_tick()
                ,
            }
        }
    }

    fn interval_tick(&mut self) {
        let current_time = self.context.current_time();
        self.context.process_notifications();
        self.sequencer.set_current_time(current_time);
        self.notify_playback_state();
        self.metronome.schedule(&current_time, &self.sequencer);
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

    pub fn progress(&self) -> &Progress {
        &self.progress
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

        let event_channel_capacity = 1024;
        let sampler = Sampler::new_with_event_capacity(self.context.as_ref(), audio_data, event_channel_capacity);
        sampler.node.connect_to(&self.mixer.node);

        self.samplers.insert(result.sample_id, sampler);
    }

    fn add_samples(&mut self, project: &Project, samples_cache: &SamplesCache) {
        for song in project.songs.iter() {
            let sample = match &song.sample {
                Some(sample) => sample,
                None => continue,
            };

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
            .filter(|(sample_id, _)| project.find_sample(sample_id).is_none())
            .map(|(sample_id, _)| sample_id)
            .copied()
            .collect();

        for sample_id in samples_to_remove {
            self.remove_sample(&sample_id);
        }
    }

    fn remove_sample(&mut self, sample_id: &ID) {
        if let Some(mut sampler) = self.samplers.remove(sample_id) {
            sampler.node.disconnect_from_node(&self.mixer.node);
            sampler.stop_now();
        }
    }

    pub fn on_project_updated(&mut self, project: &Project, samples_cache: &SamplesCache) {
        self.add_samples(project, samples_cache);
        self.project = project.clone();
        self.remove_samples(project);
    }

    fn lookahead_time(&self) -> Timestamp {
        self.context.current_time().incremented_by_seconds(0.001)
    }
}

impl Audio for AudioManager {
    fn play(&mut self) {
        self.sequencer
            .play(self.lookahead_time(), self.project.clone(), &mut self.samplers);
    }

    fn stop(&mut self) {
        self.sequencer.stop(&mut self.samplers);
    }

    fn enter_loop(&mut self) {
        self.sequencer.enter_loop(self.lookahead_time(), &mut self.samplers);
    }

    fn exit_loop(&mut self) {
        self.sequencer.exit_loop(self.lookahead_time(), &mut self.samplers);
    }

    fn queue(&mut self, song_id: &ID, section_id: &ID) {
        self.sequencer
            .queue(self.lookahead_time(), song_id, section_id, &mut self.samplers);
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
