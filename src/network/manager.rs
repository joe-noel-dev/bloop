use super::handlers;
use super::server;
use crate::api::{request, response};
use crate::database::database;
use crate::generators;

use tokio::sync::{broadcast, mpsc};

pub async fn run() {
    let (request_tx, mut request_rx) = mpsc::channel(100);
    let (response_tx, _) = broadcast::channel(100);

    let project = generators::projects::generate_project(4, 3, 3);

    let mut database = database::Database { project };

    let server_response_tx = response_tx.clone();
    tokio::spawn(async move {
        server::run(request_tx.clone(), server_response_tx).await;
    });
    while let Some(message) = request_rx.recv().await {
        println!("Received message: {:?}", message);

        let new_db = match message {
            request::Request::Get(_) => Ok(database.clone()),
            request::Request::Add(add_request) => handlers::handle_add(database.clone(), add_request),
            request::Request::Select(select_request) => handlers::handle_select(database.clone(), select_request),
            request::Request::Remove(remove_request) => handlers::handle_remove(database.clone(), remove_request),
        };

        let response = match new_db {
            Ok(db) => {
                let response = response::Response::new().with_project(&db.project);
                database = db;
                response
            }
            Err(message) => response::Response::new().with_error(&message),
        };

        response_tx.send(response).unwrap();
    }
}
