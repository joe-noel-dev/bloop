use super::id::ID;
use super::state::State;

#[derive(Debug)]
pub struct Section {
    pub id: ID,
    pub state: State,
    pub name: String,
    pub beat_length: f64,
    pub loop_properties: LoopProperties,
    pub samples: Vec<ChannelSamplePair>,
}

#[derive(Debug)]
pub struct LoopProperties {
    pub mode: LoopMode,
    pub count: u32,
}

#[derive(Debug)]
pub struct ChannelSamplePair {
    pub channel_id: ID,
    pub sample_id: ID,
}

#[derive(Debug)]
pub enum LoopMode {
    Fixed,
    Indefinite,
}
