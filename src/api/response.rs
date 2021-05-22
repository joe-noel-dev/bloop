use crate::model::{playback_state::PlaybackState, project::Project, project::ProjectInfo};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<Project>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<ProjectInfo>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_state: Option<PlaybackState>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

pub trait ResponseBroadcaster {
    fn broadcast(&self, response: Response);
}

impl<F> ResponseBroadcaster for F
where
    F: Fn(Response),
{
    fn broadcast(&self, response: Response) {
        self(response);
    }
}

impl Response {
    pub fn new() -> Self {
        Response {
            project: None,
            projects: None,
            playback_state: None,
            error: None,
        }
    }

    pub fn with_error(mut self, message: &str) -> Self {
        self.error = Some(message.to_string());
        self
    }

    pub fn with_project(mut self, project: &Project) -> Self {
        self.project = Some(project.clone());
        self
    }

    pub fn with_projects(mut self, projects: &[ProjectInfo]) -> Self {
        self.projects = Some(Vec::from(projects));
        self
    }

    pub fn with_playback_state(mut self, playback_state: &PlaybackState) -> Self {
        self.playback_state = Some(playback_state.clone());
        self
    }
}
