use super::tempo::Tempo;
use super::Sample;
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
    pub sample: Option<Sample>,
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
            tempo: Tempo::new(120.0),
            metronome: Metronome::Default,
            sections: vec![],
            sample: None,
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
        !self.id.is_nil()
    }

    pub fn find_section(&self, section_id: &ID) -> Option<&Section> {
        self.sections.iter().find(|section| section.id == *section_id)
    }

    pub fn find_section_mut(&mut self, section_id: &ID) -> Option<&mut Section> {
        self.sections.iter_mut().find(|section| section.id == *section_id)
    }

    pub fn section_length(&self, section_id: &ID) -> Option<f64> {
        let mut start: Option<f64> = None;

        for section in self.sections.iter() {
            if let Some(start) = start {
                let end = section.start;
                if end >= start {
                    return Some(end - start);
                }
            }

            if section.id == *section_id {
                start = Some(section.start);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn calculates_section_lengths() {
        let mut song = Song::default();
        let mut section_1 = Section::new();
        section_1.start = 23.0;

        let mut section_2 = Section::new();
        section_2.start = 48.0;

        let mut section_3 = Section::new();
        section_3.start = 89.0;
        song.sections = vec![section_1.clone(), section_2.clone(), section_3.clone()];

        assert_relative_eq!(song.section_length(&section_1.id).unwrap(), 25.0);
        assert_relative_eq!(song.section_length(&section_2.id).unwrap(), 41.0);
        assert!(song.section_length(&section_3.id).is_none());
    }
}
