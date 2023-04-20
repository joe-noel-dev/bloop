use super::id::ID;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum PlayingState {
    Stopped,
    Playing,
}

impl Default for PlayingState {
    fn default() -> Self {
        Self::Stopped
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlaybackState {
    pub playing: PlayingState,
    pub song_id: Option<ID>,
    pub section_id: Option<ID>,

    pub queued_song_id: Option<ID>,
    pub queued_section_id: Option<ID>,

    pub looping: bool,
}
