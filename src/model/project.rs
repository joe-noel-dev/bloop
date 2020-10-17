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
    pub fn _new() -> Self {
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

    pub fn contains_song(&self, song_id: ID) -> bool {
        self.songs.iter().find(|s| s.id == song_id).is_some()
    }

    pub fn contains_section(&self, section_id: ID) -> bool {
        self.sections.iter().find(|section| section.id == section_id).is_some()
    }

    pub fn contains_channel(&self, channel_id: ID) -> bool {
        self.channels.iter().find(|channel| channel.id == channel_id).is_some()
    }

    pub fn remove_sections_for_song(mut self, song: &Song) -> Self {
        self.sections = self
            .sections
            .iter_mut()
            .filter(|section| !song.section_ids.contains(&section.id))
            .map(|section| section.clone())
            .collect();
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

    pub fn remove_section(mut self, section_id: &ID) -> Result<Self, String> {
        self.sections = self
            .sections
            .iter()
            .filter(|section| &section.id != section_id)
            .map(|section| section.clone())
            .collect();

        self.songs = self
            .songs
            .iter()
            .map(|song| song.clone().remove_section_id(section_id))
            .collect();

        Ok(self)
    }

    pub fn remove_channel(mut self, channel_id: &ID) -> Self {
        self.channels = self
            .channels
            .iter()
            .filter(|channel| &channel.id != channel_id)
            .map(|channel| channel.clone())
            .collect();

        // TODO: Remove channels in sections

        self
    }

    pub fn remove_song(mut self, song_id: &ID) -> Result<Self, String> {
        let selected_song_index = self.selected_song_index();

        let song = match self.song_with_id(song_id) {
            Some(song) => song.clone(),
            None => return Err(format!("Song not found with ID {}", song_id)),
        };

        self = self.remove_sections_for_song(&song);

        self.songs = self
            .songs
            .iter_mut()
            .filter(|song| &song.id != song_id)
            .map(|song| song.clone())
            .collect();

        if let Some(selected_song_index) = selected_song_index {
            self = self.select_song_index(selected_song_index);
        }

        Ok(self)
    }
}

impl ProjectInfo {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            version: "1".to_string(),
        }
    }
}
