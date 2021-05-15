use super::id::ID;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Section {
    pub id: ID,
    pub name: String,
    pub start: f64,
    pub beat_length: f64,
    #[serde(rename = "loop")]
    pub looping: bool,
    pub samples: Vec<ChannelSamplePair>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LoopProperties {
    pub mode: LoopMode,
    pub count: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChannelSamplePair {
    pub channel_id: ID,
    pub sample_id: ID,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum LoopMode {
    Fixed,
    Indefinite,
}

impl Section {
    pub fn new() -> Self {
        Section {
            id: ID::new_v4(),
            name: "Section".to_string(),
            start: 0.0,
            beat_length: 0.0,
            looping: false,
            samples: vec![],
        }
    }

    pub fn with_start(mut self, start: f64) -> Self {
        self.start = start;
        self
    }

    pub fn with_beat_length(mut self, beat_length: f64) -> Self {
        self.beat_length = beat_length;
        self
    }

    pub fn is_valid(&self) -> bool {
        true
    }
}
