use crate::model::{channel, project, sample, section, song};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub project: Option<project::Project>,
    pub channels: Option<Vec<channel::Channel>>,
    pub songs: Option<Vec<song::Song>>,
    pub sections: Option<Vec<section::Section>>,
    pub samples: Option<Vec<sample::Sample>>,
    pub error: Option<String>,
}

impl Response {
    pub fn new() -> Self {
        Response {
            project: Option::None,
            channels: Option::None,
            songs: Option::None,
            sections: Option::None,
            samples: Option::None,
            error: Option::None,
        }
    }
}
