use std::collections::HashMap;

use rawdio::{Sampler, Timestamp};

use super::{
    sequence::{Sequence, SequencePoint},
    sequence_generator::{generate_sequence_for_song, SequenceData},
};
use crate::model::{PlaybackState, PlayingState, Progress, Project, ID};

#[derive(Default)]
pub struct Sequencer {
    project: Project,
    sequence: Sequence<SequenceData>,
    queued_song: Option<ID>,
    queued_section: Option<ID>,
    current_time: Timestamp,
}

impl Sequencer {
    pub fn set_current_time(&mut self, current_time: Timestamp) {
        self.current_time = current_time;

        if let Some(current_point) = self.sequence.point_at_time(current_time) {
            if current_point.data.song_id == self.queued_song {
                self.queued_song = None;
            }

            if current_point.data.section_id == self.queued_section {
                self.queued_section = None;
            }
        }
    }

    pub fn sequence_point_at_time(&self, time: Timestamp) -> Option<SequencePoint<SequenceData>> {
        self.sequence.point_at_time(time)
    }

    pub fn get_playback_state(&mut self) -> PlaybackState {
        let current_point = self.sequence.point_at_time(self.current_time);

        match current_point {
            Some(current_point) => {
                if current_point.data.song_id == self.queued_song {
                    self.queued_song = None;
                }

                if current_point.data.section_id == self.queued_section {
                    self.queued_section = None;
                }

                PlaybackState {
                    playing: PlayingState::Playing,
                    song_id: current_point.data.song_id,
                    section_id: current_point.data.section_id,
                    queued_song_id: self.queued_song,
                    queued_section_id: self.queued_section,
                    looping: current_point.loop_enabled,
                }
            }
            None => PlaybackState::default(),
        }
    }

    pub fn get_progress(&self) -> Progress {
        let current_point = self.sequence.point_at_time(self.current_time);

        match current_point {
            Some(current_point) => self.progress_through_sequence(&self.current_time, &current_point),
            None => Progress::default(),
        }
    }

    pub fn stop(&mut self, samplers: &mut HashMap<ID, Sampler>) {
        self.queued_section = None;
        self.queued_song = None;

        self.set_sequence(Sequence::default(), samplers);
    }

    pub fn play(&mut self, start_time: Timestamp, project: Project, samplers: &mut HashMap<ID, Sampler>) {
        self.queued_section = None;
        self.queued_song = None;

        self.project = project;

        let selected_song_id = match self.project.selections.song {
            Some(id) => id,
            None => return,
        };

        let selected_section_id = match self.project.selections.section {
            Some(id) => id,
            None => return,
        };

        let sequence = generate_sequence_for_song(start_time, &self.project, &selected_song_id, &selected_section_id);

        self.set_sequence(sequence, samplers);
    }

    pub fn enter_loop(&mut self, at_time: Timestamp, samplers: &mut HashMap<ID, Sampler>) {
        let new_sequence = self.sequence.enable_loop_at_time(at_time);
        self.set_sequence(new_sequence, samplers);
    }

    pub fn exit_loop(&mut self, at_time: Timestamp, samplers: &mut HashMap<ID, Sampler>) {
        let new_sequence = self.sequence.cancel_loop_at_time(at_time);
        self.set_sequence(new_sequence, samplers);
    }

    pub fn queue(&mut self, after_time: Timestamp, song_id: &ID, section_id: &ID, samplers: &mut HashMap<ID, Sampler>) {
        let transition_time = self.sequence.next_transition(after_time);
        let existing_sequence = self.sequence.truncate_to_time(transition_time);
        let new_sequence = generate_sequence_for_song(transition_time, &self.project, song_id, section_id);
        let sequence = existing_sequence.append(new_sequence);

        self.set_sequence(sequence, samplers);

        self.queued_section = Some(*section_id);
        self.queued_song = Some(*song_id);
    }

    fn progress_through_sequence(&self, time: &Timestamp, point: &SequencePoint<SequenceData>) -> Progress {
        let section = point
            .data
            .section_id
            .and_then(|section_id| self.project.section_with_id(&section_id));

        let song = point
            .data
            .song_id
            .and_then(|song_id| self.project.song_with_id(&song_id));

        let sample = point
            .data
            .sample_id
            .and_then(|sample_id| self.project.find_sample(&sample_id));

        match (section, song, sample) {
            (Some(section), Some(song), Some(sample)) => {
                let seconds_into_section =
                    (time.as_seconds() - point.start_time.as_seconds()) % point.duration.as_seconds();
                let beats_into_section = Timestamp::from_seconds(seconds_into_section).as_beats(song.tempo.get_bpm());

                let position_in_sample = seconds_into_section + point.data.position_in_sample.as_seconds();
                let sample_duration =
                    Timestamp::from_samples(sample.sample_count as f64, sample.sample_rate as usize).as_seconds();

                let section_length = song.section_length(&section.id);

                Progress {
                    song_progress: if sample_duration > 0.0 {
                        position_in_sample / sample_duration
                    } else {
                        0.0
                    },
                    section_progress: if section_length > 0.0 {
                        beats_into_section / section_length
                    } else {
                        0.0
                    },

                    section_beat: beats_into_section,
                }
            }
            _ => Progress::default(),
        }
    }

    fn set_sequence(&mut self, sequence: Sequence<SequenceData>, samplers: &mut HashMap<ID, Sampler>) {
        for (_, sampler) in samplers.iter_mut() {
            sampler.cancel_all();
        }

        for point in sequence.points.iter() {
            if point.end_time() < self.current_time {
                continue;
            }

            self.schedule_sequence_point(point, samplers);

            if point.loop_enabled {
                break;
            }
        }

        self.sequence = sequence;
    }

    fn schedule_sequence_point(
        &mut self,
        sequence_point: &SequencePoint<SequenceData>,
        samplers: &mut HashMap<ID, Sampler>,
    ) {
        if let Some(sample_id) = sequence_point.data.sample_id {
            if let Some(sampler) = samplers.get_mut(&sample_id) {
                sampler.start_from_position_at_time(sequence_point.start_time, sequence_point.data.position_in_sample);

                if sequence_point.loop_enabled {
                    let loop_start = sequence_point.data.position_in_sample;
                    let loop_end = sequence_point.data.position_in_sample + sequence_point.duration;

                    sampler.enable_loop_at_time(sequence_point.start_time, loop_start, loop_end);
                } else {
                    // Adjust the stop time to ensure that stop events are processed before start events
                    sampler.stop_at_time(sequence_point.end_time() - Timestamp::from_seconds(0.001));
                }
            }
        }
    }
}
