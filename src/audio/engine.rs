use super::{
    buffer::AudioBuffer,
    command::{Command, QueueCommand},
    notification::Notification,
    timeline::Timeline,
};
use crate::{
    model::{
        playback_state::{PlaybackState, PlayingState},
        project::Project,
        sample::Sample,
        section::Section,
        song::Song,
    },
    types::beats::Beats,
};
use futures_channel::mpsc::{Receiver, Sender};
use std::{convert::TryInto, mem};

pub trait Engine {
    fn render<T>(&mut self, output: &mut T)
    where
        T: AudioBuffer;
}

pub struct AudioEngine {
    playback_state: PlaybackState,
    command_rx: Receiver<Command>,
    notification_tx: Sender<Notification>,
    sample_position: usize,
    project: Project,
    timeline: Timeline,
    last_section_start: Beats,
    loop_count: i32,
}

impl AudioEngine {
    pub fn new(command_rx: Receiver<Command>, notification_tx: Sender<Notification>) -> Self {
        Self {
            playback_state: PlaybackState::new(),
            command_rx,
            notification_tx,
            sample_position: 0,
            project: Project::new(),
            timeline: Timeline::new(44100),
            last_section_start: Beats::from_num(0.0),
            loop_count: 0,
        }
    }

    fn send_notification(&mut self, notification: Notification) {
        self.notification_tx.try_send(notification).unwrap();
    }

    fn notify_playback_state(&mut self) {
        self.send_notification(Notification::Transport(self.playback_state.clone()));
    }

    fn current_song(&self) -> Option<&Song> {
        match self.playback_state.song_id {
            Some(song_id) => self.project.song_with_id(&song_id),
            None => None,
        }
    }

    fn current_sample(&self) -> Option<&Sample> {
        let song = self.current_song()?;
        match song.sample_id {
            Some(sample_id) => self.project.sample_with_id(&sample_id),
            None => None,
        }
    }

    fn current_section(&self) -> Option<&Section> {
        match self.playback_state.section_id {
            Some(section_id) => self.project.section_with_id(&section_id),
            None => None,
        }
    }

    fn play(&mut self) {
        self.timeline
            .set_reference_point(Beats::from_num(0.0), self.sample_position.try_into().unwrap());

        self.playback_state.song_id = self.project.selections.song;
        self.playback_state.section_id = self.project.selections.section;

        let mut playing = PlayingState::Stopped;
        let mut tempo = 120.0;

        if let Some(sample) = self.current_sample() {
            playing = PlayingState::Playing;
            tempo = sample.tempo.bpm;

            self.last_section_start = Beats::from_num(0);
            self.loop_count = 0;

            if let Some(section) = self.current_section() {
                self.playback_state.looping = section.looping;
            } else {
                self.playback_state.looping = false;
            }
        }

        self.playback_state.playing = playing;
        self.timeline.set_tempo(tempo);
    }

    fn stop(&mut self) {
        self.playback_state = PlaybackState::new();
    }

    fn update_project(&mut self, project: Project) {
        let old_project = mem::replace(&mut self.project, project);
        self.send_notification(Notification::ReturnProject(old_project));
    }

    fn enter_loop(&mut self) {
        if self.playback_state.playing == PlayingState::Playing {
            self.playback_state.looping = true;
        }
    }

    fn exit_loop(&mut self) {
        if self.playback_state.playing == PlayingState::Playing {
            self.playback_state.looping = false;
        }
    }

    fn queue(&mut self, command: QueueCommand) {
        self.playback_state.queued_song_id = Some(command.song_id);
        self.playback_state.queued_section_id = Some(command.section_id);
    }

    fn process_project<T>(&self, output: &mut T) -> bool
    where
        T: AudioBuffer,
    {
        output.clear();
        true
    }

    fn process_command(&mut self, command: Command) {
        match command {
            Command::Play => self.play(),
            Command::Stop => self.stop(),
            Command::UpdateProject(project) => self.update_project(project),
            Command::EnterLoop => self.enter_loop(),
            Command::ExitLoop => self.exit_loop(),
            Command::Queue(queue_command) => self.queue(queue_command),
        }
    }
}

impl Engine for AudioEngine {
    fn render<T>(&mut self, output: &mut T)
    where
        T: AudioBuffer,
    {
        let previous_state = self.playback_state.clone();

        loop {
            match self.command_rx.try_next() {
                Ok(Some(command)) => self.process_command(command),
                Ok(None) => break,
                Err(_) => break,
            };
        }

        let continue_processing = self.process_project(output);
        if !continue_processing {
            self.stop();
        }

        if self.playback_state != previous_state {
            self.notify_playback_state();
        }

        self.sample_position += output.num_frames();
    }
}
