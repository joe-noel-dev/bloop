use rawdio::Timestamp;

use crate::model::{Project, Section, Song, ID};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SequencePoint {
    pub start_time: Timestamp,
    pub duration: Timestamp,
    pub song_id: Option<ID>,
    pub section_id: Option<ID>,
    pub sample_id: Option<ID>,
    pub position_in_sample: Timestamp,
    pub loop_enabled: bool,
}

impl SequencePoint {
    pub fn end_time(&self) -> Timestamp {
        self.start_time + self.duration
    }
}

#[derive(Clone, Default, PartialEq)]
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

        let points = project
            .sections
            .iter()
            .filter_map(|section| {
                sequence_point_for_section_from_reference(project, section, &reference_section_id, start_time, song)
            })
            .collect();

        Self { points }
    }

    pub fn point_at_time(&self, time: Timestamp) -> Option<SequencePoint> {
        self.points
            .iter()
            .find(|point| is_playing_at_time(point, time))
            .copied()
    }

    pub fn enable_loop_at_time(&self, time: Timestamp) -> Sequence {
        let mut sequence = self.clone();

        let mut loop_enabled = false;

        sequence.points.iter_mut().for_each(|point| {
            if !loop_enabled && is_playing_at_time(point, time) {
                point.loop_enabled = true;
                loop_enabled = true;
            } else {
                point.loop_enabled = false;
            }
        });

        sequence
    }
}

fn sequence_point_for_section_from_reference(
    project: &Project,
    section: &Section,
    reference_section_id: &ID,
    reference_time: Timestamp,
    song: &Song,
) -> Option<SequencePoint> {
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
    if let Some(beat_position) = beat_position_of_section(project, section_id, reference_section_id) {
        return Some(reference_time.incremented_by_beats(beat_position, song.tempo.bpm));
    }

    None
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

fn sequence_point_for_section(section: &Section, song: &Song, start_time: Timestamp) -> SequencePoint {
    let section_duration = Timestamp::from_beats(section.beat_length, song.tempo.bpm);
    let start_position_in_sample = Timestamp::from_beats(section.start, song.tempo.bpm);

    SequencePoint {
        start_time,
        duration: section_duration,
        song_id: Some(song.id),
        section_id: Some(section.id),
        sample_id: song.sample_id,
        position_in_sample: start_position_in_sample,
        loop_enabled: section.looping,
    }
}

fn is_playing_at_time(point: &SequencePoint, time: Timestamp) -> bool {
    if point.start_time > time {
        return false;
    }

    if !point.loop_enabled && point.end_time() < time {
        return false;
    }

    true
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
                duration: Timestamp::from_beats(2.0, tempo),
                song_id: Some(song_id),
                section_id: Some(project.sections[0].id),
                sample_id: Some(sample_id),
                position_in_sample: Timestamp::from_beats(1.0, tempo),
                loop_enabled: false,
            },
            SequencePoint {
                start_time: start_time.incremented_by_beats(2.0, tempo),
                duration: Timestamp::from_beats(3.0, tempo),
                song_id: Some(song_id),
                section_id: Some(project.sections[1].id),
                sample_id: Some(sample_id),
                position_in_sample: Timestamp::from_beats(5.0, tempo),
                loop_enabled: false,
            },
            SequencePoint {
                start_time: start_time.incremented_by_beats(5.0, tempo),
                duration: Timestamp::from_beats(4.0, tempo),
                song_id: Some(song_id),
                section_id: Some(project.sections[2].id),
                sample_id: Some(sample_id),
                position_in_sample: Timestamp::from_beats(9.0, tempo),
                loop_enabled: false,
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

        assert_eq!(sequence.points.len(), 3);

        let expected_values: Vec<SequencePoint> = vec![
            SequencePoint {
                start_time,
                duration: Timestamp::from_beats(5.0, tempo),
                song_id: Some(song_id),
                section_id: Some(project.sections[0].id),
                sample_id: Some(sample_id),
                position_in_sample: Timestamp::from_beats(7.0, tempo),
                loop_enabled: false,
            },
            SequencePoint {
                start_time: start_time.incremented_by_beats(5.0, tempo),
                duration: Timestamp::from_beats(6.0, tempo),
                song_id: Some(song_id),
                section_id: Some(project.sections[1].id),
                sample_id: Some(sample_id),
                position_in_sample: Timestamp::from_beats(9.0, tempo),
                loop_enabled: true,
            },
            SequencePoint {
                start_time: start_time.incremented_by_beats(11.0, tempo),
                duration: Timestamp::from_beats(3.0, tempo),
                song_id: Some(song_id),
                section_id: Some(project.sections[2].id),
                sample_id: Some(sample_id),
                position_in_sample: Timestamp::from_beats(8.0, tempo),
                loop_enabled: false,
            },
        ];

        assert_eq!(sequence.points, expected_values);
    }
}
