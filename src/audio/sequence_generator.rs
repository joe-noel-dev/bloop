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
        .sections
        .iter()
        .filter_map(|section| sequence_point_for_section_from_reference(section, from_section, start_time, song))
        .collect();

    Sequence { points }
}

fn sequence_point_for_section_from_reference(
    section: &Section,
    reference_section_id: &ID,
    reference_time: Timestamp,
    song: &Song,
) -> Option<SequencePoint<SequenceData>> {
    start_time_of_section(song, &section.id, reference_section_id, reference_time)
        .map(|start_time| sequence_point_for_section(section, song, start_time))
}

fn start_time_of_section(
    song: &Song,
    section_id: &ID,
    reference_section_id: &ID,
    reference_time: Timestamp,
) -> Option<Timestamp> {
    if let Some(beat_position) = beat_position_of_section(song, section_id, reference_section_id) {
        return Some(reference_time.incremented_by_beats(beat_position, song.tempo.get_bpm()));
    }

    None
}

fn beat_position_of_section(song: &Song, section_id: &ID, reference_section_id: &ID) -> Option<f64> {
    let reference_section = song.find_section(reference_section_id);
    let section = song.find_section(section_id);

    if let (Some(reference_section), Some(section)) = (reference_section, section) {
        if section.start >= reference_section.start {
            return Some(section.start - reference_section.start);
        }
    }

    None
}

fn sequence_point_for_section(section: &Section, song: &Song, start_time: Timestamp) -> SequencePoint<SequenceData> {
    let section_length = song.section_length(&section.id);
    let section_duration = Timestamp::from_beats(section_length, song.tempo.get_bpm());
    let start_position_in_sample = Timestamp::from_beats(section.start, song.tempo.get_bpm());

    SequencePoint {
        start_time,
        duration: section_duration,
        loop_enabled: section.looping,
        data: SequenceData {
            song_id: Some(song.id),
            section_id: Some(section.id),
            sample_id: song.sample.as_ref().map(|sample| sample.id),
            position_in_sample: start_position_in_sample,
        },
    }
}

#[cfg(test)]
mod test {

    use crate::model::{Sample, Tempo};

    use super::*;

    #[test]
    fn song_with_sequential_sections() {
        let song_count = 1;
        let section_count = 3;

        let mut project = Project::new().with_songs(song_count, section_count);

        let tempo = 123.0;
        let sample_beat_length = 15.0;
        let sample = Sample::new().with_beat_length(Tempo::new(tempo), sample_beat_length, 48_000);

        {
            let song = &mut project.songs[0];
            song.tempo = Tempo::new(tempo);
            song.sample = Some(sample.clone());

            {
                let section_1 = &mut song.sections[0];
                section_1.start = 1.0;
            }

            {
                let section_2 = &mut song.sections[1];
                section_2.start = 5.0;
            }

            {
                let section_3 = &mut song.sections[2];
                section_3.start = 10.0;
            }
        }

        let song = &project.songs[0];
        let start_time = Timestamp::from_seconds(8.0);

        let song_id = song.id;
        let sequence = generate_sequence_for_song(start_time, &project, &song_id, &song.sections[0].id);

        assert_eq!(sequence.points.len(), 3);

        let expected_values = vec![
            SequencePoint {
                start_time,
                duration: Timestamp::from_beats(4.0, tempo),
                loop_enabled: false,
                data: SequenceData {
                    song_id: Some(song_id),
                    section_id: Some(song.sections[0].id),
                    sample_id: Some(sample.id),
                    position_in_sample: Timestamp::from_beats(1.0, tempo),
                },
            },
            SequencePoint {
                start_time: start_time.incremented_by_beats(4.0, tempo),
                duration: Timestamp::from_beats(5.0, tempo),
                loop_enabled: false,
                data: SequenceData {
                    song_id: Some(song_id),
                    section_id: Some(song.sections[1].id),
                    sample_id: Some(sample.id),
                    position_in_sample: Timestamp::from_beats(5.0, tempo),
                },
            },
            SequencePoint {
                start_time: start_time.incremented_by_beats(9.0, tempo),
                duration: Timestamp::from_beats(sample.beat_length() - 10.0, tempo),
                loop_enabled: false,
                data: SequenceData {
                    song_id: Some(song_id),
                    section_id: Some(song.sections[2].id),
                    sample_id: Some(sample.id),
                    position_in_sample: Timestamp::from_beats(10.0, tempo),
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

        let tempo = 142.0;
        let sample_beat_length = 20.0;
        let sample = Sample::new().with_beat_length(Tempo::new(tempo), sample_beat_length, 48_000);

        {
            let song = &mut project.songs[0];
            song.tempo = Tempo::new(tempo);
            song.sample = Some(sample.clone());

            {
                let section_1 = &mut song.sections[0];
                section_1.start = 7.0;
            }

            {
                let section_2 = &mut song.sections[1];
                section_2.start = 9.0;
                section_2.looping = true;
            }

            {
                let section_3 = &mut song.sections[2];
                section_3.start = 15.0;
            }
        }

        let start_time = Timestamp::from_seconds(4.0);

        let song = &project.songs[0];
        let sequence = generate_sequence_for_song(start_time, &project, &song.id, &song.sections[0].id);

        assert_eq!(sequence.points.len(), 3);

        let expected_values = vec![
            SequencePoint {
                start_time,
                duration: Timestamp::from_beats(2.0, tempo),
                loop_enabled: false,
                data: SequenceData {
                    song_id: Some(song.id),
                    section_id: Some(song.sections[0].id),
                    sample_id: Some(sample.id),
                    position_in_sample: Timestamp::from_beats(7.0, tempo),
                },
            },
            SequencePoint {
                start_time: start_time.incremented_by_beats(2.0, tempo),
                duration: Timestamp::from_beats(6.0, tempo),
                loop_enabled: true,
                data: SequenceData {
                    song_id: Some(song.id),
                    section_id: Some(song.sections[1].id),
                    sample_id: Some(sample.id),
                    position_in_sample: Timestamp::from_beats(9.0, tempo),
                },
            },
            SequencePoint {
                start_time: start_time.incremented_by_beats(8.0, tempo),
                duration: Timestamp::from_beats(sample.beat_length() - 15.0, tempo),
                loop_enabled: false,
                data: SequenceData {
                    song_id: Some(song.id),
                    section_id: Some(song.sections[2].id),
                    sample_id: Some(sample.id),
                    position_in_sample: Timestamp::from_beats(15.0, tempo),
                },
            },
        ];

        assert_eq!(sequence.points, expected_values);
    }
}
