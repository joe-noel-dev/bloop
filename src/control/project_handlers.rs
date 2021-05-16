use crate::{
    api::request::Request,
    api::response::Response,
    api::{
        request::AddRequest,
        request::{Entity, RemoveRequest, RenameRequest, SelectRequest, UpdateRequest},
        response::ResponseBroadcaster,
    },
    model::{project::Project, proxy::Proxy},
};

use crate::model::project;

pub fn handle_request(
    request: &Request,
    project_proxy: &mut dyn Proxy<Project>,
    response_broadcaster: &dyn ResponseBroadcaster,
) {
    let existing_project = project_proxy.get();

    let result = match request {
        Request::Add(add_request) => handle_add(existing_project, add_request),
        Request::Select(select_request) => handle_select(existing_project, select_request),
        Request::Remove(remove_request) => handle_remove(existing_project, remove_request),
        Request::Update(update_request) => handle_update(existing_project, update_request),
        Request::Rename(rename_reuqest) => handle_rename(existing_project, rename_reuqest),
        _ => Ok(existing_project),
    };

    match result {
        Ok(project) => project_proxy.set(project),
        Err(error) => response_broadcaster.broadcast(Response::new().with_error(&error)),
    };
}

fn handle_add(project: project::Project, request: &AddRequest) -> Result<Project, String> {
    match request.entity {
        Entity::Channel => project.add_channel(),
        Entity::Section => handle_add_section(project, request),
        Entity::Song => Ok(project.add_song(1)),
        Entity::Project => Ok(project::Project::new()),
        _ => Ok(project),
    }
}

fn handle_add_section(project: project::Project, request: &AddRequest) -> Result<Project, String> {
    let song_id = match request.id {
        Some(id) => id,
        None => return Err("Missing parent ID".to_string()),
    };

    project.add_section_to_song(&song_id)
}

fn handle_select(project: project::Project, select_request: &SelectRequest) -> Result<Project, String> {
    match select_request.entity {
        Entity::Song => Ok(project.select_song_with_id(&select_request.id)),
        Entity::Section => project.select_section(&select_request.id),
        _ => Ok(project),
    }
}

fn handle_remove(project: Project, remove_request: &RemoveRequest) -> Result<Project, String> {
    match remove_request.entity {
        Entity::Song => project.remove_song(&remove_request.id),
        Entity::Section => project.remove_section(&remove_request.id),
        Entity::Channel => project.remove_channel(&remove_request.id),
        _ => Ok(project),
    }
}

fn handle_update(project: Project, update_request: &UpdateRequest) -> Result<Project, String> {
    match update_request {
        UpdateRequest::Song(song) => project.replace_song(song),
        UpdateRequest::Section(section) => project.replace_section(section),
        UpdateRequest::Sample(sample) => project.replace_sample(sample),
    }
}

fn handle_rename(project: Project, rename_request: &RenameRequest) -> Result<Project, String> {
    match rename_request.entity {
        Entity::Project => Ok(project.with_name(&rename_request.name)),
        _ => Ok(project),
    }
}
