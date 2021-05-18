use super::{
    directories::Directories, project_handlers, project_store::ProjectStore, project_store_handlers, transport_handlers,
};
use crate::{
    api::request::Entity,
    audio::manager::Audio,
    audio::manager::AudioManager,
    control::sample_handlers,
    model::{project::Project, proxy::Proxy},
    samples::cache::SamplesCache,
};
use crate::{
    api::{request::GetRequest, response::ResponseBroadcaster},
    generators::projects,
};
use crate::{
    api::{request::Request, response::Response},
    model::proxy::NotifyingProxy,
};
use tokio::sync::{broadcast, mpsc};

pub async fn run(request_rx: &mut mpsc::Receiver<Request>, response_tx: broadcast::Sender<Response>) {
    let project = projects::generate_project(4, 3, 3);
    let mut project_proxy = NotifyingProxy::new(project, |new_project: &Project| {
        send_project_response(&new_project, &response_tx)
    });

    let directories = Directories::new();
    let mut samples_cache = SamplesCache::new(&directories.samples);
    let mut project_store = ProjectStore::new(&directories.projects);
    let audio_manager = AudioManager::new();

    let send_response = |response| send_response(response, &response_tx);

    while let Some(request) = request_rx.recv().await {
        project_store_handlers::handle_request(
            &request,
            &mut project_proxy,
            &mut project_store,
            &mut samples_cache,
            &send_response,
        );
        sample_handlers::handle_request(&request, &mut project_proxy, &mut samples_cache, &send_response);
        project_handlers::handle_request(&request, &mut project_proxy, &send_response);
        transport_handlers::handle_request(&request, &audio_manager);

        if let Request::Get(get_request) = request {
            handle_get(&get_request, &project_proxy, &send_response);
        }
    }
}

fn handle_get(
    get_request: &GetRequest,
    project_proxy: &dyn Proxy<Project>,
    response_broadcaster: &dyn ResponseBroadcaster,
) {
    if let Entity::All = get_request.entity {
        response_broadcaster.broadcast(Response::new().with_project(&project_proxy.get()))
    }
}

fn send_project_response(project: &Project, response_tx: &broadcast::Sender<Response>) {
    send_response(Response::new().with_project(project), response_tx);
}

fn send_response(response: Response, response_tx: &broadcast::Sender<Response>) {
    response_tx.send(response).unwrap();
}
