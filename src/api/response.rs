#![allow(dead_code)]

use crate::{
    model::{PlaybackState, Progress, Project, ProjectInfo, ID},
    waveform::WaveformData,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WaveformResponse {
    pub sample_id: ID,
    pub waveform_data: WaveformData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UploadAck {
    pub upload_id: ID,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<Project>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<ProjectInfo>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_state: Option<PlaybackState>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub waveform: Option<WaveformResponse>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<Progress>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload: Option<UploadAck>,

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

    pub fn with_waveform(mut self, waveform: WaveformResponse) -> Self {
        self.waveform = Some(waveform);
        self
    }

    pub fn with_progress(mut self, progress: Progress) -> Self {
        self.progress = Some(progress);
        self
    }

    pub fn with_upload_ack(mut self, upload_ack: UploadAck) -> Self {
        self.upload = Some(upload_ack);
        self
    }
}
