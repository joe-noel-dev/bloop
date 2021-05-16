use std::path::PathBuf;

pub struct Directories {
    pub projects: PathBuf,
}

impl Directories {
    pub fn new() -> Self {
        let mut root = home::home_dir().unwrap();
        root.push("Bloop");

        let mut projects = root.clone();
        projects.push("Projects");

        Self { projects }
    }
}
