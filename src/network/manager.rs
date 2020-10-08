use super::handlers;
use super::server;
use crate::api::request;
use crate::database::database::Database;
use crate::generators;

use tokio::sync::{broadcast, mpsc};

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
        let (new_db, response) = match message {
            request::Request::Get(get_request) => handlers::handle_get(database, get_request),
            request::Request::Add(add_request) => handlers::handle_add(database, add_request),
        };

        database = new_db;
        response_tx.send(response).unwrap();
    }
}
