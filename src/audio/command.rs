use crate::model::{id::ID, project::Project};

pub struct QueueCommand {
    pub song_id: ID,
    pub section_id: ID,
}

pub enum Command {
    UpdateProject(Project),
    Play,
    Stop,
    EnterLoop,
    ExitLoop,
    Queue(QueueCommand),
}
