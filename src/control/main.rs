use super::handlers;
use crate::api::request;
use crate::api::response;
use crate::database::database;
use crate::generators;

use tokio::sync::{broadcast, mpsc};

pub async fn run(
    request_rx: &mut mpsc::Receiver<request::Request>,
    response_tx: broadcast::Sender<response::Response>,
) {
    let mut database = database::Database {
        project: generators::projects::generate_project(4, 3, 3),
    };

    while let Some(request) = request_rx.recv().await {
        let response = match handle_request(request, database.clone()) {
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

fn handle_request(request: request::Request, database: database::Database) -> Result<database::Database, String> {
    println!("Received message: {:?}", request);

    return match request {
        request::Request::Get(_) => Ok(database.clone()),
        request::Request::Add(add_request) => handlers::handle_add(database, add_request),
        request::Request::Select(select_request) => handlers::handle_select(database, select_request),
        request::Request::Remove(remove_request) => handlers::handle_remove(database, remove_request),
        request::Request::Update(update_request) => handlers::handle_update(database, update_request),
    };
}
