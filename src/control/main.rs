use super::{
    directories::Directories, project_handlers, project_store::ProjectStore, project_store_handlers, transport_handlers,
};
use crate::{
    api::request::Entity,
    audio::manager::{Audio, AudioManager},
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
use futures::StreamExt;
use tokio::sync::{broadcast, mpsc};

pub async fn run(request_rx: &mut mpsc::Receiver<Request>, response_tx: broadcast::Sender<Response>) {
    let directories = Directories::new();
    let mut samples_cache = SamplesCache::new(&directories.samples);
    let mut project_store = ProjectStore::new(&directories.projects);

    let (audio_notification_tx, mut audio_notification_rx) = futures_channel::mpsc::channel(100);

    let mut audio_manager = AudioManager::new(audio_notification_tx);

    let mut project_proxy = NotifyingProxy::new(projects::generate_project(4, 3, 3), |new_project: &Project| {
        send_project_response(&new_project, &response_tx)
    });

    let send_response = |response| send_response(response, &response_tx);

    loop {
        tokio::select! {
            Some(request) = request_rx.recv() => handle_request(request, &mut project_proxy, &mut project_store, &mut samples_cache, &mut audio_manager, &send_response),
            Some(notification) =  audio_notification_rx.next() => audio_manager.on_notification(notification, &send_response),
            else => break,
        }
    }
}

fn handle_request(
    request: Request,
    project_proxy: &mut dyn Proxy<Project>,
    project_store: &mut ProjectStore,
    samples_cache: &mut SamplesCache,
    audio_manager: &mut dyn Audio,
    response_broadcaster: &dyn ResponseBroadcaster,
) {
    project_store_handlers::handle_request(
        &request,
        project_proxy,
        project_store,
        samples_cache,
        response_broadcaster,
    );
    sample_handlers::handle_request(&request, project_proxy, samples_cache, response_broadcaster);
    project_handlers::handle_request(&request, project_proxy, response_broadcaster);
    transport_handlers::handle_request(&request, audio_manager);

    if let Request::Get(get_request) = request {
        handle_get(&get_request, project_proxy, response_broadcaster);
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
