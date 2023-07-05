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
    pub sections: Vec<Section>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample: Option<Sample>,
}

impl Default for Song {
    fn default() -> Self {
        Self {
            id: ID::new_v4(),
            name: "Song".to_string(),
            tempo: Tempo::new(120.0),
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

    pub fn beat_length(&self) -> f64 {
        debug_assert!(self.is_valid());
        match &self.sample {
            Some(sample) => sample.beat_length(),
            None => 0.0,
        }
    }

    pub fn remove_section(mut self, section_id: &ID) -> Self {
        self.sections.retain(|section| section.id != *section_id);
        self
    }

    pub fn is_valid(&self) -> bool {
        !self.id.is_nil()
            && !self.sections.is_empty()
            && self.sections.iter().is_sorted_by(|a, b| a.start.partial_cmp(&b.start))
    }

    pub fn find_section(&self, section_id: &ID) -> Option<&Section> {
        self.sections.iter().find(|section| section.id == *section_id)
    }

    pub fn find_section_mut(&mut self, section_id: &ID) -> Option<&mut Section> {
        self.sections.iter_mut().find(|section| section.id == *section_id)
    }

    pub fn section_length(&self, section_id: &ID) -> f64 {
        let index = self
            .sections
            .iter()
            .position(|section| section.id == *section_id)
            .expect("Section not found");

        let section = self.sections.get(index);
        let next_section = self.sections.get(index + 1);

        let mut start = 0.0;
        let mut end = self.beat_length();

        if let Some(section) = section {
            start = section.start;
        }

        if let Some(next_section) = next_section {
            end = next_section.start;
        }

        if start > end {
            return 0.0;
        }

        end - start
    }

    pub fn replace_ids(mut self) -> Self {
        self.id = ID::new_v4();

        self.sections = self
            .sections
            .iter()
            .map(|section| section.clone().replace_ids())
            .collect();

        // Don't replace the ID in the sample, since this is also referenced on disk

        self
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn calculates_section_lengths() {
        let mut song = Song::default();

        let beat_length = 100.0;

        let mut sample = Sample::new();
        sample.sample_rate = 48_000;
        sample.tempo = Tempo::new(120.0);
        sample.sample_count = (sample.sample_rate as f64 * beat_length / sample.tempo.beat_frequency()).ceil() as i64;

        song.sample = Some(sample);

        let mut section_1 = Section::new();
        section_1.start = 23.0;

        let mut section_2 = Section::new();
        section_2.start = 48.0;

        let mut section_3 = Section::new();
        section_3.start = 89.0;
        song.sections = vec![section_1.clone(), section_2.clone(), section_3.clone()];

        assert_relative_eq!(song.section_length(&section_1.id), 25.0);
        assert_relative_eq!(song.section_length(&section_2.id), 41.0);
        assert_relative_eq!(song.section_length(&section_3.id), 11.0);
    }
}
