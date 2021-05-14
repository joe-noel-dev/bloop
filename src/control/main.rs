use super::handlers;
use crate::api;
use crate::generators;
use crate::model;

use tokio::sync::{broadcast, mpsc};

pub async fn run(
    request_rx: &mut mpsc::Receiver<api::request::Request>,
    response_tx: broadcast::Sender<api::response::Response>,
) {
    let mut project = generators::projects::generate_project(4, 3, 3);

    while let Some(request) = request_rx.recv().await {
        let response = match handle_request(request, project.clone()) {
            Ok(new_project) => {
                let response = api::response::Response::new().with_project(&new_project);
                project = new_project;
                response
            }
            Err(message) => api::response::Response::new().with_error(&message),
        };

        response_tx.send(response).unwrap();
    }
}

fn handle_request(
    request: api::request::Request,
    project: model::project::Project,
) -> Result<model::project::Project, String> {
    println!("Received message: {:?}", request);

    return match request {
        api::request::Request::Get(_) => Ok(project),
        api::request::Request::Add(add_request) => handlers::handle_add(project, add_request),
        api::request::Request::Select(select_request) => handlers::handle_select(project, select_request),
        api::request::Request::Remove(remove_request) => handlers::handle_remove(project, remove_request),
        api::request::Request::Update(update_request) => handlers::handle_update(project, update_request),
    };
}
