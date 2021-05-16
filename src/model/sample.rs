use super::{id::ID, tempo::Tempo};
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Sample {
    pub id: ID,
    pub name: String,
    pub tempo: Tempo,
    pub sample_rate: i32,
    pub sample_count: i64,
    pub channel_count: i32,
}

impl Sample {
    pub fn new() -> Self {
        Sample {
            id: ID::new_v4(),
            name: "".to_string(),
            tempo: Tempo { bpm: 120.0 },
            sample_rate: 0,
            sample_count: 0,
            channel_count: 0,
        }
    }

    pub fn is_valid(&self) -> bool {
        true
    }
}
