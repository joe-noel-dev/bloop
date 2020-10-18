use super::channel::Channel;
use super::id::ID;
use super::sample::Sample;
use super::section::Section;
use super::selections::Selections;
use super::song::Song;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

pub const MAX_CHANNELS: usize = 8;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: ID,
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
    pub name: String,
    pub version: String,
}

impl Project {
    pub fn empty() -> Self {
        Self {
            id: ID::new_v4(),
            info: ProjectInfo::new(),
            songs: vec![],
            sections: vec![],
            channels: vec![],
            samples: vec![],
            selections: Selections::new(),
        }
    }

    pub fn with_songs(mut self, num_songs: usize) -> Self {
        self.songs.clear();
        for _ in 0..num_songs {
            self = self.add_song();
        }

        self = self.select_song_index(0);
        self
    }

    pub fn with_channels(mut self, num_channels: usize) -> Self {
        self.channels.clear();
        for _ in 0..std::cmp::min(num_channels, MAX_CHANNELS) {
            self = self.add_channel().unwrap()
        }
        self
    }

    pub fn new() -> Self {
        Self::empty().with_songs(1).with_channels(1)
    }

    pub fn _get_channel_ids(&self) -> Vec<ID> {
        return self.channels.iter().map(|c| c.id.clone()).collect::<Vec<uuid::Uuid>>();
    }

    pub fn song_with_id(&self, id: &ID) -> Option<&Song> {
        self.songs.iter().find(|s| s.id == *id)
    }

    pub fn _section_with_id(&self, id: &ID) -> Option<&Section> {
        self.sections.iter().find(|s| s.id == *id)
    }

    pub fn replace_song(mut self, song: Song) -> Self {
        let old_song = match self.songs.iter_mut().find(|s| s.id == song.id) {
            Some(song) => song,
            None => return self,
        };

        *old_song = song;
        self
    }

    pub fn add_section_to_song(mut self, song_id: &ID) -> Result<Self, String> {
        let section = Section::new();

        let mut song = match self.song_with_id(song_id) {
            Some(song) => song.clone(),
            None => return Err(format!("Couldn't find song ID {}", song_id)),
        };

        song.section_ids.push(section.id);

        self.sections.push(section);

        self = self.replace_song(song);
        Ok(self)
    }

    pub fn add_channel(mut self) -> Result<Self, String> {
        if self.channels.len() >= MAX_CHANNELS {
            return Err("Max channels reached".to_string());
        }

        let channel = Channel::new();
        self.channels.push(channel);

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
        let song_id = match self.selections.song {
            Some(song_id) => song_id,
            None => return None,
        };

        match self.songs.iter().position(|song| song.id == song_id) {
            Some(index) => Some(index),
            None => return None,
        }
    }

    pub fn song_with_index(&self, index: usize) -> Option<&Song> {
        if index >= self.songs.len() {
            return None;
        } else {
            return Some(&self.songs[index]);
        }
    }

    pub fn select_song_index(mut self, song_index: usize) -> Self {
        if self.songs.len() == 0 {
            return self;
        }
        let song_index = std::cmp::min(song_index, self.songs.len() - 1);
        if let Some(new_selected_song) = self.song_with_index(song_index) {
            self.selections = Selections {
                song: Some(new_selected_song.id),
                section: None,
                channel: None,
            }
        }

        self
    }

    pub fn song_with_section(&self, section_id: &ID) -> Option<Song> {
        match self.songs.iter().find(|song| song.section_ids.contains(&section_id)) {
            Some(song) => Some(song.clone()),
            None => None,
        }
    }

    pub fn remove_section(mut self, section_id: &ID) -> Result<Self, String> {
        if !self.contains_section(section_id) {
            return Err(format!("Section ID not found to remove - {}", section_id));
        }

        let mut song = match self.song_with_section(&section_id) {
            Some(song) => song,
            None => return Err(format!("Couldn't find containing song for section - {}", section_id)),
        };

        if song.section_ids.len() < 2 {
            return Err("Can't remove last section".to_string());
        }

        song = song.remove_section_id(section_id);
        self = self.replace_song(song);

        self.sections.retain(|section| &section.id != section_id);

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

        // TODO: Remove channels in sections

        Ok(self)
    }

    pub fn add_song(mut self) -> Self {
        let section = Section::new();
        let song = Song::new().with_section_ids(vec![section.id]);
        self.songs.push(song);
        self.sections.push(section);
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

        Ok(self)
    }
}

impl ProjectInfo {
    pub fn new() -> Self {
        Self {
            name: "Project".to_string(),
            version: "1".to_string(),
        }
    }
}
