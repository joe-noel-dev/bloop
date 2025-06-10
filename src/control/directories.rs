use std::path::PathBuf;

use log::info;

pub struct Directories {
    pub projects: PathBuf,
    pub samples: PathBuf,
    pub root: PathBuf,
    pub backend: PathBuf,
}

impl Directories {
    pub fn new() -> Self {
        let root = if let Ok(bloop_home) = std::env::var("BLOOP_HOME") {
            PathBuf::from(bloop_home)
        } else {
            let mut home = home::home_dir().unwrap();

            if cfg!(target_os = "ios") {
                home.push("Documents");
            }

            home.push("bloop");

            home
        };

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
