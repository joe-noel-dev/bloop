use crate::api::{request, response};
use crate::database::database;
use crate::generators;
use crate::model::{channel, project, selections};

type HandlerError = String;

fn unhandled_error() -> HandlerError {
    "Unsupported method".to_string()
}

fn error_response(message: &str) -> response::Response {
    response::Response::new().with_error(message)
}

pub fn handle_add(
    database: database::Database,
    request: request::AddRequest,
) -> Result<database::Database, HandlerError> {
    match request.entity {
        request::Entity::Channel => handle_add_channel(database),
        request::Entity::Section => handle_add_section(database, request),
        request::Entity::Song => handle_add_song(database),
        request::Entity::Project => handle_add_project(database),
        _ => Err(unhandled_error()),
    }
}

fn handle_add_channel(mut database: database::Database) -> Result<database::Database, HandlerError> {
    if database.project.channels.len() >= project::MAX_CHANNELS {
        return Err(unhandled_error());
    }

    let channel: channel::Channel = generators::channels::generate_channel();
    database.project.channels.push(channel.clone());
    Ok(database)
}

fn handle_add_section(
    mut database: database::Database,
    request: request::AddRequest,
) -> Result<database::Database, HandlerError> {
    let song_id = match request.parent_id {
        Some(id) => id,
        None => return Err("Missing parent ID".to_string()),
    };

    if let None = database.project.add_section_to_song(song_id) {
        return Err("Failed to add section".to_string());
    }

    Ok(database)
}

fn handle_add_song(mut database: database::Database) -> Result<database::Database, HandlerError> {
    let song = generators::songs::generate_song();
    database.project.songs.push(song.clone());

    database.project.selections = selections::Selections {
        song: Some(song.id),
        section: None,
        channel: None,
    };

    Ok(database)
}

fn handle_add_project(mut database: database::Database) -> Result<database::Database, HandlerError> {
    let project = generators::projects::generate_project(0, 0, 0);
    database.project = project.clone();
    Ok(database)
}

pub fn handle_select(
    database: database::Database,
    select_request: request::SelectRequest,
) -> Result<database::Database, HandlerError> {
    match select_request.entity {
        request::Entity::Song => handle_select_song(database, select_request),
        _ => Err(unhandled_error()),
    }
}

pub fn handle_select_song(
    mut database: database::Database,
    select_request: request::SelectRequest,
) -> Result<database::Database, HandlerError> {
    let song_id = select_request.id;

    if !database.project.contains_song(song_id) {
        return Err(format!("Song ID not found to select - {}", song_id));
    }

    database.project.selections = selections::Selections {
        song: Some(song_id),
        section: None,
        channel: None,
    };

    Ok(database)
}

pub fn handle_remove(
    database: database::Database,
    remove_request: request::RemoveRequest,
) -> Result<database::Database, HandlerError> {
    match remove_request.entity {
        request::Entity::Song => handle_remove_song(database, remove_request),
        _ => Err(unhandled_error()),
    }
}

pub fn handle_remove_song(
    mut database: database::Database,
    remove_request: request::RemoveRequest,
) -> Result<database::Database, HandlerError> {
    let song_id = remove_request.id;

    if !database.project.contains_song(song_id) {
        return Err(format!("Song ID not found to remove - {}", song_id));
    }

    database.project.remove_song(song_id);

    Ok(database)
}
