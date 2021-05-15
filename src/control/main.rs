use super::project_handlers;
use crate::api::{request, response};
use crate::generators::projects;
use crate::model::project;
use tokio::sync::{broadcast, mpsc};

struct AppState {
    project: project::Project,
}

pub async fn run(
    request_rx: &mut mpsc::Receiver<request::Request>,
    response_tx: broadcast::Sender<response::Response>,
) {
    let mut app_state = AppState {
        project: projects::generate_project(4, 3, 3),
    };

    while let Some(request) = request_rx.recv().await {
        let response = match project_handlers::handle_request(request, app_state.project.clone()) {
            Ok(new_project) => {
                let response = response::Response::new().with_project(&new_project);
                app_state.project = new_project;
                response
            }
            Err(message) => response::Response::new().with_error(&message),
        };

        match response_tx.send(response) {
            Ok(_) => continue,
            Err(error) => {
                println!("Error sending response: {}", error);
            }
        }
    }
}
