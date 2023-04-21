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

        let mut remove = false;

        let points = project
            .sections
            .iter()
            .filter_map(|section| {
                Self::sequence_point_for_section(project, song_id, section, &reference_section_id, start_time, song)
            })
            .filter(|section| {
                if !remove && section.loop_point.is_some() {
                    remove = true;
                    return true;
                }

                !remove
            })
            .collect();

        Self { points }
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
            let section_duration = Timestamp::from_beats(section.beat_length, song.tempo.bpm);

            let start_position_in_sample = Timestamp::from_beats(section.start, song.tempo.bpm);
            let end_position_in_sample = start_position_in_sample.incremented(&section_duration);

            let end_time = if section.looping {
                None
            } else {
                Some(start_time.incremented(&section_duration))
            };

            SequencePoint {
                start_time,
                end_time,
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

    fn start_time_of_section(
        project: &Project,
        song_id: &ID,
        section_id: &ID,
        reference_section_id: &ID,
        reference_time: Timestamp,
    ) -> Option<Timestamp> {
        if let Some(beat_position) = Self::beat_position_of_section(project, section_id, reference_section_id) {
            if let Some(song) = project.song_with_id(song_id) {
                return Some(reference_time.incremented_by_beats(beat_position, song.tempo.bpm));
            }
        }

        None
    }
}

#[cfg(test)]
mod test {

    use uuid::Uuid;

    use super::*;

    #[test]
    fn song_with_sequential_sections() {
        let song_count = 1;
        let section_count = 3;

        let mut project = Project::new().with_songs(song_count, section_count);

        let sample_id = Uuid::new_v4();
        let tempo = 123.0;

        project.songs[0].tempo.bpm = tempo;
        project.songs[0].sample_id = Some(sample_id);

        {
            let section_1 = &mut project.sections[0];
            section_1.start = 1.0;
            section_1.beat_length = 2.0;
        }

        {
            let section_2 = &mut project.sections[1];
            section_2.start = 5.0;
            section_2.beat_length = 3.0;
        }

        {
            let section_3 = &mut project.sections[2];
            section_3.start = 9.0;
            section_3.beat_length = 4.0;
        }

        let start_time = Timestamp::from_seconds(8.0);

        let song_id = project.songs[0].id;
        let sequence = Sequence::play_song(start_time, &project, &song_id);

        assert_eq!(sequence.points.len(), 3);

        let expected_values: Vec<SequencePoint> = vec![
            SequencePoint {
                start_time,
                end_time: Some(start_time.incremented_by_beats(2.0, tempo)),
                sample_id: Some(sample_id),
                position_in_sample: Timestamp::from_beats(1.0, tempo),
                loop_point: None,
            },
            SequencePoint {
                start_time: start_time.incremented_by_beats(2.0, tempo),
                end_time: Some(start_time.incremented_by_beats(5.0, tempo)),
                sample_id: Some(sample_id),
                position_in_sample: Timestamp::from_beats(5.0, tempo),
                loop_point: None,
            },
            SequencePoint {
                start_time: start_time.incremented_by_beats(5.0, tempo),
                end_time: Some(start_time.incremented_by_beats(9.0, tempo)),
                sample_id: Some(sample_id),
                position_in_sample: Timestamp::from_beats(9.0, tempo),
                loop_point: None,
            },
        ];

        assert_eq!(sequence.points, expected_values);
    }

    #[test]
    fn song_with_looping_section() {
        let song_count = 1;
        let section_count = 3;

        let mut project = Project::new().with_songs(song_count, section_count);

        let sample_id = Uuid::new_v4();
        let tempo = 142.0;

        project.songs[0].tempo.bpm = tempo;
        project.songs[0].sample_id = Some(sample_id);

        {
            let section_1 = &mut project.sections[0];
            section_1.start = 7.0;
            section_1.beat_length = 5.0;
        }

        {
            let section_2 = &mut project.sections[1];
            section_2.start = 9.0;
            section_2.beat_length = 6.0;
            section_2.looping = true;
        }

        {
            let section_3 = &mut project.sections[2];
            section_3.start = 8.0;
            section_3.beat_length = 3.0;
        }

        let start_time = Timestamp::from_seconds(4.0);

        let song_id = project.songs[0].id;
        let sequence = Sequence::play_song(start_time, &project, &song_id);

        assert_eq!(sequence.points.len(), 2);

        let expected_values: Vec<SequencePoint> = vec![
            SequencePoint {
                start_time,
                end_time: Some(start_time.incremented_by_beats(5.0, tempo)),
                sample_id: Some(sample_id),
                position_in_sample: Timestamp::from_beats(7.0, tempo),
                loop_point: None,
            },
            SequencePoint {
                start_time: start_time.incremented_by_beats(5.0, tempo),
                end_time: None,
                sample_id: Some(sample_id),
                position_in_sample: Timestamp::from_beats(9.0, tempo),
                loop_point: Some((Timestamp::from_beats(9.0, tempo), Timestamp::from_beats(15.0, tempo))),
            },
        ];

        assert_eq!(sequence.points, expected_values);
    }
}
