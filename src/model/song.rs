use super::id::ID;
use super::state::State;

#[derive(Debug)]
pub struct Song {
    pub id: ID,
    pub state: State,
    pub name: String,
    pub tempo: Tempo,
    pub metronome: Metronome,
    pub section_ids: Vec<ID>,
}

#[derive(Debug)]
pub enum Metronome {
    Default,
    // CountIn,
    // On,
    // Off,
}

#[derive(Debug)]
pub struct Tempo {
    pub bpm: f64,
}
