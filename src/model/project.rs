use super::id::ID;
use super::sample::Sample;
use super::section::Section;
use super::selections::Selections;
use super::song::Song;
use anyhow::anyhow;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub info: ProjectInfo,
    pub songs: Vec<Song>,
    pub selections: Selections,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ProjectInfo {
    pub id: ID,
    pub name: String,
    pub version: String,
    pub last_saved: i64,
}

impl Default for Project {
    fn default() -> Self {
        Self::empty()
    }
}

impl Project {
    pub fn empty() -> Self {
        Self {
            info: ProjectInfo::default(),
            songs: vec![],
            selections: Selections::default(),
        }
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.info.name = String::from(name);
        self
    }

    pub fn with_songs(mut self, num_songs: usize, num_sections: usize) -> Self {
        assert!(num_songs >= 1);
        self.songs.clear();
        for _ in 0..num_songs {
            self = self.add_song(num_sections);
        }

        self = self.select_song_index(0);
        self
    }

    pub fn new() -> Self {
        Self::empty().with_songs(1, 1)
    }

    pub fn song_with_id(&self, id: &ID) -> Option<&Song> {
        self.songs.iter().find(|s| s.id == *id)
    }

    pub fn song_with_id_mut(&mut self, id: &ID) -> Option<&mut Song> {
        self.songs.iter_mut().find(|s| s.id == *id)
    }

    pub fn section_with_id(&self, id: &ID) -> Option<&Section> {
        for song in self.songs.iter() {
            if let Some(section) = song.find_section(id) {
                return Some(section);
            }
        }

        None
    }

    pub fn replace_song(mut self, song: &Song) -> anyhow::Result<Self> {
        if !song.is_valid() {
            return Err(anyhow!("Invalid song"));
        }

        let old_song = match self.songs.iter_mut().find(|s| s.id == song.id) {
            Some(song) => song,
            None => return Err(anyhow!("Song not found")),
        };

        *old_song = song.clone();

        Ok(self)
    }

    pub fn add_section_to_song(self, song_id: &ID) -> anyhow::Result<Self> {
        let song = self
            .song_with_id(song_id)
            .with_context(|| format!("Couldn't find song ID {song_id}"))?;

        let mut song = song.clone();

        let mut start = 0.0;

        if let Some(last_section) = song.sections.last() {
            let default_length = 16.0;
            start = last_section.start + default_length;
        }

        let section = Section::new().with_start(start);

        song.sections.push(section);

        self.replace_song(&song)
    }

    pub fn contains_song(&self, song_id: &ID) -> bool {
        self.songs.iter().any(|s| s.id == *song_id)
    }

    pub fn selected_song_index(&self) -> Option<usize> {
        let song_id = self.selections.song?;
        self.songs.iter().position(|song| song.id == song_id)
    }

    pub fn song_with_index(&self, index: usize) -> Option<&Song> {
        self.songs.get(index)
    }

    pub fn select_song_index(self, song_index: usize) -> Self {
        let song_index = std::cmp::min(song_index, self.songs.len() - 1);

        let selected_song_id = match self.song_with_index(song_index) {
            Some(song) => song.id,
            None => {
                return self;
            }
        };

        self.select_song_with_id(&selected_song_id)
    }

    pub fn select_song_with_id(mut self, song_id: &ID) -> Self {
        if let Some(song) = self.song_with_id(song_id) {
            self.selections = Selections {
                song: Some(*song_id),
                section: song.sections.first().map(|section| section.id),
            }
        }

        self
    }

    pub fn song_with_section(&self, section_id: &ID) -> Option<&Song> {
        self.songs
            .iter()
            .find(|song| song.sections.iter().any(|section| section.id == *section_id))
    }

    pub fn remove_section(mut self, section_id: &ID) -> anyhow::Result<Self> {
        let mut song = self
            .song_with_section(section_id)
            .with_context(|| format!("Couldn't find song with section ID: {section_id}"))?
            .clone();

        if song.sections.len() < 2 {
            return Err(anyhow!("Can't remove last section"));
        }

        let section_index = song.sections.iter().position(|section| section.id == *section_id);

        song = song.remove_section(section_id);

        self = self.replace_song(&song)?;

        if !self.selection_is_valid() {
            self = match section_index {
                Some(index) => self.select_section_at_index(index)?,
                None => self.select_last_song(),
            }
        }

        if !self.is_valid() {
            return Err(anyhow!("Project is in an invalid state"));
        }

        Ok(self)
    }

    pub fn selected_song(&self) -> Option<&Song> {
        let song_id = self.selections.song?;
        self.song_with_id(&song_id)
    }

    pub fn select_section_at_index(mut self, index: usize) -> anyhow::Result<Self> {
        let song = match self.selected_song() {
            Some(song) => song,
            None => return Err(anyhow!("No song selected")),
        };

        let index = index.min(song.sections.len() - 1);
        let new_section_id = song.sections[index].id;

        self = self.select_section(&new_section_id)?;

        Ok(self)
    }

    pub fn selection_is_valid(&self) -> bool {
        let song_id = match self.selections.song {
            Some(song_id) => song_id,
            None => return false,
        };

        let section_id = match self.selections.section {
            Some(section_id) => section_id,
            None => return false,
        };

        let song = match self.song_with_id(&song_id) {
            Some(song) => song,
            None => return false,
        };

        return song.sections.iter().any(|section| section.id == section_id);
    }

    pub fn add_song(mut self, num_sections: usize) -> Self {
        assert!(num_sections >= 1);
        let sections: Vec<Section> = (0..num_sections).map(|_| Section::new()).collect();
        let song = Song::default().with_sections(sections);
        self.songs.push(song);
        self.select_last_song()
    }

    pub fn remove_song(mut self, song_id: &ID) -> anyhow::Result<Self> {
        if self.songs.len() < 2 {
            return Err(anyhow!("Can't remove last song"));
        }

        if !self.contains_song(song_id) {
            return Err(anyhow!("Song ID not found to remove - {}", song_id));
        }

        let selected_song_index = self.selected_song_index();

        self.songs.retain(|song| &song.id != song_id);

        if !self.selection_is_valid() {
            self = match selected_song_index {
                Some(index) => self.select_song_index(index),
                None => self.select_last_song(),
            };
        }

        Ok(self)
    }

    pub fn replace_section(mut self, new_section: &Section) -> anyhow::Result<Self> {
        if !new_section.is_valid() {
            return Err(anyhow!("Invalid section"));
        }

        self.songs
            .iter_mut()
            .filter_map(|song| song.find_section_mut(&new_section.id))
            .for_each(|section| *section = new_section.clone());

        if !self.is_valid() {
            return Err(anyhow!("Project in an invalid state"));
        }

        Ok(self)
    }

    pub fn is_valid(&self) -> bool {
        self.songs.iter().all(|song| song.is_valid())
    }

    pub fn replace_sample(mut self, sample: &Sample) -> anyhow::Result<Self> {
        if !sample.is_valid() {
            return Err(anyhow!("Invalid sample"));
        }

        let old_sample = match self.find_sample_mut(&sample.id) {
            Some(sample) => sample,
            None => return Err(anyhow!("Sample not found: {}", sample.id)),
        };

        *old_sample = sample.clone();

        self.songs
            .iter_mut()
            .filter(|song| song.sample.is_some() && song.sample.as_ref().unwrap().id == sample.id)
            .for_each(|song| song.tempo = sample.tempo);

        Ok(self)
    }

    pub fn add_sample_to_song(mut self, sample: Sample, song_id: &ID) -> anyhow::Result<Self> {
        let song = self
            .song_with_id_mut(song_id)
            .ok_or_else(|| anyhow!("Couldn't find song with ID: {}", song_id))?;

        let tempo = sample.tempo;
        song.sample = Some(sample);
        song.tempo = tempo;

        Ok(self)
    }

    pub fn select_last_song(self) -> Self {
        let last_song_id = match self.songs.last() {
            Some(song) => song.id,
            None => {
                return self;
            }
        };

        self.select_song_with_id(&last_song_id)
    }

    pub fn select_section(mut self, section_id: &ID) -> anyhow::Result<Self> {
        if self.section_with_id(section_id).is_none() {
            return Err(anyhow!("Couldn't find section with ID: {}", section_id));
        }

        let song_id = self
            .song_with_section(section_id)
            .ok_or_else(|| anyhow!("Couldn't find song with Section ID: {}", section_id))?
            .id;

        self.selections = Selections {
            song: Some(song_id),
            section: Some(*section_id),
        };

        Ok(self)
    }

    #[allow(dead_code)]
    pub fn select_next_song(mut self) -> Self {
        let selected_song_index = match self.selected_song_index() {
            Some(index) => index,
            None => {
                return self;
            }
        };

        if selected_song_index < self.songs.len() - 1 {
            self = self.select_song_index(selected_song_index + 1)
        }

        self
    }

    #[allow(dead_code)]
    pub fn select_previous_song(mut self) -> Self {
        let selected_song_index = match self.selected_song_index() {
            Some(index) => index,
            None => {
                return self;
            }
        };

        if selected_song_index > 0 {
            self = self.select_song_index(selected_song_index - 1)
        }

        self
    }

    #[allow(dead_code)]
    pub fn select_next_section(self) -> anyhow::Result<Self> {
        let song_id = self.selections.song.ok_or_else(|| anyhow!("No song selected"))?;
        let section_id = self.selections.section.ok_or_else(|| anyhow!("No section selected"))?;

        let song = self
            .song_with_id(&song_id)
            .ok_or_else(|| anyhow!("Couldn't find song with ID: {}", song_id))?;

        let current_section_index = match song.sections.iter().position(|section| section.id == section_id) {
            Some(position) => position,
            None => {
                return Ok(self);
            }
        };

        if current_section_index >= song.sections.len() - 1 {
            return Ok(self);
        }

        let next_section_id = song.sections[current_section_index + 1].id;
        self.select_section(&next_section_id)
    }

    pub fn select_previous_section(self) -> anyhow::Result<Self> {
        let song_id = self.selections.song.ok_or_else(|| anyhow!("No song selected"))?;
        let section_id = self.selections.section.ok_or_else(|| anyhow!("No section selected"))?;

        let song = self
            .song_with_id(&song_id)
            .ok_or_else(|| anyhow!("Couldn't find song with ID: {}", song_id))?;

        let current_section_index = match song.sections.iter().position(|section| section.id == section_id) {
            Some(position) => position,
            None => {
                return Ok(self);
            }
        };

        if current_section_index == 0 {
            return Ok(self);
        }

        let next_section_id = song.sections[current_section_index - 1].id;
        self.select_section(&next_section_id)
    }

    pub fn remove_sample_from_song(mut self, song_id: &ID) -> anyhow::Result<Self> {
        if let Some(song) = self.song_with_id_mut(song_id) {
            song.sample = None;
        }

        Ok(self)
    }

    pub fn find_sample(&self, sample_id: &ID) -> Option<&Sample> {
        for song in self.songs.iter() {
            if let Some(sample) = &song.sample {
                if sample.id == *sample_id {
                    return Some(sample);
                }
            }
        }

        None
    }

    pub fn find_sample_mut(&mut self, sample_id: &ID) -> Option<&mut Sample> {
        for song in self.songs.iter_mut() {
            if let Some(sample) = &mut song.sample {
                if sample.id == *sample_id {
                    return Some(sample);
                }
            }
        }

        None
    }

    pub fn replace_ids(mut self) -> Self {
        self.info.id = ID::new_v4();
        self.songs = self.songs.iter().map(|song| song.clone().replace_ids()).collect();
        self.selections.song = self.songs.first().map(|song| song.id);
        self.selections.section = (|| Some(self.songs.first()?.sections.first()?.id))();
        self
    }
}

impl Default for ProjectInfo {
    fn default() -> Self {
        Self {
            id: ID::new_v4(),
            name: "Project".to_string(),
            version: "1".to_string(),
            last_saved: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_with_songs() {
        let num_songs = 10;
        let num_sections = 10;
        let project = Project::new().with_songs(num_songs, num_sections);
        assert_eq!(project.songs.len(), num_songs);
        assert!(project.songs.iter().all(|song| song.sections.len() == num_sections));
    }

    #[test]
    fn get_song_by_id() {
        let project = Project::new().with_songs(5, 5);
        let song = &project.songs[2];
        let retrieved_song = match project.song_with_id(&song.id) {
            Some(song) => song,
            None => panic!("Couldn't find song"),
        };
        assert_eq!(retrieved_song, song);
    }

    #[test]
    fn get_missing_song_by_id() {
        let project = Project::new().with_songs(5, 5);
        let retrieved_song = project.song_with_id(&ID::new_v4());
        assert!(retrieved_song.is_none());
    }

    #[test]
    fn replace_song() {
        let mut project = Project::new().with_songs(5, 5);
        let mut song = project.songs[3].clone();
        song.name = "New song name".to_string();
        project = project.replace_song(&song).expect("Couldn't replace song");
        assert_eq!(project.songs[3].name, "New song name");
    }

    #[test]
    fn select_next_song() {
        let mut project = Project::new().with_songs(5, 5);
        let song_id = project.songs[1].id;
        project = project.select_next_song();
        let selected_song_id = project.selections.song.expect("No song selected");
        assert_eq!(selected_song_id, song_id);
    }

    #[test]
    fn select_next_song_from_end() {
        let mut project = Project::new().with_songs(5, 5);
        project = project.select_last_song();
        let song_id = project.songs[4].id;
        project = project.select_next_song();
        let selected_song_id = project.selections.song.expect("No song selected");
        assert_eq!(selected_song_id, song_id);
    }

    #[test]
    fn select_previous_song() {
        let mut project = Project::new().with_songs(5, 5);
        project = project.select_last_song();
        let song_id = project.songs[3].id;
        project = project.select_previous_song();
        let selected_song_id = project.selections.song.expect("No song selected");
        assert_eq!(selected_song_id, song_id);
    }

    #[test]
    fn select_previous_song_from_start() {
        let mut project = Project::new().with_songs(5, 5);
        let song_id = project.songs[0].id;
        project = project.select_previous_song();
        let selected_song_id = project.selections.song.expect("No song selected");
        assert_eq!(selected_song_id, song_id);
    }

    #[test]
    fn select_next_section() {
        let mut project = Project::new().with_songs(5, 5);
        let section_id = project.songs[0].sections[1].id;
        project = project.select_next_section().expect("Couldn't select next section");
        let selected_section_id = project.selections.section.expect("No section selected");
        assert_eq!(selected_section_id, section_id);
    }

    #[test]
    fn select_previous_section() {
        let mut project = Project::new().with_songs(5, 5);
        let initial_section_id = project.songs[0].sections[4].id;
        project = project
            .select_section(&initial_section_id)
            .expect("Couldn't select initial section");
        project = project.select_previous_section().expect("Couldn't select next section");
        let selected_section_id = project.selections.section.expect("No section selected");
        assert_eq!(selected_section_id, project.songs[0].sections[3].id);
    }
}
