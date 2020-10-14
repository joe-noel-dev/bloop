use super::{channels, names, samples, sections, songs};
use crate::model::{project, selections};
use uuid::Uuid;

pub fn generate_project(num_channels: u32, num_songs: u32, num_sections_per_song: u32) -> project::Project {
    let channels = channels::generate_channels(num_channels);
    let channel_ids = channels::get_channel_ids(&channels);
    let mut songs = songs::generate_songs(num_songs);
    let sections = sections::generate_sections(num_sections_per_song, &mut songs, &channel_ids);
    let sample_ids = sections::get_sample_ids(&sections);
    let samples = samples::generate_samples(&sample_ids);

    let selections = selections::Selections {
        song: match songs.len() {
            0 => None,
            _ => Some(songs[0].id),
        },
        section: match songs.len() > 0 && songs[0].section_ids.len() > 0 {
            false => None,
            true => Some(songs[0].section_ids[0]),
        },
        channel: match channels.len() {
            0 => None,
            _ => Some(channels[0].id),
        },
    };

    project::Project {
        id: Uuid::new_v4(),
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
