use super::{random_id, Section, Song, Tempo, ID, INVALID_ID};

impl Song {
    pub fn empty() -> Self {
        let mut song = Self::new();
        song.id = random_id();
        song.name = "Song".to_string();
        song.tempo = Some(Tempo::new_with_bpm(120.0)).into();
        song
    }

    pub fn with_sections(mut self, sections: Vec<Section>) -> Self {
        self.sections = sections;
        self
    }

    pub fn beat_length(&self) -> f64 {
        debug_assert!(self.is_valid());
        match self.sample.as_ref() {
            Some(sample) => sample.beat_length(),
            None => 0.0,
        }
    }

    pub fn remove_section(mut self, section_id: ID) -> Self {
        self.sections.retain(|section| section.id != section_id);
        self
    }

    pub fn is_valid(&self) -> bool {
        self.id != INVALID_ID
            && !self.sections.is_empty()
            && self.sections.iter().is_sorted_by(|a, b| a.start <= b.start)
    }

    pub fn find_section(&self, section_id: ID) -> Option<&Section> {
        self.sections.iter().find(|section| section.id == section_id)
    }

    pub fn find_section_mut(&mut self, section_id: ID) -> Option<&mut Section> {
        self.sections.iter_mut().find(|section| section.id == section_id)
    }

    pub fn section_length(&self, section_id: ID) -> f64 {
        let index = self
            .sections
            .iter()
            .position(|section| section.id == section_id)
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
        self.id = random_id();

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

    use crate::model::Sample;

    use super::*;

    #[test]
    fn calculates_section_lengths() {
        let mut song = Song::empty();

        let beat_length = 100.0;

        let mut sample = Sample::empty();
        sample.sample_rate = 48_000;
        sample.tempo = Some(Tempo::new_with_bpm(120.0)).into();
        sample.sample_count = (sample.sample_rate as f64 * beat_length / sample.tempo.beat_frequency()).ceil() as i64;

        song.sample = Some(sample).into();

        let section_1 = Section::empty().with_start(23.0);
        let section_2 = Section::empty().with_start(48.0);
        let section_3 = Section::empty().with_start(89.0);
        song.sections = vec![section_1.clone(), section_2.clone(), section_3.clone()];

        assert_relative_eq!(song.section_length(section_1.id), 25.0);
        assert_relative_eq!(song.section_length(section_2.id), 41.0);
        assert_relative_eq!(song.section_length(section_3.id), 11.0);
    }
}
