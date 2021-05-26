use std::path::{Path, PathBuf};

pub struct Sample {
    path: PathBuf,
    name: String,
    cached: bool,
}

impl Sample {
    pub fn new(name: &str) -> Self {
        Self {
            path: PathBuf::new(),
            name: String::from(name),
            cached: false,
        }
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

    pub fn get_name(&self) -> &str {
        &self.name
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
