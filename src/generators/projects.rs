use super::{colours, names};
use crate::model::project;

pub fn generate_project(num_channels: usize, num_songs: usize, num_sections_per_song: usize) -> project::Project {
    let mut project = project::Project::new()
        .with_channels(num_channels)
        .with_songs(num_songs, num_sections_per_song);

    project.info.name = names::random_name();

    for song in &mut project.songs {
        song.name = names::random_name();
    }

    for section in &mut project.sections {
        section.name = String::from(names::random_section_name());
    }

    for channel in &mut project.channels {
        channel.name = String::from(names::random_channel_name());
        channel.colour = String::from(colours::random_colour());
    }

    project
}
