use rawdio::Timestamp;

use crate::model::{Project, Section, Song, ID};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SequencePoint {
    pub start_time: Timestamp,
    pub end_time: Option<Timestamp>,
    pub sample_id: Option<ID>,
    pub position_in_sample: Timestamp,
    pub loop_point: Option<(Timestamp, Timestamp)>,
}

#[derive(Default, PartialEq)]
pub struct Sequence {
    pub points: Vec<SequencePoint>,
}

impl Sequence {
    pub fn play_song(start_time: Timestamp, project: &Project, song_id: &ID) -> Self {
        let song = match project.song_with_id(song_id) {
            Some(song) => song,
            None => return Self::default(),
        };

        let reference_section_id = match project.sections.first() {
            Some(section) => section.id,
            None => return Self::default(),
        };

        Self {
            points: project
                .sections
                .iter()
                .filter_map(|section| {
                    Self::sequence_point_for_section(project, song_id, section, &reference_section_id, start_time, song)
                })
                .collect(),
        }
    }

    fn sequence_point_for_section(
        project: &Project,
        song_id: &ID,
        section: &Section,
        reference_section_id: &ID,
        start_time: Timestamp,
        song: &Song,
    ) -> Option<SequencePoint> {
        Self::start_time_of_section(project, song_id, &section.id, reference_section_id, start_time).map(|start_time| {
            let start_position_in_sample =
                Timestamp::from_seconds(Self::beat_position_to_time(section.start, song.tempo.bpm));

            let section_duration = Self::beat_position_to_time(section.start + section.beat_length, song.tempo.bpm);

            let end_position_in_sample = start_position_in_sample.incremented_by_seconds(section_duration);

            SequencePoint {
                start_time,
                end_time: if section.looping {
                    None
                } else {
                    Some(start_time.incremented_by_seconds(section_duration))
                },
                sample_id: song.sample_id,
                position_in_sample: start_position_in_sample,
                loop_point: if section.looping {
                    Some((start_position_in_sample, end_position_in_sample))
                } else {
                    None
                },
            }
        })
    }

    fn beat_position_of_section(project: &Project, section_id: &ID, reference_section_id: &ID) -> Option<f64> {
        let mut position = None;

        for section in project.sections.iter() {
            if section.id == *reference_section_id {
                position = Some(0.0);
            }

            if section.id == *section_id {
                return position;
            }

            if let Some(beat_position) = position {
                position = Some(beat_position + section.beat_length);
            }
        }

        position
    }

    fn beat_position_to_time(beat_position: f64, bpm: f64) -> f64 {
        let beat_frequency = bpm / 60.0;
        beat_position / beat_frequency
    }

    fn start_time_of_section(
        project: &Project,
        song_id: &ID,
        section_id: &ID,
        reference_section_id: &ID,
        reference_time: Timestamp,
    ) -> Option<Timestamp> {
        if let Some(beat_position) = Self::beat_position_of_section(project, section_id, reference_section_id) {
            if let Some(song) = project.song_with_id(song_id) {
                return Some(
                    reference_time.incremented_by_seconds(Self::beat_position_to_time(beat_position, song.tempo.bpm)),
                );
            }
        }

        None
    }
}
