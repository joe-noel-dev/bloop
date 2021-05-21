use crate::model::{playback_state::PlaybackState, project::Project};
pub enum Notification {
    ReturnProject(Box<Project>),
    Transport(PlaybackState),
}
