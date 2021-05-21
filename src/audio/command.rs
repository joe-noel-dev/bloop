use crate::model::{id::ID, project::Project};

use super::buffer::AudioBuffer;

pub struct QueueCommand {
    pub song_id: ID,
    pub section_id: ID,
}

pub struct AddSampleCommand {
    pub sample_id: ID,
    pub audio_data: Box<dyn AudioBuffer + Send>,
}

pub struct RemoveSampleCommand {
    pub sample_id: ID,
}

pub enum Command {
    AddSample(AddSampleCommand),
    RemoveSample(RemoveSampleCommand),
    UpdateProject(Box<Project>),
    Play,
    Stop,
    EnterLoop,
    ExitLoop,
    Queue(QueueCommand),
}
