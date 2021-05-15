use crate::api::request;
use crate::model::project;

type HandlerError = String;

fn unhandled_error() -> HandlerError {
    "Unsupported method".to_string()
}

pub fn handle_request(request: request::Request, project: project::Project) -> Result<project::Project, String> {
    println!("Received message: {:?}", request);

    return match request {
        request::Request::Get(_) => Ok(project),
        request::Request::Add(add_request) => handle_add(project, add_request),
        request::Request::Select(select_request) => handle_select(project, select_request),
        request::Request::Remove(remove_request) => handle_remove(project, remove_request),
        request::Request::Update(update_request) => handle_update(project, update_request),
    };
}

fn handle_add(project: project::Project, request: request::AddRequest) -> Result<project::Project, HandlerError> {
    match request.entity {
        request::Entity::Channel => project.add_channel(),
        request::Entity::Section => handle_add_section(project, request),
        request::Entity::Song => Ok(project.add_song(1)),
        request::Entity::Project => Ok(project::Project::new()),
        _ => Err(unhandled_error()),
    }
}

fn handle_add_section(
    project: project::Project,
    request: request::AddRequest,
) -> Result<project::Project, HandlerError> {
    let song_id = match request.parent_id {
        Some(id) => id,
        None => return Err("Missing parent ID".to_string()),
    };

    project.add_section_to_song(&song_id)
}

fn handle_select(
    project: project::Project,
    select_request: request::SelectRequest,
) -> Result<project::Project, HandlerError> {
    match select_request.entity {
        request::Entity::Song => Ok(project.select_song_with_id(&select_request.id)),
        request::Entity::Section => project.select_section(&select_request.id),
        _ => Err(unhandled_error()),
    }
}

fn handle_remove(
    project: project::Project,
    remove_request: request::RemoveRequest,
) -> Result<project::Project, HandlerError> {
    match remove_request.entity {
        request::Entity::Song => project.remove_song(&remove_request.id),
        request::Entity::Section => project.remove_section(&remove_request.id),
        request::Entity::Channel => project.remove_channel(&remove_request.id),
        _ => Err(unhandled_error()),
    }
}

fn handle_update(
    project: project::Project,
    update_request: request::UpdateRequest,
) -> Result<project::Project, HandlerError> {
    match update_request {
        request::UpdateRequest::Song(song) => project.replace_song(song),
        request::UpdateRequest::Section(section) => project.replace_section(section),
        request::UpdateRequest::Sample(sample) => project.replace_sample(sample),
    }
}
