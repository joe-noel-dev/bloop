use super::channel::Channel;
use super::id::ID;
use super::sample::Sample;
use super::section::Section;
use super::selections::Selections;
use super::song::Song;
use serde::{Deserialize, Serialize};
use std::{cmp::PartialEq, collections::HashSet};

use std::iter::FromIterator;

pub const MAX_CHANNELS: usize = 8;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub info: ProjectInfo,
    pub songs: Vec<Song>,
    pub sections: Vec<Section>,
    pub channels: Vec<Channel>,
    pub samples: Vec<Sample>,
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

impl Project {
    pub fn empty() -> Self {
        Self {
            info: ProjectInfo::new(),
            songs: vec![],
            sections: vec![],
            channels: vec![],
            samples: vec![],
            selections: Selections::new(),
        }
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.info.name = String::from(name);
        self
    }

    pub fn with_songs(mut self, num_songs: usize, num_sections: usize) -> Self {
        assert!(num_songs >= 1);
        self.songs.clear();
        self.sections.clear();
        for _ in 0..num_songs {
            self = self.add_song(num_sections);
        }

        self = self.select_song_index(0);
        self
    }

    pub fn with_channels(mut self, num_channels: usize) -> Self {
        assert!(1 <= num_channels && num_channels <= MAX_CHANNELS);
        self.channels.clear();
        for _ in 0..num_channels {
            self = self.add_channel().unwrap()
        }
        self
    }

    pub fn new() -> Self {
        Self::empty().with_songs(1, 1).with_channels(1)
    }

    pub fn song_with_id(&self, id: &ID) -> Option<&Song> {
        self.songs.iter().find(|s| s.id == *id)
    }

    pub fn song_with_id_mut(&mut self, id: &ID) -> Option<&mut Song> {
        self.songs.iter_mut().find(|s| s.id == *id)
    }

    pub fn section_with_id(&self, id: &ID) -> Option<&Section> {
        self.sections.iter().find(|s| s.id == *id)
    }

    pub fn replace_song(mut self, song: &Song) -> Result<Self, String> {
        if !song.is_valid() {
            return Err("Invalid song".to_string());
        }

        let old_song = match self.songs.iter_mut().find(|s| s.id == song.id) {
            Some(song) => song,
            None => return Err("Song not found".to_string()),
        };

        *old_song = song.clone();
        Ok(self)
    }

    pub fn add_section_to_song(mut self, song_id: &ID) -> Result<Self, String> {
        let mut song = match self.song_with_id(song_id) {
            Some(song) => song.clone(),
            None => return Err(format!("Couldn't find song ID {}", song_id)),
        };

        let mut start = 0.0;
        let mut length = 16.0;

        if let Some(last_section_id) = song.section_ids.last() {
            if let Some(last_section) = self.section_with_id(last_section_id) {
                start = last_section.start + last_section.beat_length;
                length = last_section.beat_length;
            }
        }

        let section = Section::new().with_start(start).with_beat_length(length);

        song.section_ids.push(section.id);

        self.sections.push(section);

        self.replace_song(&song)
    }

    pub fn add_channel(mut self) -> Result<Self, String> {
        if self.channels.len() >= MAX_CHANNELS {
            return Err("Max channels reached".to_string());
        }

        self.channels.push(Channel::new());

        Ok(self)
    }

    pub fn contains_song(&self, song_id: &ID) -> bool {
        self.songs.iter().find(|s| s.id == *song_id).is_some()
    }

    pub fn contains_section(&self, section_id: &ID) -> bool {
        self.sections.iter().find(|section| section.id == *section_id).is_some()
    }

    pub fn contains_channel(&self, channel_id: &ID) -> bool {
        self.channels.iter().find(|channel| channel.id == *channel_id).is_some()
    }

    pub fn remove_sections_for_song(mut self, song: &Song) -> Self {
        self.sections.retain(|section| !song.section_ids.contains(&section.id));
        self
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
            if self.selections.song.unwrap_or(ID::nil()) != song.id {
                self.selections = Selections {
                    song: Some(*song_id),
                    section: match song.section_ids.first() {
                        Some(section_id) => Some(*section_id),
                        None => None,
                    },
                }
            }
        }

        self
    }

    pub fn song_with_section(&self, section_id: &ID) -> Option<&Song> {
        self.songs.iter().find(|song| song.section_ids.contains(&section_id))
    }

    pub fn song_with_section_mut(&mut self, section_id: &ID) -> Option<&mut Song> {
        self.songs
            .iter_mut()
            .find(|song| song.section_ids.contains(&section_id))
    }

    pub fn remove_section(mut self, section_id: &ID) -> Result<Self, String> {
        if !self.contains_section(section_id) {
            return Err(format!("Section ID not found to remove - {}", section_id));
        }

        let mut song = match self.song_with_section(&section_id) {
            Some(song) => song.clone(),
            None => return Err(format!("Couldn't find containing song for section - {}", section_id)),
        };

        if song.section_ids.len() < 2 {
            return Err("Can't remove last section".to_string());
        }

        song = song.remove_section_id(section_id);
        self = self.replace_song(&song)?;

        self.sections.retain(|section| &section.id != section_id);

        // FIXME: Selections?

        Ok(self)
    }

    pub fn remove_channel(mut self, channel_id: &ID) -> Result<Self, String> {
        if self.channels.len() < 2 {
            return Err(format!("Can't remove last channel"));
        }

        if !self.contains_channel(channel_id) {
            return Err(format!("Channel ID not found to remove - {}", channel_id));
        }

        self.channels.retain(|channel| &channel.id != channel_id);

        for section in &mut self.sections {
            section.samples.retain(|pair| &pair.channel_id != channel_id);
        }

        Ok(self)
    }

    pub fn add_song(mut self, num_sections: usize) -> Self {
        assert!(num_sections >= 1);
        let mut sections: Vec<Section> = (0..num_sections).map(|_| Section::new()).collect();
        let song = Song::new().with_section_ids(sections.iter().map(|section| section.id).collect());
        self.songs.push(song);
        self.sections.append(&mut sections);
        self
    }

    pub fn remove_song(mut self, song_id: &ID) -> Result<Self, String> {
        if self.songs.len() < 2 {
            return Err("Can't remove last song".to_string());
        }

        if !self.contains_song(&song_id) {
            return Err(format!("Song ID not found to remove - {}", song_id));
        }

        let selected_song_index = self.selected_song_index();

        let song = match self.song_with_id(song_id) {
            Some(song) => song.clone(),
            None => return Err(format!("Song not found with ID {}", song_id)),
        };

        self = self.remove_sections_for_song(&song);

        self.songs.retain(|song| &song.id != song_id);

        if let Some(selected_song_index) = selected_song_index {
            self = self.select_song_index(selected_song_index);
        }

        Ok(self.remove_unused_samples())
    }

    pub fn replace_section(mut self, section: &Section) -> Result<Self, String> {
        if !section.is_valid() {
            return Err("Invalid section".to_string());
        }

        let old_section = match self.sections.iter_mut().find(|s| s.id == section.id) {
            Some(section) => section,
            None => return Err(format!("Section not found: {}", section.id)),
        };

        *old_section = section.clone();
        Ok(self)
    }

    pub fn replace_sample(mut self, sample: &Sample) -> Result<Self, String> {
        if !sample.is_valid() {
            return Err("Invalid sample".to_string());
        }

        let old_sample = match self.samples.iter_mut().find(|s| s.id == sample.id) {
            Some(sample) => sample,
            None => return Err(format!("Sample not found: {}", sample.id)),
        };

        *old_sample = sample.clone();
        Ok(self)
    }

    pub fn sample_with_id(&self, sample_id: &ID) -> Option<&Sample> {
        self.samples.iter().find(|sample| sample.id == *sample_id)
    }

    fn remove_unused_samples(mut self) -> Self {
        let samples_in_use: HashSet<ID> = HashSet::from_iter(
            self.songs
                .iter()
                .filter(|song| song.sample_id.is_some())
                .map(|song| song.sample_id.unwrap()),
        );

        self.samples.retain(|sample| samples_in_use.contains(&sample.id));
        self
    }

    pub fn add_sample_to_song(mut self, sample: Sample, song_id: &ID) -> Result<Self, String> {
        let song = match self.song_with_id_mut(song_id) {
            Some(song) => song,
            None => return Err(format!("Couldn't find song with ID: {}", song_id)),
        };

        song.sample_id = Some(sample.id);
        self.samples.push(sample);
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

    pub fn select_section(mut self, section_id: &ID) -> Result<Self, String> {
        if self.section_with_id(&section_id).is_none() {
            return Err(format!("Couldn't find section with ID: {}", section_id));
        }

        let song_id = match self.song_with_section(section_id) {
            Some(song) => song.id,
            None => {
                return Err(format!("Couldn't find song with Section ID: {}", section_id));
            }
        };

        self = self.select_song_with_id(&song_id);
        self.selections.section = Some(*section_id);

        Ok(self)
    }

    pub fn selected_song(&self) -> Option<&Song> {
        let selected_song_id = match self.selections.song {
            Some(id) => id,
            None => {
                return None;
            }
        };

        self.song_with_id(&selected_song_id)
    }

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

    pub fn select_next_section(self) -> Result<Self, String> {
        let song_id = match self.selections.song {
            Some(id) => id,
            None => {
                return Err("No song selected".to_string());
            }
        };

        let section_id = match self.selections.section {
            Some(id) => id,
            None => {
                return Err("No section selected".to_string());
            }
        };

        let song = match self.song_with_id(&song_id) {
            Some(song) => song,
            None => {
                return Err(format!("Couldn't find song with ID: {}", song_id));
            }
        };

        let current_section_index = match song.section_ids.iter().position(|id| *id == section_id) {
            Some(position) => position,
            None => {
                return Ok(self);
            }
        };

        if current_section_index >= song.section_ids.len() - 1 {
            return Ok(self);
        }

        let next_section_id = song.section_ids[current_section_index + 1];
        self.select_section(&next_section_id)
    }

    pub fn select_previous_section(self) -> Result<Self, String> {
        let song_id = match self.selections.song {
            Some(id) => id,
            None => {
                return Err("No song selected".to_string());
            }
        };

        let section_id = match self.selections.section {
            Some(id) => id,
            None => {
                return Err("No section selected".to_string());
            }
        };

        let song = match self.song_with_id(&song_id) {
            Some(song) => song,
            None => {
                return Err(format!("Couldn't find song with ID: {}", song_id));
            }
        };

        let current_section_index = match song.section_ids.iter().position(|id| *id == section_id) {
            Some(position) => position,
            None => {
                return Ok(self);
            }
        };

        if current_section_index == 0 {
            return Ok(self);
        }

        let next_section_id = song.section_ids[current_section_index - 1];
        self.select_section(&next_section_id)
    }

    pub fn remove_sample(mut self, sample_id: &ID, song_id: &ID) -> Result<Self, String> {
        if self.sample_with_id(&sample_id).is_none() {
            return Err(format!("Sample not found with ID: {}", sample_id));
        }

        let mut song = match self.song_with_id_mut(&song_id) {
            Some(song) => song,
            None => {
                return Err(format!("Song not found with ID: {}", song_id));
            }
        };

        song.sample_id = None;

        Ok(self.remove_unused_samples())
    }
}

impl ProjectInfo {
    pub fn new() -> Self {
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
        assert_eq!(project.sections.len(), num_songs * num_sections);
    }

    #[test]
    fn get_song_by_id() {
        let project = Project::new().with_songs(5, 5);
        let song = &project.songs[2];
        let retrieved_song = match project.song_with_id(&song.id) {
            Some(song) => song,
            None => return assert!(false),
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
        let section_id = project.songs[0].section_ids[1];
        project = project.select_next_section().expect("Couldn't select next section");
        let selected_section_id = project.selections.section.expect("No section selected");
        assert_eq!(selected_section_id, section_id);
    }

    #[test]
    fn select_previous_section() {
        let mut project = Project::new().with_songs(5, 5);
        let initial_section_id = project.songs[0].section_ids[4];
        project = project
            .select_section(&initial_section_id)
            .expect("Couldn't select initial section");
        project = project.select_previous_section().expect("Couldn't select next section");
        let selected_section_id = project.selections.section.expect("No section selected");
        assert_eq!(selected_section_id, project.songs[0].section_ids[3]);
    }
}
