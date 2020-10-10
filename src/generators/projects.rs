use super::{channels, names, samples, sections, songs};
use crate::model::{project, selections, state};
use uuid::Uuid;

pub fn generate_project(num_channels: u32, num_songs: u32, num_sections_per_song: u32) -> project::Project {
    let channels = channels::generate_channels(num_channels);
    let channel_ids = channels::get_channel_ids(&channels);
    let mut songs = songs::generate_songs(num_songs);
    let sections = sections::generate_sections(num_sections_per_song, &mut songs, &channel_ids);
    let sample_ids = sections::get_sample_ids(&sections);
    let samples = samples::generate_samples(&sample_ids);

    let selections = selections::Selections {
        song: if songs.len() > 0 { Some(songs[0].id) } else { None },
        section: if songs.len() > 0 && songs[0].section_ids.len() > 0 {
            Some(songs[0].section_ids[0])
        } else {
            None
        },
        channel: if channels.len() > 0 { Some(channels[0].id) } else { None },
    };

    project::Project {
        id: Uuid::new_v4(),
        state: state::State::Active,
        info: project::ProjectInfo {
            name: names::random_name(),
            version: "0".to_string(),
        },
        songs,
        sections,
        channels,
        samples,
        selections,
    }
}
