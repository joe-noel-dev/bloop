use super::{
    command::{AddSampleCommand, Command, RemoveSampleCommand},
    notification::{Notification, SampleConversionResult},
    process::Process,
};
use crate::{
    api::response::Response,
    audio::{command::QueueCommand, convert::convert_sample},
    model::{
        id::ID,
        playback_state::{PlaybackState, PlayingState},
        project::Project,
    },
    samples::cache::SamplesCache,
};
use futures::StreamExt;
use futures_channel::mpsc;
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
    _process: Process,
    command_tx: mpsc::Sender<Command>,
    notification_rx: mpsc::Receiver<Notification>,
    notification_tx: mpsc::Sender<Notification>,
    response_tx: broadcast::Sender<Response>,
    samples_in_engine: HashSet<ID>,
    samples_being_converted: HashSet<ID>,
    playback_state: PlaybackState,
}

impl AudioManager {
    pub fn new(response_tx: broadcast::Sender<Response>, preferences_dir: &Path) -> Self {
        let (command_tx, command_rx) = mpsc::channel(128);
        let (notification_tx, notification_rx) = futures_channel::mpsc::channel(128);

        Self {
            _process: Process::new(command_rx, notification_tx.clone(), preferences_dir),
            command_tx,
            notification_tx,
            notification_rx,
            response_tx,
            samples_in_engine: HashSet::new(),
            samples_being_converted: HashSet::new(),
            playback_state: PlaybackState::default(),
        }
    }

    pub async fn run(&mut self) {
        loop {
            let notification = match self.notification_rx.next().await {
                Some(notification) => notification,
                None => return,
            };

            self.on_notification(notification);
        }
    }

    pub fn playback_state(&self) -> &PlaybackState {
        &self.playback_state
    }

    pub fn on_notification(&mut self, notification: Notification) {
        match notification {
            Notification::ReturnProject(_) => (/* Project is dropped here */),
            Notification::ReturnSample(_) => (/* Sample is dropped here */),
            Notification::Transport(playback_state) => {
                let _ = self
                    .response_tx
                    .send(Response::default().with_playback_state(&playback_state));

                self.playback_state = playback_state;
            }
            Notification::SampleConverted(result) => self.on_sample_converted(result),
            Notification::Progress(progress) => {
                let _ = self.response_tx.send(Response::default().with_progress(progress));
            }
        }
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

        self.send(Command::AddSample(AddSampleCommand {
            sample_id: result.sample_id,
            audio_data,
        }))
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
        if self.samples_in_engine.contains(&sample_id) {
            return;
        }

        if self.samples_being_converted.contains(&sample_id) {
            return;
        }

        self.samples_being_converted.insert(*sample_id);

        let mut notification_tx = self.notification_tx.clone();
        let sample_id = *sample_id;

        std::thread::spawn(move || {
            let result = convert_sample(&sample_path);
            notification_tx
                .try_send(Notification::SampleConverted(SampleConversionResult {
                    sample_id,
                    result,
                }))
                .unwrap();
        });
    }

    fn remove_samples(&mut self, project: &Project) {
        let samples_to_remove: HashSet<ID> = self
            .samples_in_engine
            .iter()
            .filter(|sample_id| project.samples.iter().find(|sample| sample.id == **sample_id).is_none())
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

        self.send(Command::RemoveSample(RemoveSampleCommand { sample_id: *sample_id }));
    }

    pub fn on_project_updated(&mut self, project: &Project, samples_cache: &SamplesCache) {
        self.add_samples(project, samples_cache);
        self.send(Command::UpdateProject(Box::new(project.clone())));
        self.remove_samples(project);
    }

    fn send(&mut self, command: Command) {
        self.command_tx.try_send(command).unwrap();
    }
}

impl Audio for AudioManager {
    fn play(&mut self) {
        self.send(Command::Play);
    }

    fn stop(&mut self) {
        self.send(Command::Stop);
    }

    fn enter_loop(&mut self) {
        self.send(Command::EnterLoop);
    }

    fn exit_loop(&mut self) {
        self.send(Command::ExitLoop);
    }

    fn queue(&mut self, song_id: &ID, section_id: &ID) {
        self.send(Command::Queue(QueueCommand {
            song_id: *song_id,
            section_id: *section_id,
        }));
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
