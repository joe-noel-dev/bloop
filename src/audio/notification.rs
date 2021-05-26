use crate::model::{id::ID, playback_state::PlaybackState, progress::Progress, project::Project};
use anyhow::Result;

use super::buffer::OwnedAudioBuffer;

pub struct SampleConversionResult {
    pub sample_id: ID,
    pub result: Result<Box<OwnedAudioBuffer>>,
}

pub enum Notification {
    ReturnProject(Box<Project>),
    ReturnSample(Box<OwnedAudioBuffer>),
    Transport(PlaybackState),
    SampleConverted(SampleConversionResult),
    Progress(Progress),
}
