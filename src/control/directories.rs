use std::path::PathBuf;

pub struct Directories {
    pub projects: PathBuf,
    pub samples: PathBuf,
    pub preferences: PathBuf,
}

impl Directories {
    pub fn new() -> Self {
        let mut root = home::home_dir().unwrap();
        root.push("Bloop");

        let mut projects = root.clone();
        projects.push("projects");

        let mut samples = root.clone();
        samples.push("samples");

        let mut preferences = root;
        preferences.push("preferences");

        Self {
            projects,
            samples,
            preferences,
        }
    }
}
