use super::tempo::Tempo;
use super::{id::ID, Section};
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Song {
    pub id: ID,
    pub name: String,
    pub tempo: Tempo,
    pub metronome: Metronome,
    pub sections: Vec<Section>,
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

impl Default for Song {
    fn default() -> Self {
        Self {
            id: ID::new_v4(),
            name: "Song".to_string(),
            tempo: Tempo { bpm: 120.0 },
            metronome: Metronome::Default,
            sections: vec![],
            sample_id: None,
        }
    }
}

impl Song {
    pub fn with_sections(mut self, sections: Vec<Section>) -> Self {
        self.sections = sections;
        self
    }

    pub fn remove_section(mut self, section_id: &ID) -> Self {
        self.sections.retain(|section| section.id != *section_id);
        self
    }

    pub fn is_valid(&self) -> bool {
        !self.id.is_nil() && self.tempo.is_valid()
    }

    pub fn find_section(&self, section_id: &ID) -> Option<&Section> {
        self.sections.iter().find(|section| section.id == *section_id)
    }

    pub fn find_section_mut(&mut self, section_id: &ID) -> Option<&mut Section> {
        self.sections.iter_mut().find(|section| section.id == *section_id)
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
