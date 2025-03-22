use super::names;
use crate::model::Project;

pub fn generate_project(num_songs: usize, num_sections_per_song: usize) -> Project {
    let mut project = Project::empty();
    project = project.with_songs(num_songs, num_sections_per_song);

    let info = project.info.as_mut().expect("Missing project info");
    info.name = names::random_name();

    for song in &mut project.songs {
        song.name = names::random_name();

        for section in &mut song.sections {
            section.name = String::from(names::random_section_name());
        }
    }

    project
}
