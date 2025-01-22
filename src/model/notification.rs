use super::{PlaybackState, Progress};

#[derive(Clone, Debug)]
pub struct Notification {
    pub playback_state: PlaybackState,
    pub progress: Progress,
}
