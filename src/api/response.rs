use crate::model::{channel, project, sample, section, song};
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
            samples: None,
            error: None,
        }
    }

    pub fn with_error(&self, message: &str) -> Self {
        let mut response = self.clone();
        response.error = Some(message.to_string());
        response
    }

    pub fn with_project(&self, project: &project::Project) -> Self {
        let mut response = self.clone();
        response.project = Some(project.clone());
        response
    }

    pub fn with_songs(&self, songs: &[song::Song]) -> Self {
        let mut response = self.clone();
        response.songs = Some(Vec::from(songs));
        response
    }

    pub fn with_sections(&self, sections: &[section::Section]) -> Self {
        let mut response = self.clone();
        response.sections = Some(Vec::from(sections));
        response
    }

    pub fn with_channels(&self, channels: &[channel::Channel]) -> Self {
        let mut response = self.clone();
        response.channels = Some(Vec::from(channels));
        response
    }
}
