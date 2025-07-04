#![allow(dead_code)]

use crate::bloop::{
    PlaybackState, Progress, Project, ProjectInfo, ProjectSyncResponse, UploadAck, User, UserStatusResponse,
    WaveformResponse,
};

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

    pub fn with_cloud_projects(mut self, projects: &[ProjectInfo]) -> Self {
        self.cloud_projects = projects.to_vec();
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

    pub fn with_project_info(mut self, project_info: &ProjectInfo) -> Self {
        self.project_info = Some(project_info.clone()).into();
        self
    }

    pub fn with_user(mut self, user: Option<User>) -> Self {
        self.user_status = Some({
            UserStatusResponse {
                user: user.into(),
                ..Default::default()
            }
        })
        .into();
        self
    }

    pub fn with_project_sync(mut self, project_sync: &ProjectSyncResponse) -> Self {
        self.project_sync = Some(project_sync.clone()).into();
        self
    }
}
