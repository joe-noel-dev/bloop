#![allow(dead_code)]

use crate::bloop::{PlaybackState, Progress, Project, ProjectInfo, UploadAck, UserStatusResponse, WaveformResponse};

impl crate::bloop::Response {
    pub fn with_error(mut self, message: &str) -> Self {
        self.error = message.to_string();
        self
    }

    pub fn with_project(mut self, project: &Project) -> Self {
        self.project = Some(project.clone()).into();
        self
    }

    pub fn with_projects(mut self, projects: &[ProjectInfo]) -> Self {
        self.projects = projects.to_vec();
        self
    }

    pub fn with_playback_state(mut self, playback_state: &PlaybackState) -> Self {
        self.playback_state = Some(playback_state.clone()).into();
        self
    }

    pub fn with_waveform(mut self, waveform: &WaveformResponse) -> Self {
        self.waveform = Some(waveform.clone()).into();
        self
    }

    pub fn with_progress(mut self, progress: &Progress) -> Self {
        self.progress = Some(progress.clone()).into();
        self
    }

    pub fn with_upload_ack(mut self, upload_ack: &UploadAck) -> Self {
        self.upload = Some(upload_ack.clone()).into();
        self
    }

    pub fn with_user_status(mut self, user_status: &UserStatusResponse) -> Self {
        self.user_status = Some(user_status.clone()).into();
        self
    }
}
