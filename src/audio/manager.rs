use super::{
    process::Process,
    sampler_converter::{SampleConversionResult, SampleConverter},
};
use crate::{
    api::Response,
    model::{PlaybackState, PlayingState, Project, ID},
    samples::SamplesCache,
};
use futures::StreamExt;
use futures_channel::mpsc;
use rawdio::{create_engine, Context};
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
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
    _context: Box<dyn Context>,
    _process: Process,

    sample_converter: SampleConverter,
    conversion_rx: mpsc::Receiver<SampleConversionResult>,
    _response_tx: broadcast::Sender<Response>,
    samples_in_engine: HashSet<ID>,
    samples_being_converted: HashSet<ID>,
    playback_state: PlaybackState,
}

impl AudioManager {
    pub fn new(response_tx: broadcast::Sender<Response>, preferences_dir: &Path) -> Self {
        let sample_rate = 48_000;
        let (context, process) = create_engine(sample_rate);
        let (conversion_tx, conversion_rx) = futures_channel::mpsc::channel(64);

        Self {
            _context: context,
            _process: Process::new(process, preferences_dir),
            sample_converter: SampleConverter::new(conversion_tx),
            conversion_rx,
            _response_tx: response_tx,
            samples_in_engine: HashSet::new(),
            samples_being_converted: HashSet::new(),
            playback_state: PlaybackState::default(),
        }
    }

    pub async fn run(&mut self) {
        loop {
            if let Some(conversion_result) = self.conversion_rx.next().await {
                self.on_sample_converted(conversion_result);
            };
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

        self.samples_in_engine.insert(result.sample_id);

        println!("Adding sample to the audio engine: {}", result.sample_id);

        // TODO: Add sample
    }

    fn add_samples(&mut self, project: &Project, samples_cache: &SamplesCache) {
        for sample in project.samples.iter() {
            if self.samples_in_engine.contains(&sample.id) {
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
        if self.samples_in_engine.contains(sample_id) {
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
            .samples_in_engine
            .iter()
            .filter(|sample_id| !project.samples.iter().any(|sample| &sample.id == *sample_id))
            .copied()
            .collect();

        for sample_id in samples_to_remove {
            self.remove_sample(&sample_id);
        }
    }

    fn remove_sample(&mut self, sample_id: &ID) {
        if !self.samples_in_engine.contains(sample_id) {
            return;
        }

        self.samples_in_engine.remove(sample_id);

        // TODO: Remove sample
    }

    pub fn on_project_updated(&mut self, project: &Project, samples_cache: &SamplesCache) {
        self.add_samples(project, samples_cache);
        // TODO: Update project
        self.remove_samples(project);
    }
}

impl Audio for AudioManager {
    fn play(&mut self) {
        // TODO: Start playback
    }

    fn stop(&mut self) {
        // TODO: Stop playback
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
