use super::{PlaybackState, Progress};

pub struct Notification {
    pub playback_state: PlaybackState,
    pub progress: Progress,
}
