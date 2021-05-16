use std::path::PathBuf;

use super::{project_handlers, project_store, project_store_handlers};
use crate::api::{request, response};
use crate::generators::projects;
use crate::model::{project, proxy};
use tokio::sync::{broadcast, mpsc};

pub struct Directories {
    projects: PathBuf,
}

impl Directories {
    pub fn new() -> Self {
        let mut root = home::home_dir().unwrap();
        root.push("Bloop");

        let mut projects = root.clone();
        projects.push("Projects");

        Self { projects }
    }
}

pub async fn run(
    request_rx: &mut mpsc::Receiver<request::Request>,
    response_tx: broadcast::Sender<response::Response>,
) {
    let project = projects::generate_project(4, 3, 3);
    let mut project_proxy = proxy::Proxy::new(project, |new_project: &project::Project| {
        send_project_response(&new_project, &response_tx)
    });

    let directories = Directories::new();
    let project_store = project_store::ProjectStore::new(&directories.projects);

    while let Some(request) = request_rx.recv().await {
        match project_store_handlers::handle_request(&request, project_proxy.get().clone(), &project_store) {
            Ok(project) => project_proxy.set(project),
            Err(message) => send_error_response(&message, &response_tx),
        };

        match project_handlers::handle_request(&request, project_proxy.get().clone()) {
            Ok(project) => project_proxy.set(project),
            Err(message) => send_error_response(&message, &response_tx),
        };
    }
}

fn send_project_response(project: &project::Project, response_tx: &broadcast::Sender<response::Response>) {
    let response = response::Response::new().with_project(project);
    send_response(response, response_tx);
}

fn send_error_response(error: &str, response_tx: &broadcast::Sender<response::Response>) {
    let response = response::Response::new().with_error(error);
    send_response(response, response_tx);
}

fn send_response(response: response::Response, response_tx: &broadcast::Sender<response::Response>) {
    response_tx.send(response).unwrap();
}
