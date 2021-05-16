use super::project_store;
use crate::api::{request, response::Response, response::ResponseBroadcaster};
use crate::model::project;

pub fn handle_request(
    request: &request::Request,
    project: project::Project,
    project_store: &project_store::ProjectStore,
    response_broadcaster: &dyn ResponseBroadcaster,
) {
    let result = match request {
        request::Request::Get(get_request) => handle_get(get_request, project_store, response_broadcaster),
        request::Request::Save => project_store.save(project.clone()),
        _ => Ok(()),
    };

    match result {
        Err(error) => response_broadcaster.broadcast(Response::new().with_error(&error)),
        Ok(_) => (),
    };
}

fn handle_get(
    request: &request::GetRequest,
    project_store: &project_store::ProjectStore,
    response_broadcaster: &dyn ResponseBroadcaster,
) -> Result<(), String> {
    match request.entity {
        request::Entity::Projects => handle_get_projects(project_store, response_broadcaster),
        _ => Ok(()),
    }
}

fn handle_get_projects(
    project_store: &project_store::ProjectStore,
    response_broadcaster: &dyn ResponseBroadcaster,
) -> Result<(), String> {
    let projects = match project_store.projects() {
        Ok(projects) => projects,
        Err(error) => return Err(error),
    };

    response_broadcaster.broadcast(Response::new().with_projects(&projects));
    Ok(())
}
