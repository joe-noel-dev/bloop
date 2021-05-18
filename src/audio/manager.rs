use crate::model::id::ID;

use super::process::Process;

pub trait Audio {
    fn play(&self);
    fn stop(&self);
    fn enter_loop(&self);
    fn exit_loop(&self);
    fn queue(&self, song_id: &ID, section_id: &ID);
    fn queue_selected(&self);
}

pub struct AudioManager {
    process: Process,
}

impl AudioManager {
    pub fn new() -> Self {
        Self {
            process: Process::new(),
        }
    }
}

impl Audio for AudioManager {
    fn play(&self) {
        println!("Play");
    }

    fn stop(&self) {
        println!("Stop");
    }

    fn enter_loop(&self) {
        println!("Enter loop");
    }

    fn exit_loop(&self) {
        println!("Exit loop");
    }

    fn queue(&self, song_id: &ID, section_id: &ID) {
        println!("Queue song {}, section {}", song_id, section_id);
    }

    fn queue_selected(&self) {
        println!("Queue selected");
    }
}
