use super::channel::Channel;
use super::id::ID;
use super::sample::Sample;
use super::section::Section;
use super::selections::Selections;
use super::song::Song;
use super::state::State;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

pub const MAX_CHANNELS: usize = 8;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: ID,
    pub state: State,
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
            state: State::Active,
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

    pub fn song_with_id(&self, id: ID) -> Option<&Song> {
        self.songs.iter().find(|s| s.id == id)
    }

    fn song_with_id_mut(&mut self, id: ID) -> Option<&mut Song> {
        self.songs.iter_mut().find(|s| s.id == id)
    }

    pub fn section_with_id(&self, id: ID) -> Option<&Section> {
        self.sections.iter().find(|s| s.id == id)
    }

    pub fn section_with_id_mut(&mut self, id: ID) -> Option<&mut Section> {
        self.sections.iter_mut().find(|s| s.id == id)
    }

    pub fn add_section_to_song(&mut self, song_id: ID) -> Option<ID> {
        let section = Section::new();

        let song = match self.song_with_id_mut(song_id) {
            Some(song) => song,
            None => return None,
        };

        song.section_ids.push(section.id);
        self.sections.push(section.clone());
        Some(section.id)
    }

    pub fn contains_song(&self, song_id: ID) -> bool {
        self.songs.iter().find(|s| s.id == song_id).is_some()
    }

    pub fn remove_sections_for_song(&mut self, song: &mut Song) {
        song.section_ids
            .iter()
            .map(|section_id| self.section_with_id_mut(section_id.clone()))
            .filter(|section| section.is_some())
            .map(|section| section.unwrap().state = State::Deleted);
    }

    pub fn remove_song(&mut self, song_id: ID) -> Result<(), String> {
        let song = match self.song_with_id_mut(song_id) {
            Some(song) => song,
            None => return Err(format!("Song not found with ID {}", song_id)),
        };

        self.remove_sections_for_song(song);
        song.state = State::Deleted;
        Ok(())
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
