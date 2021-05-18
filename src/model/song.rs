use super::id::ID;
use super::tempo::Tempo;
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
    pub sample_id: Option<ID>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Metronome {
    Default,
    CountIn,
    On,
    Off,
}

impl Song {
    pub fn new() -> Self {
        Self {
            id: ID::new_v4(),
            name: "Song".to_string(),
            tempo: Tempo { bpm: 120.0 },
            metronome: Metronome::Default,
            section_ids: vec![],
            sample_id: None,
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
            .copied()
            .collect();

        self
    }

    pub fn is_valid(&self) -> bool {
        !self.id.is_nil() && self.tempo.is_valid()
    }
}

impl Tempo {
    pub fn min() -> f64 {
        30.0
    }

    pub fn max() -> f64 {
        300.0
    }

    pub fn is_valid(&self) -> bool {
        Self::min() <= self.bpm && self.bpm <= Self::max()
    }
}
