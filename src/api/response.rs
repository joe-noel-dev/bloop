use crate::model::{channel, project, sample, section, selections, song};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<project::Project>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<channel::Channel>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub songs: Option<Vec<song::Song>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sections: Option<Vec<section::Section>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub selections: Option<selections::Selections>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub samples: Option<Vec<sample::Sample>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<project::ProjectInfo>>,

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
            channels: None,
            songs: None,
            sections: None,
            selections: None,
            samples: None,
            projects: None,
            error: None,
        }
    }

    pub fn with_error(mut self, message: &str) -> Self {
        self.error = Some(message.to_string());
        self
    }

    pub fn with_project(mut self, project: &project::Project) -> Self {
        self.project = Some(project.clone());
        self
    }

    pub fn with_projects(mut self, projects: &[project::ProjectInfo]) -> Self {
        self.projects = Some(Vec::from(projects));
        self
    }

    pub fn _with_songs(mut self, songs: &[song::Song]) -> Self {
        self.songs = Some(Vec::from(songs));
        self
    }

    pub fn _with_sections(mut self, sections: &[section::Section]) -> Self {
        self.sections = Some(Vec::from(sections));
        self
    }

    pub fn _with_channels(mut self, channels: &[channel::Channel]) -> Self {
        self.channels = Some(Vec::from(channels));
        self
    }

    pub fn _with_selections(mut self, selections: &selections::Selections) -> Self {
        self.selections = Some(selections.clone());
        self
    }
}
