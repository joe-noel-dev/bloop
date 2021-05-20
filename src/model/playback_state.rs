use super::id::ID;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum PlayingState {
    Stopped,
    Playing,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlaybackState {
    pub playing: PlayingState,
    pub song_id: Option<ID>,
    pub section_id: Option<ID>,

    pub queued_song_id: Option<ID>,
    pub queued_section_id: Option<ID>,

    pub looping: bool,
    pub loop_count: i32,
}

impl PlaybackState {
    pub fn new() -> Self {
        Self {
            playing: PlayingState::Stopped,
            song_id: Option::None,
            section_id: Option::None,
            queued_song_id: Option::None,
            queued_section_id: Option::None,
            looping: false,
            loop_count: 0,
        }
    }
}
