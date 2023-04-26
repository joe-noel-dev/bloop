use rawdio::Timestamp;

use crate::model::{Project, Section, Song, ID};

use super::sequence::{Sequence, SequencePoint};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SequenceData {
    pub song_id: Option<ID>,
    pub section_id: Option<ID>,
    pub sample_id: Option<ID>,
    pub position_in_sample: Timestamp,
}

pub fn generate_sequence_for_song(
    start_time: Timestamp,
    project: &Project,
    song_id: &ID,
    from_section: &ID,
) -> Sequence<SequenceData> {
    let song = match project.song_with_id(song_id) {
        Some(song) => song,
        None => return Sequence::default(),
    };

    let points = song
        .section_ids
        .iter()
        .filter_map(|section_id| {
            let section = match project.section_with_id(section_id) {
                Some(section) => section,
                None => return None,
            };

            sequence_point_for_section_from_reference(project, section, from_section, start_time, song)
        })
        .collect();

    Sequence { points }
}

fn sequence_point_for_section_from_reference(
    project: &Project,
    section: &Section,
    reference_section_id: &ID,
    reference_time: Timestamp,
    song: &Song,
) -> Option<SequencePoint<SequenceData>> {
    start_time_of_section(project, song, &section.id, reference_section_id, reference_time)
        .map(|start_time| sequence_point_for_section(section, song, start_time))
}

fn start_time_of_section(
    project: &Project,
    song: &Song,
    section_id: &ID,
    reference_section_id: &ID,
    reference_time: Timestamp,
) -> Option<Timestamp> {
    if let Some(beat_position) = beat_position_of_section(project, song, section_id, reference_section_id) {
        return Some(reference_time.incremented_by_beats(beat_position, song.tempo.bpm));
    }

    None
}

fn beat_position_of_section(project: &Project, song: &Song, section_id: &ID, reference_section_id: &ID) -> Option<f64> {
    let mut position = None;

    for sec_id in song.section_ids.iter() {
        if *sec_id == *reference_section_id {
            position = Some(0.0);
        }

        if *sec_id == *section_id {
            return position;
        }

        let section_length = project
            .section_with_id(sec_id)
            .map(|section| section.beat_length)
            .unwrap_or(0.0);

        position = position.map(|current_position| current_position + section_length);
    }

    position
}

fn sequence_point_for_section(section: &Section, song: &Song, start_time: Timestamp) -> SequencePoint<SequenceData> {
    let section_duration = Timestamp::from_beats(section.beat_length, song.tempo.bpm);
    let start_position_in_sample = Timestamp::from_beats(section.start, song.tempo.bpm);

    SequencePoint {
        start_time,
        duration: section_duration,
        loop_enabled: section.looping,
        data: SequenceData {
            song_id: Some(song.id),
            section_id: Some(section.id),
            sample_id: song.sample_id,
            position_in_sample: start_position_in_sample,
        },
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
        let sequence = generate_sequence_for_song(start_time, &project, &song_id, &project.sections[0].id);

        assert_eq!(sequence.points.len(), 3);

        let expected_values = vec![
            SequencePoint {
                start_time,
                duration: Timestamp::from_beats(2.0, tempo),
                loop_enabled: false,
                data: SequenceData {
                    song_id: Some(song_id),
                    section_id: Some(project.sections[0].id),
                    sample_id: Some(sample_id),
                    position_in_sample: Timestamp::from_beats(1.0, tempo),
                },
            },
            SequencePoint {
                start_time: start_time.incremented_by_beats(2.0, tempo),
                duration: Timestamp::from_beats(3.0, tempo),
                loop_enabled: false,
                data: SequenceData {
                    song_id: Some(song_id),
                    section_id: Some(project.sections[1].id),
                    sample_id: Some(sample_id),
                    position_in_sample: Timestamp::from_beats(5.0, tempo),
                },
            },
            SequencePoint {
                start_time: start_time.incremented_by_beats(5.0, tempo),
                duration: Timestamp::from_beats(4.0, tempo),
                loop_enabled: false,
                data: SequenceData {
                    song_id: Some(song_id),
                    section_id: Some(project.sections[2].id),
                    sample_id: Some(sample_id),
                    position_in_sample: Timestamp::from_beats(9.0, tempo),
                },
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
        let sequence = generate_sequence_for_song(start_time, &project, &song_id, &project.sections[0].id);

        assert_eq!(sequence.points.len(), 3);

        let expected_values = vec![
            SequencePoint {
                start_time,
                duration: Timestamp::from_beats(5.0, tempo),
                loop_enabled: false,
                data: SequenceData {
                    song_id: Some(song_id),
                    section_id: Some(project.sections[0].id),
                    sample_id: Some(sample_id),
                    position_in_sample: Timestamp::from_beats(7.0, tempo),
                },
            },
            SequencePoint {
                start_time: start_time.incremented_by_beats(5.0, tempo),
                duration: Timestamp::from_beats(6.0, tempo),
                loop_enabled: true,
                data: SequenceData {
                    song_id: Some(song_id),
                    section_id: Some(project.sections[1].id),
                    sample_id: Some(sample_id),
                    position_in_sample: Timestamp::from_beats(9.0, tempo),
                },
            },
            SequencePoint {
                start_time: start_time.incremented_by_beats(11.0, tempo),
                duration: Timestamp::from_beats(3.0, tempo),
                loop_enabled: false,
                data: SequenceData {
                    song_id: Some(song_id),
                    section_id: Some(project.sections[2].id),
                    sample_id: Some(sample_id),
                    position_in_sample: Timestamp::from_beats(8.0, tempo),
                },
            },
        ];

        assert_eq!(sequence.points, expected_values);
    }
}
