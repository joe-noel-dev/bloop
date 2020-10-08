use super::server;
use crate::api::request;
use crate::api::response;
use crate::database::database::Database;
use crate::generators;
use crate::model::channel;
use crate::model::project;
use tokio::sync::{broadcast, mpsc};
use uuid;

pub async fn run() {
    let (request_tx, mut request_rx) = mpsc::channel(100);
    let (response_tx, _) = broadcast::channel(100);

    let mut database = Database {
        project: generators::projects::generate_project(4, 3, 3),
    };

    let server_response_tx = response_tx.clone();
    tokio::spawn(async move {
        server::run(request_tx.clone(), server_response_tx).await;
    });

    while let Some(message) = request_rx.recv().await {
        let response = match message {
            request::Request::Get(get_request) => {
                handle_get(&database, get_request)
            }
            request::Request::Add(add_request) => {
                handle_add(&mut database, add_request)
            }
        };
        response_tx.send(response).unwrap();
    }
}

fn unhandled_error() -> response::Response {
    let mut response = response::Response::new();
    response.error = Some("Unsupported method".to_string());
    response
}

fn handle_get(
    database: &Database,
    request: request::GetRequest,
) -> response::Response {
    match request.entity {
        request::Entity::All => handle_get_all(&database),
        request::Entity::Channel => handle_get_channel(&database, request),
        _ => unhandled_error(),
    }
}

fn handle_get_channel(
    _database: &Database,
    _request: request::GetRequest,
) -> response::Response {
    unhandled_error()
}

fn handle_get_all(database: &Database) -> response::Response {
    let mut response = response::Response::new();
    response.project = Some(database.project.clone());
    response
}

fn handle_add(
    database: &mut Database,
    request: request::AddRequest,
) -> response::Response {
    match request.entity {
        request::Entity::Channel => handle_add_channel(database),
        request::Entity::Section => handle_add_section(database, request),
        request::Entity::Song => handle_add_song(database),
        _ => unhandled_error(),
    }
}

fn handle_add_channel(database: &mut Database) -> response::Response {
    if database.project.channels.len() >= project::MAX_CHANNELS {
        return unhandled_error();
    }

    let channel: channel::Channel = generators::channels::generate_channel();
    database.project.channels.push(channel.clone());
    let mut response = response::Response::new();
    response.channels = Some(vec![channel]);
    response
}

fn handle_add_section(
    database: &mut Database,
    request: request::AddRequest,
) -> response::Response {
    if request.parent_id.is_none() {
        return response::Response::new().with_error("Missing parent ID");
    }

    let song_id = request.parent_id.unwrap();

    if let Some(song) = database
        .project
        .songs
        .iter_mut()
        .find(|s| s.id.to_string() == song_id)
    {
        let channel_ids = database
            .project
            .channels
            .iter()
            .map(|c| c.id.clone())
            .collect::<Vec<uuid::Uuid>>();
        let section = generators::sections::generate_section(&channel_ids);
        database.project.sections.push(section.clone());
        song.section_ids.push(section.id);
        let mut response = response::Response::new();
        response.sections = Some(vec![section]);
        response.songs = Some(vec![song.clone()]);
        return response;
    }
    unhandled_error()
}

fn handle_add_song(database: &mut Database) -> response::Response {
    let song = generators::songs::generate_song();
    database.project.songs.push(song.clone());

    let mut response = response::Response::new();
    response.songs = Some(vec![song]);
    response
}
