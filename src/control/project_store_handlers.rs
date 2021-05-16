use super::project_store;
use crate::api::request;
use crate::model::project;

pub fn handle_request(
    request: &request::Request,
    project: project::Project,
    project_store: &project_store::ProjectStore,
) -> Result<project::Project, String> {
    match request {
        request::Request::Save => handle_save(project, project_store),
        _ => Ok(project),
    }
}

fn handle_save(
    project: project::Project,
    project_store: &project_store::ProjectStore,
) -> Result<project::Project, String> {
    match project_store.save(project.clone()) {
        Ok(_) => (),
        Err(error) => return Err(error),
    };

    Ok(project)
}
