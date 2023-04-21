use super::{
    process::Process,
    sampler_converter::{SampleConversionResult, SampleConverter},
    sequence::{Sequence, SequencePoint},
};
use crate::{
    api::Response,
    model::{PlaybackState, PlayingState, Project, ID},
    samples::SamplesCache,
};
use futures::StreamExt;
use futures_channel::mpsc;
use rawdio::{create_engine, AudioBuffer, Context, Gain, Sampler};
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
    _response_tx: broadcast::Sender<Response>,
    samples_being_converted: HashSet<ID>,
    playback_state: PlaybackState,
    samplers: HashMap<ID, Sampler>,
    project: Project,
    output_gain: Gain,
}

impl AudioManager {
    pub fn new(response_tx: broadcast::Sender<Response>, preferences_dir: &Path) -> Self {
        let sample_rate = 44_100;
        let (mut context, process) = create_engine(sample_rate);

        context.start();

        let (conversion_tx, conversion_rx) = futures_channel::mpsc::channel(64);

        let channel_count = 2;
        let gain = Gain::new(context.as_ref(), channel_count);
        gain.node.connect_to_output();

        Self {
            context,
            _process: Process::new(process, preferences_dir),
            sample_converter: SampleConverter::new(conversion_tx),
            conversion_rx,
            _response_tx: response_tx,
            samples_being_converted: HashSet::new(),
            playback_state: PlaybackState::default(),
            samplers: HashMap::new(),
            project: Project::empty(),
            output_gain: gain,
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
                    self.notify_position();
                },
            }
        }
    }

    pub fn notify_position(&self) {}

    pub fn play_sequence(&mut self, sequence: Sequence) {
        for point in sequence.points {
            self.schedule_sequence_point(&point);
        }
    }

    fn schedule_sequence_point(&mut self, sequence_point: &SequencePoint) {
        if let Some(sample_id) = sequence_point.sample_id {
            if let Some(sampler) = self.samplers.get_mut(&sample_id) {
                sampler.start_from_position_at_time(sequence_point.start_time, sequence_point.position_in_sample);

                if let Some((loop_start, loop_end)) = sequence_point.loop_point {
                    sampler.enable_loop_at_time(sequence_point.start_time, loop_start, loop_end);
                }

                if let Some(end_time) = sequence_point.end_time {
                    sampler.stop_at_time(end_time);
                }
            }
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

        let sample_rate = audio_data.sample_rate();
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
        if let Some(selected_song_id) = self.project.selections.song {
            let sequence = Sequence::play_song(self.context.current_time(), &self.project, &selected_song_id);
            self.play_sequence(sequence);
        }
    }

    fn stop(&mut self) {
        self.play_sequence(Sequence::default());
    }

    fn enter_loop(&mut self) {
        // TODO: Loop
    }

    fn exit_loop(&mut self) {
        // TODO: Exit loop
    }

    fn queue(&mut self, song_id: &ID, section_id: &ID) {
        // TODO: Queue a song
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
