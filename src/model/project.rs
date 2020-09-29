use super::channel::Channel;
use super::id::ID;
use super::sample::Sample;
use super::section::Section;
use super::song::Song;
use super::state::State;

#[derive(Debug)]
pub struct Project {
    pub id: ID,
    pub state: State,
    pub info: ProjectInfo,
    pub songs: Vec<Song>,
    pub sections: Vec<Section>,
    pub channels: Vec<Channel>,
    pub samples: Vec<Sample>,
}

#[derive(Debug)]
pub struct ProjectInfo {
    pub name: String,
    pub version: String,
}
