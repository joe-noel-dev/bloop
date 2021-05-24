use super::{
    buffer::{AudioBuffer, AudioBufferSlice, OwnedAudioBuffer},
    command::{Command, QueueCommand},
    notification::Notification,
    periodic_notification::PeriodicNotification,
    sampler,
    timeline::Timeline,
};
use crate::{
    model::{
        id::ID,
        playback_state::{PlaybackState, PlayingState},
        progress::Progress,
        project::Project,
        sample::Sample,
        section::Section,
        song::Song,
    },
    types::beats::Beats,
};
use futures_channel::mpsc::{Receiver, Sender};
use std::{collections::HashMap, convert::TryInto, mem, usize};

pub trait Engine {
    fn render<T>(&mut self, output: &mut T)
    where
        T: AudioBuffer;
}

const SAMPLE_RATE: u32 = 44100;
const MAX_SAMPLES: usize = 128;
const NOTIFICATION_RATE_HZ: f64 = 30.0;

pub struct AudioEngine {
    playback_state: PlaybackState,
    command_rx: Receiver<Command>,
    notification_tx: Sender<Notification>,
    sample_position: usize,
    project: Box<Project>,
    timeline: Timeline,
    last_section_start: usize,
    loop_count: i32,
    sample_rate: f64,
    audio_samples: HashMap<ID, Box<OwnedAudioBuffer>>,
    progress_notification: PeriodicNotification,
}

macro_rules! unwrap_or_return {
    ( $e:expr ) => {
        match $e {
            Some(x) => x,
            None => return,
        }
    };
}

impl AudioEngine {
    pub fn new(command_rx: Receiver<Command>, notification_tx: Sender<Notification>) -> Self {
        let mut audio_samples = HashMap::new();
        audio_samples.reserve(MAX_SAMPLES);

        let mut progress_notification = PeriodicNotification::default();
        progress_notification.reset(SAMPLE_RATE, NOTIFICATION_RATE_HZ);

        Self {
            playback_state: PlaybackState::default(),
            command_rx,
            notification_tx,
            sample_position: 0,
            project: Box::new(Project::new()),
            timeline: Timeline::new(SAMPLE_RATE),
            last_section_start: 0,
            loop_count: 0,
            sample_rate: 44100.0,
            audio_samples,
            progress_notification,
        }
    }

    fn send_notification(&mut self, notification: Notification) {
        self.notification_tx.try_send(notification).unwrap();
    }

    fn notify_playback_state(&mut self) {
        self.send_notification(Notification::Transport(self.playback_state.clone()));
    }

    fn current_song(&self) -> Option<&Song> {
        let song_id = self.playback_state.song_id?;
        self.project.song_with_id(&song_id)
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

            self.last_section_start = self.sample_position;
            self.loop_count = 0;

            self.playback_state.looping = match self.current_section() {
                Some(section) => section.looping,
                None => false,
            };
        }

        self.playback_state.playing = playing;
        self.timeline.set_tempo(tempo);
    }

    fn stop(&mut self) {
        self.playback_state = PlaybackState::default();
    }

    fn update_project(&mut self, project: Box<Project>) {
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

    fn next_sample_after(&self, beat_position: Beats, bpm: f64) -> usize {
        let beat_frequency = bpm / 60.0;
        let sample_position = (beat_position.to_num::<f64>() * self.sample_rate / beat_frequency).ceil();
        sample_position as usize
    }

    fn increment_section(&mut self) {
        self.loop_count += 1;

        if let Some(queued_song_id) = self.playback_state.queued_song_id {
            if let Some(queued_section_id) = self.playback_state.queued_section_id {
                self.playback_state.song_id = Some(queued_song_id);
                self.playback_state.section_id = Some(queued_section_id);
                self.playback_state.queued_section_id = None;
                self.playback_state.queued_song_id = None;

                if let Some(queued_section) = self.current_section() {
                    self.playback_state.looping = queued_section.looping;
                }

                return;
            }
        }

        if self.playback_state.looping {
            return;
        }

        let song = unwrap_or_return!(self.current_song());
        let current_section_id = unwrap_or_return!(self.playback_state.section_id);

        let mut iter = song.section_ids.iter();
        iter.find(|song_section_id| *song_section_id == &current_section_id);
        self.playback_state.section_id = iter.next().cloned();
        self.playback_state.looping = match self.current_section() {
            Some(section) => section.looping,
            None => false,
        };
    }

    fn update_tempo(&mut self, sample_position: usize) {
        let tempo = self.current_sample().map(|sample| sample.tempo.bpm);

        if let Some(tempo) = tempo {
            let beat_position = self.timeline.beat_position_for_samples(sample_position as i64);
            self.timeline.set_reference_point(beat_position, sample_position as i64);
            self.timeline.set_tempo(tempo);
        }
    }

    fn process_current_section<T>(&mut self, output: &mut T, start_position: usize, end_position: usize) -> usize
    where
        T: AudioBuffer,
    {
        let section = match self.current_section() {
            Some(section) => section,
            None => return 0,
        };

        let sample = match self.current_sample() {
            Some(sample) => sample,
            None => return 0,
        };

        let source = match self.audio_samples.get(&sample.id) {
            Some(buffer) => buffer,
            None => return 0,
        };

        let section_offset = if self.last_section_start < start_position {
            start_position - self.last_section_start
        } else {
            0
        };

        let section_start = self.next_sample_after(Beats::from_num(section.start), sample.tempo.bpm);
        let section_end =
            self.next_sample_after(Beats::from_num(section.start + section.beat_length), sample.tempo.bpm);

        let num_frames = sampler::render(output, source.as_ref(), section_start + section_offset, section_end);

        if num_frames < end_position - start_position {
            self.increment_section();
            self.update_tempo(start_position + num_frames);
            self.last_section_start = start_position + num_frames;
        }

        num_frames
    }

    fn process_project<T>(&mut self, output: &mut T) -> bool
    where
        T: AudioBuffer,
    {
        output.clear();

        if self.playback_state.playing != PlayingState::Playing {
            return false;
        }

        let mut output_offset: usize = 0;

        while output_offset < output.num_frames() {
            let remaining = output.num_frames() - output_offset;

            let mut output_slice = AudioBufferSlice::new(output, output_offset, remaining);

            let start_sample = self.sample_position + output_offset;
            let end_sample = start_sample + remaining;

            let mut num_frames_processed = self.process_current_section(&mut output_slice, start_sample, end_sample);

            if num_frames_processed == 0 {
                num_frames_processed = self.process_current_section(output, start_sample, end_sample);

                if num_frames_processed == 0 {
                    return false;
                }
            }

            output_offset += num_frames_processed;
        }

        true
    }

    fn add_sample(&mut self, sample_id: ID, audio_data: Box<OwnedAudioBuffer>) {
        self.audio_samples.insert(sample_id, audio_data);
    }

    fn remove_sample(&mut self, sample_id: ID) {
        if let Some(sample) = self.audio_samples.remove(&sample_id) {
            self.send_notification(Notification::ReturnSample(sample));
        }
    }

    fn process_command(&mut self, command: Command) {
        match command {
            Command::AddSample(add_sample_command) => {
                self.add_sample(add_sample_command.sample_id, add_sample_command.audio_data)
            }
            Command::RemoveSample(remove_sample_command) => self.remove_sample(remove_sample_command.sample_id),
            Command::Play => self.play(),
            Command::Stop => self.stop(),
            Command::UpdateProject(project) => self.update_project(project),
            Command::EnterLoop => self.enter_loop(),
            Command::ExitLoop => self.exit_loop(),
            Command::Queue(queue_command) => self.queue(queue_command),
        }
    }

    fn notify_progress(&mut self) {
        let section = unwrap_or_return!(self.current_section());
        let sample = unwrap_or_return!(self.current_sample());
        let audio_sample = unwrap_or_return!(self.audio_samples.get(&sample.id));

        if self.last_section_start > self.sample_position {
            return;
        }

        let beat_frequency = sample.tempo.bpm / 60.0;

        let section_beat_offset =
            (self.sample_position - self.last_section_start) as f64 * beat_frequency / self.sample_rate;
        let song_frame_offset = (section.start + section_beat_offset) * self.sample_rate / beat_frequency;

        let progress = Progress {
            song_progress: song_frame_offset / audio_sample.num_frames() as f64,
            section_progress: section_beat_offset / section.beat_length,
        };

        self.send_notification(Notification::Progress(progress));
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
        if self.playback_state.playing != PlayingState::Stopped && !continue_processing {
            self.stop();
        }

        if self.playback_state != previous_state {
            self.notify_playback_state();
        }

        self.sample_position += output.num_frames();

        if self.progress_notification.increment(output.num_frames() as i64) {
            self.notify_progress();
        }
    }
}
