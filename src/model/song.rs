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
