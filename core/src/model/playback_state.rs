use super::{PlaybackState, PlayingState};

impl PlaybackState {
    pub fn is_playing(&self) -> bool {
        self.playing.enum_value_or_default() == PlayingState::PLAYING
    }

    pub fn is_stopped(&self) -> bool {
        self.playing.enum_value_or_default() == PlayingState::STOPPED
    }
}
