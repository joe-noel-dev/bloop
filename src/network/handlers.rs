use crate::api::{request, response};
use crate::database::database;
use crate::generators;
use crate::model::channel;
use crate::model::project;
use uuid;

fn unhandled_error() -> response::Response {
    response::Response::new().with_error("Unsupported method")
}

pub fn handle_get(
    database: &database::Database,
    request: request::GetRequest,
) -> response::Response {
    match request.entity {
        request::Entity::All => handle_get_all(&database),
        request::Entity::Channel => handle_get_channel(&database, request),
        _ => unhandled_error(),
    }
}

fn handle_get_channel(
    _database: &database::Database,
    _request: request::GetRequest,
) -> response::Response {
    unhandled_error()
}

fn handle_get_all(database: &database::Database) -> response::Response {
    response::Response::new().with_project(&database.project)
}

pub fn handle_add(
    database: &mut database::Database,
    request: request::AddRequest,
) -> response::Response {
    match request.entity {
        request::Entity::Channel => handle_add_channel(database),
        request::Entity::Section => handle_add_section(database, request),
        request::Entity::Song => handle_add_song(database),
        _ => unhandled_error(),
    }
}

fn handle_add_channel(database: &mut database::Database) -> response::Response {
    if database.project.channels.len() >= project::MAX_CHANNELS {
        return unhandled_error();
    }

    let channel: channel::Channel = generators::channels::generate_channel();
    database.project.channels.push(channel.clone());
    response::Response::new().with_channels(&vec![channel])
}

fn handle_add_section(
    database: &mut database::Database,
    request: request::AddRequest,
) -> response::Response {
    if request.parent_id.is_none() {
        return response::Response::new().with_error("Missing parent ID");
    }

    let song_id = request.parent_id.unwrap();

    let song = database
        .project
        .songs
        .iter_mut()
        .find(|s| s.id.to_string() == song_id);

    if song.is_none() {
        return response::Response::new().with_error("Couldn't find song");
    }

    let song = song.unwrap();

    let channel_ids = database
        .project
        .channels
        .iter()
        .map(|c| c.id.clone())
        .collect::<Vec<uuid::Uuid>>();
    let section = generators::sections::generate_section(&channel_ids);
    database.project.sections.push(section.clone());
    song.section_ids.push(section.id);
    response::Response::new()
        .with_sections(&vec![section])
        .with_songs(&vec![song.clone()])
}

fn handle_add_song(database: &mut database::Database) -> response::Response {
    let song = generators::songs::generate_song();
    database.project.songs.push(song.clone());
    response::Response::new().with_songs(&vec![song])
}
