use std::path::PathBuf;

use log::info;

pub struct Directories {
    pub projects: PathBuf,
    pub samples: PathBuf,
    pub root: PathBuf,
    pub backend: PathBuf,
}

impl Directories {
    pub fn new(root: PathBuf) -> Self {
        info!("Using home directory: {}", root.display());

        let mut projects = root.clone();
        projects.push("projects");

        let mut samples = root.clone();
        samples.push("samples");

        let mut backend = root.clone();
        backend.push("backend");

        Self {
            projects,
            samples,
            root,
            backend,
        }
    }
}
