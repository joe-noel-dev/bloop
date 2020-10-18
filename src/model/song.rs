use super::id::ID;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Song {
    pub id: ID,
    pub name: String,
    pub tempo: Tempo,
    pub metronome: Metronome,
    pub section_ids: Vec<ID>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Metronome {
    Default,
    CountIn,
    On,
    Off,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Tempo {
    pub bpm: f64,
}

impl Song {
    pub fn new() -> Self {
        Self {
            id: ID::new_v4(),
            name: "Song".to_string(),
            tempo: Tempo { bpm: 120.0 },
            metronome: Metronome::Default,
            section_ids: vec![],
        }
    }

    pub fn with_section_ids(mut self, section_ids: Vec<ID>) -> Self {
        self.section_ids = section_ids;
        self
    }

    pub fn remove_section_id(mut self, section_id: &ID) -> Self {
        self.section_ids = self
            .section_ids
            .iter()
            .filter(|id| id != &section_id)
            .map(|id| id.clone())
            .collect();

        self
    }
}
