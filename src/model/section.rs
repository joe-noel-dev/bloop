use super::id::ID;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Section {
    pub id: ID,
    pub name: String,
    pub start: f64,
    #[serde(rename = "loop")]
    pub looping: bool,

    #[serde(default)]
    pub metronome: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LoopProperties {
    pub mode: LoopMode,
    pub count: i32,
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
            looping: false,
            metronome: false,
        }
    }

    pub fn with_start(mut self, start: f64) -> Self {
        self.start = start;
        self
    }

    pub fn is_valid(&self) -> bool {
        self.start >= 0.0
    }

    pub fn replace_ids(mut self) -> Self {
        self.id = ID::new_v4();
        self
    }
}
