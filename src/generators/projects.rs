use super::names;
use crate::model::Project;

#[allow(dead_code)]
pub fn generate_project(num_songs: usize, num_sections_per_song: usize) -> Project {
    let mut project = Project::new().with_songs(num_songs, num_sections_per_song);

    project.info.name = names::random_name();

    for song in &mut project.songs {
        song.name = names::random_name();

        for section in &mut song.sections {
            section.name = String::from(names::random_section_name());
        }
    }

    project
}
