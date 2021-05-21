use super::{
    buffer::{AudioBuffer, AudioBufferSlice, SampleLocation},
    command::{Command, QueueCommand},
    notification::Notification,
    timeline::Timeline,
};
use crate::{
    model::{
        id::ID,
        playback_state::{PlaybackState, PlayingState},
        project::Project,
        sample::Sample,
        section::Section,
        song::Song,
    },
    types::beats::Beats,
};
use futures_channel::mpsc::{Receiver, Sender};
use std::{cmp::min, collections::HashMap, convert::TryInto, mem, usize};

pub trait Engine {
    fn render<T>(&mut self, output: &mut T)
    where
        T: AudioBuffer;
}

const SAMPLE_RATE: u32 = 44100;
const MAX_SAMPLES: usize = 128;

pub struct AudioEngine {
    playback_state: PlaybackState,
    command_rx: Receiver<Command>,
    notification_tx: Sender<Notification>,
    sample_position: usize,
    project: Box<Project>,
    timeline: Timeline,
    last_section_start: Beats,
    loop_count: i32,
    sample_rate: f64,
    audio_samples: HashMap<ID, Box<dyn AudioBuffer + Send>>,
}

impl AudioEngine {
    pub fn new(command_rx: Receiver<Command>, notification_tx: Sender<Notification>) -> Self {
        let mut audio_samples = HashMap::new();
        audio_samples.reserve(MAX_SAMPLES);

        Self {
            playback_state: PlaybackState::new(),
            command_rx,
            notification_tx,
            sample_position: 0,
            project: Box::new(Project::new()),
            timeline: Timeline::new(SAMPLE_RATE),
            last_section_start: Beats::from_num(0.0),
            loop_count: 0,
            sample_rate: 44100.0,
            audio_samples,
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
            }
        }

        if self.playback_state.looping {
            return;
        }

        let song = match self.current_song() {
            Some(song) => song,
            None => return,
        };

        let current_section_id = match self.playback_state.section_id {
            Some(id) => id,
            None => return,
        };

        let mut iter = song.section_ids.iter();
        iter.find(|song_section_id| *song_section_id == &current_section_id);
        self.playback_state.section_id = iter.next().cloned();
        self.playback_state.looping = match self.current_section() {
            Some(section) => section.looping,
            None => false,
        };
    }

    fn update_tempo(&mut self, at_beat_position: Beats) {
        let tempo = self.current_sample().map(|sample| sample.tempo.bpm);

        if let Some(tempo) = tempo {
            let sample_position = self.timeline.sample_position_for_beats(at_beat_position).ceil() as i64;
            self.timeline.set_reference_point(at_beat_position, sample_position);
            self.timeline.set_tempo(tempo);
        }
    }

    fn process_current_section<T>(&mut self, output: &mut T, start_position: Beats, end_position: Beats) -> usize
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

        let source_buffer = match self.audio_samples.get(&sample.id) {
            Some(buffer) => buffer.as_ref(),
            None => return 0,
        };

        let start_position_in_section = start_position - self.last_section_start;
        let section_length = Beats::from_num(section.beat_length);
        let end_position_in_section = min(section_length, end_position - self.last_section_start);

        let start_frame_offset = self.next_sample_after(
            start_position_in_section + Beats::from_num(section.start),
            sample.tempo.bpm,
        );

        let end_frame_offset = self.next_sample_after(
            end_position_in_section + Beats::from_num(section.start),
            sample.tempo.bpm,
        );

        let num_frames = min(end_frame_offset - start_frame_offset, source_buffer.num_frames());
        let num_frames = min(num_frames, output.num_frames() - start_frame_offset);
        let num_channels = min(source_buffer.num_channels(), output.num_channels());

        let source_location = SampleLocation {
            channel: 0,
            frame: start_frame_offset,
        };

        let destination_location = SampleLocation { channel: 0, frame: 0 };

        output.add_from(
            source_buffer,
            &source_location,
            &destination_location,
            num_channels,
            num_frames,
        );

        if end_position_in_section == section_length {
            self.increment_section();
            self.update_tempo(end_position_in_section + self.last_section_start);
            self.last_section_start += section_length;
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

            let start_beat_position = self
                .timeline
                .beat_position_for_samples((self.sample_position + output_offset).try_into().unwrap());
            let end_beat_position = self
                .timeline
                .beat_position_for_samples((self.sample_position + output_offset + remaining).try_into().unwrap());

            let mut num_frames_processed =
                self.process_current_section(&mut output_slice, start_beat_position, end_beat_position);

            if num_frames_processed == 0 {
                num_frames_processed = self.process_current_section(output, start_beat_position, end_beat_position);

                if num_frames_processed == 0 {
                    return false;
                }
            }

            output_offset += num_frames_processed;
        }

        true
    }

    fn add_sample(&mut self, sample_id: ID, audio_data: Box<dyn AudioBuffer + Send>) {
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
