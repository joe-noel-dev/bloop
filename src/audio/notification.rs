use crate::model::{playback_state::PlaybackState, project::Project};

use super::buffer::AudioBuffer;
pub enum Notification {
    ReturnProject(Box<Project>),
    ReturnSample(Box<dyn AudioBuffer + Send>),
    Transport(PlaybackState),
}
