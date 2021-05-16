use super::{directories::Directories, project_handlers, project_store::ProjectStore, project_store_handlers};
use crate::generators::projects;
use crate::model::project;
use crate::{
    api::{request, response},
    model::proxy::NotifyingProxy,
    model::proxy::Proxy,
};
use tokio::sync::{broadcast, mpsc};

pub async fn run(
    request_rx: &mut mpsc::Receiver<request::Request>,
    response_tx: broadcast::Sender<response::Response>,
) {
    let project = projects::generate_project(4, 3, 3);
    let mut project_proxy = NotifyingProxy::new(project, |new_project: &project::Project| {
        send_project_response(&new_project, &response_tx)
    });

    let directories = Directories::new();
    let project_store = ProjectStore::new(&directories.projects);

    let send_response = |response| send_response(response, &response_tx);

    while let Some(request) = request_rx.recv().await {
        println!("Received message: {:?}", request);

        project_store_handlers::handle_request(&request, project_proxy.get(), &project_store, &send_response);
        project_handlers::handle_request(&request, &mut project_proxy, &send_response);
    }
}

fn send_project_response(project: &project::Project, response_tx: &broadcast::Sender<response::Response>) {
    let response = response::Response::new().with_project(project);
    send_response(response, response_tx);
}

fn send_response(response: response::Response, response_tx: &broadcast::Sender<response::Response>) {
    response_tx.send(response).unwrap();
}
