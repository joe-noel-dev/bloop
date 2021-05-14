use crate::api::request;
use crate::model::{project, selections, song};

type HandlerError = String;

fn unhandled_error() -> HandlerError {
    "Unsupported method".to_string()
}

pub fn handle_add(project: project::Project, request: request::AddRequest) -> Result<project::Project, HandlerError> {
    match request.entity {
        request::Entity::Channel => handle_add_channel(project),
        request::Entity::Section => handle_add_section(project, request),
        request::Entity::Song => handle_add_song(project),
        request::Entity::Project => handle_add_project(),
        _ => Err(unhandled_error()),
    }
}

fn handle_add_channel(mut project: project::Project) -> Result<project::Project, HandlerError> {
    project = match project.add_channel() {
        Ok(project) => project,
        Err(error) => return Err(error),
    };

    Ok(project)
}

fn handle_add_section(
    mut project: project::Project,
    request: request::AddRequest,
) -> Result<project::Project, HandlerError> {
    let song_id = match request.parent_id {
        Some(id) => id,
        None => return Err("Missing parent ID".to_string()),
    };

    project = match project.add_section_to_song(&song_id) {
        Ok(project) => project,
        Err(error) => return Err(error),
    };

    Ok(project)
}

fn handle_add_song(mut project: project::Project) -> Result<project::Project, HandlerError> {
    project = project.add_song(1);

    project.selections = selections::Selections {
        song: Some(project.songs.last().unwrap().id),
        section: None,
    };

    Ok(project)
}

fn handle_add_project() -> Result<project::Project, HandlerError> {
    Ok(project::Project::new())
}

pub fn handle_select(
    project: project::Project,
    select_request: request::SelectRequest,
) -> Result<project::Project, HandlerError> {
    match select_request.entity {
        request::Entity::Song => handle_select_song(project, select_request),
        _ => Err(unhandled_error()),
    }
}

pub fn handle_select_song(
    mut project: project::Project,
    select_request: request::SelectRequest,
) -> Result<project::Project, HandlerError> {
    let song_id = select_request.id;

    if !project.contains_song(&song_id) {
        return Err(format!("Song ID not found to select - {}", song_id));
    }

    project.selections = selections::Selections {
        song: Some(song_id),
        section: None,
    };

    Ok(project)
}

pub fn handle_remove(
    project: project::Project,
    remove_request: request::RemoveRequest,
) -> Result<project::Project, HandlerError> {
    match remove_request.entity {
        request::Entity::Song => handle_remove_song(project, remove_request),
        request::Entity::Section => handle_remove_section(project, remove_request),
        request::Entity::Channel => handle_remove_channel(project, remove_request),
        _ => Err(unhandled_error()),
    }
}

pub fn handle_remove_song(
    mut project: project::Project,
    remove_request: request::RemoveRequest,
) -> Result<project::Project, HandlerError> {
    let song_id = remove_request.id;

    project = match project.remove_song(&song_id) {
        Ok(project) => project,
        Err(error) => return Err(error),
    };

    Ok(project)
}

pub fn handle_remove_section(
    mut project: project::Project,
    remove_request: request::RemoveRequest,
) -> Result<project::Project, HandlerError> {
    project = match project.remove_section(&remove_request.id) {
        Ok(project) => project,
        Err(error) => return Err(error),
    };

    Ok(project)
}

pub fn handle_remove_channel(
    mut project: project::Project,
    remove_request: request::RemoveRequest,
) -> Result<project::Project, HandlerError> {
    project = match project.remove_channel(&remove_request.id) {
        Ok(project) => project,
        Err(error) => return Err(error),
    };

    Ok(project)
}

pub fn handle_update(
    project: project::Project,
    update_request: request::UpdateRequest,
) -> Result<project::Project, HandlerError> {
    match update_request {
        request::UpdateRequest::Song(song) => handle_update_song(project, song),
    }
}

pub fn handle_update_song(mut project: project::Project, song: song::Song) -> Result<project::Project, HandlerError> {
    project = project.replace_song(song)?;
    Ok(project)
}
