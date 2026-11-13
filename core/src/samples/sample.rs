use std::path::{Path, PathBuf};

use log::error;

pub struct Sample {
    generation: u64,
    path: PathBuf,
    tmp_path: PathBuf,
    name: String,
    cached: bool,
}

impl Sample {
    pub fn new(name: &str, generation: u64) -> Self {
        Self {
            generation,
            path: PathBuf::new(),
            tmp_path: PathBuf::new(),
            name: String::from(name),
            cached: false,
        }
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn is_cached(&self) -> bool {
        self.cached
    }

    pub fn set_cached(&mut self, cached: bool) {
        self.cached = cached
    }

    pub fn get_path(&self) -> &Path {
        self.path.as_path()
    }

    pub fn set_cache_location(&mut self, path: &Path) {
        self.path = PathBuf::from(path);
    }

    pub fn get_tmp_path(&self) -> &Path {
        self.tmp_path.as_path()
    }

    pub fn set_tmp_location(&mut self, path: &Path) {
        self.tmp_path = PathBuf::from(path);
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn delete_sample_on_disk(&self) {
        for path in [&self.path, &self.tmp_path] {
            if !path.exists() {
                continue;
            }
            match std::fs::remove_file(path) {
                Ok(_) => (),
                Err(error) => error!("Failed to remove sample from disk ({}): {}", path.display(), error),
            }
        }
    }
}

impl Drop for Sample {
    fn drop(&mut self) {
        self.delete_sample_on_disk();
    }
}
