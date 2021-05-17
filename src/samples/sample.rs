use std::path::{Path, PathBuf};

use crate::model::id::ID;

#[derive(PartialEq)]
pub enum CacheState {
    Init,
    Cached,
}

pub struct Sample {
    id: ID,
    cache_state: CacheState,
    path: PathBuf,
}

impl Sample {
    pub fn new(id: &ID) -> Self {
        Self {
            id: *id,
            cache_state: CacheState::Init,
            path: PathBuf::new(),
        }
    }

    pub fn reset(&mut self) {
        self.delete_sample_on_disk();
        self.set_cache_location(&PathBuf::new());
    }

    pub fn get_id(&self) -> &ID {
        &self.id
    }

    pub fn get_cache_state(&self) -> &CacheState {
        &self.cache_state
    }

    pub fn get_path(&self) -> &Path {
        self.path.as_path()
    }

    pub fn set_cache_location(&mut self, path: &Path) {
        if path.exists() {
            self.cache_state = CacheState::Cached;
        } else {
            self.cache_state = CacheState::Init;
        }

        self.path = PathBuf::from(path);
    }

    pub fn delete_sample_on_disk(&self) {
        if self.path.exists() {
            match std::fs::remove_file(&self.path) {
                Ok(_) => (),
                Err(error) => println!(
                    "Failed to remove sample from disk ({}): {}",
                    self.path.display(),
                    error.to_string()
                ),
            }
        }
    }
}

impl Drop for Sample {
    fn drop(&mut self) {
        self.delete_sample_on_disk();
    }
}
