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
    pub error: Option<String>,
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

    pub fn with_songs(self, songs: &[song::Song]) -> Self {
        let mut response = self.clone();
        response.songs = Some(Vec::from(songs));
        response
    }

    pub fn with_sections(mut self, sections: &[section::Section]) -> Self {
        self.sections = Some(Vec::from(sections));
        self
    }

    pub fn with_channels(mut self, channels: &[channel::Channel]) -> Self {
        self.channels = Some(Vec::from(channels));
        self
    }

    pub fn with_selections(mut self, selections: &selections::Selections) -> Self {
        self.selections = Some(selections.clone());
        self
    }
}
