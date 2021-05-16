use super::project_store::ProjectStore;
use crate::{
    api::{
        request::{Entity, GetRequest, LoadRequest, RemoveRequest, Request},
        response::Response,
        response::ResponseBroadcaster,
    },
    model::{project::Project, proxy::Proxy},
};

pub fn handle_request(
    request: &Request,
    project_proxy: &mut dyn Proxy<Project>,
    project_store: &ProjectStore,
    response_broadcaster: &dyn ResponseBroadcaster,
) {
    let result = match request {
        Request::Get(get_request) => handle_get(get_request, project_store, response_broadcaster),
        Request::Save => project_store.save(project_proxy.get()),
        Request::Load(load_request) => handle_load(load_request, project_store, project_proxy),
        Request::Remove(remove_request) => handle_remove(remove_request, project_store, response_broadcaster),
        _ => Ok(()),
    };

    match result {
        Err(error) => response_broadcaster.broadcast(Response::new().with_error(&error)),
        Ok(_) => (),
    };
}

fn handle_get(
    request: &GetRequest,
    project_store: &ProjectStore,
    response_broadcaster: &dyn ResponseBroadcaster,
) -> Result<(), String> {
    match request.entity {
        Entity::Projects => handle_get_projects(project_store, response_broadcaster),
        _ => Ok(()),
    }
}

fn handle_get_projects(
    project_store: &ProjectStore,
    response_broadcaster: &dyn ResponseBroadcaster,
) -> Result<(), String> {
    let projects = match project_store.projects() {
        Ok(projects) => projects,
        Err(error) => return Err(error),
    };

    response_broadcaster.broadcast(Response::new().with_projects(&projects));
    Ok(())
}

fn handle_load(
    request: &LoadRequest,
    project_store: &ProjectStore,
    project_proxy: &mut dyn Proxy<Project>,
) -> Result<(), String> {
    let project = project_store.load(&request.id)?;
    project_proxy.set(project);
    Ok(())
}

fn handle_remove(
    request: &RemoveRequest,
    project_store: &ProjectStore,
    response_broadcaster: &dyn ResponseBroadcaster,
) -> Result<(), String> {
    match request.entity {
        Entity::Project => project_store.remove_project(&request.id)?,
        _ => return Ok(()),
    };

    handle_get_projects(project_store, response_broadcaster)
}
