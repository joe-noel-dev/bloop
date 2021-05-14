use super::id::ID;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Section {
    pub id: ID,
    pub name: String,
    pub beat_length: f64,
    #[serde(rename = "loop")]
    pub loop_properties: LoopProperties,
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
            beat_length: 0.0,
            loop_properties: LoopProperties {
                mode: LoopMode::Fixed,
                count: 1,
            },
            samples: vec![],
        }
    }
}
