use crate::api::{request, response};
use crate::database::database;
use crate::generators;
use crate::model::channel;
use crate::model::project;
use uuid;

fn unhandled_error() -> response::Response {
    error_response("Unsupported method")
}

fn error_response(message: &str) -> response::Response {
    response::Response::new().with_error(message)
}

pub fn handle_get(
    database: database::Database,
    request: request::GetRequest,
) -> (database::Database, response::Response) {
    match request.entity {
        request::Entity::All => handle_get_all(database),
        request::Entity::Channel => handle_get_channel(database, request),
        _ => (database, unhandled_error()),
    }
}

fn handle_get_channel(
    database: database::Database,
    request: request::GetRequest,
) -> (database::Database, response::Response) {
    let channel_id = match request.id {
        Some(id) => id,
        None => return (database, error_response("Missing channel ID")),
    };

    let channel = match database
        .project
        .channels
        .iter()
        .find(|c| c.id.to_string() == channel_id)
    {
        Some(channel) => channel,
        None => {
            return (
                database,
                error_response(&format!("Couldn't find channel {}", channel_id)),
            )
        }
    };
    let response = response::Response::new().with_channels(&vec![channel.clone()]);
    (database, response)
}

fn handle_get_all(database: database::Database) -> (database::Database, response::Response) {
    let response = response::Response::new().with_project(&database.project);
    (database, response)
}

pub fn handle_add(
    database: database::Database,
    request: request::AddRequest,
) -> (database::Database, response::Response) {
    match request.entity {
        request::Entity::Channel => handle_add_channel(database),
        request::Entity::Section => handle_add_section(database, request),
        request::Entity::Song => handle_add_song(database),
        request::Entity::Project => handle_add_project(database),
        _ => (database, unhandled_error()),
    }
}

fn handle_add_channel(mut database: database::Database) -> (database::Database, response::Response) {
    if database.project.channels.len() >= project::MAX_CHANNELS {
        return (database, unhandled_error());
    }

    let channel: channel::Channel = generators::channels::generate_channel();
    database.project.channels.push(channel.clone());
    (database, response::Response::new().with_channels(&vec![channel]))
}

fn handle_add_section(
    mut database: database::Database,
    request: request::AddRequest,
) -> (database::Database, response::Response) {
    let song_id = match request.parent_id {
        Some(id) => id,
        None => return (database, error_response("Missing parent ID")),
    };

    let song = match database.project.songs.iter_mut().find(|s| s.id.to_string() == song_id) {
        Some(song) => song,
        None => {
            return (
                database,
                error_response(&format!("Couldn't find song with ID: {}", song_id)),
            )
        }
    };

    let channel_ids = database
        .project
        .channels
        .iter()
        .map(|c| c.id.clone())
        .collect::<Vec<uuid::Uuid>>();
    let section = generators::sections::generate_section(&channel_ids);
    database.project.sections.push(section.clone());
    song.section_ids.push(section.id);
    let response = response::Response::new()
        .with_sections(&vec![section])
        .with_songs(&vec![song.clone()]);
    (database, response)
}

fn handle_add_song(mut database: database::Database) -> (database::Database, response::Response) {
    let song = generators::songs::generate_song();
    database.project.songs.push(song.clone());
    (database, response::Response::new().with_songs(&vec![song]))
}

fn handle_add_project(mut database: database::Database) -> (database::Database, response::Response) {
    let project = generators::projects::generate_project(0, 0, 0);
    database.project = project.clone();
    (database, response::Response::new().with_project(&project))
}
