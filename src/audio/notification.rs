use crate::model::{id::ID, playback_state::PlaybackState, project::Project};

use super::buffer::AudioBuffer;

pub struct SampleConversionResult {
    pub sample_id: ID,
    pub result: Result<Box<dyn AudioBuffer + Send>, String>,
}

pub enum Notification {
    ReturnProject(Box<Project>),
    ReturnSample(Box<dyn AudioBuffer + Send>),
    Transport(PlaybackState),
    SampleConverted(SampleConversionResult),
}
