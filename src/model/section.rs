use super::id::ID;
use super::state::State;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Section {
    pub id: ID,
    pub state: State,
    pub name: String,
    pub beat_length: f64,
    #[serde(rename = "loop")]
    pub loop_properties: LoopProperties,
    pub samples: Vec<ChannelSamplePair>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoopProperties {
    pub mode: LoopMode,
    pub count: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChannelSamplePair {
    pub channel_id: ID,
    pub sample_id: ID,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum LoopMode {
    Fixed,
    Indefinite,
}
