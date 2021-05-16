use crate::model::{id::ID, project};
use std::convert::TryInto;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, io::BufReader};

pub struct ProjectStore {
    root_directory: PathBuf,
}

fn current_time() -> i64 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    since_the_epoch.as_millis().try_into().unwrap()
}

impl ProjectStore {
    pub fn new(root_directory: &PathBuf) -> Self {
        if !root_directory.exists() {
            fs::create_dir_all(root_directory).expect(&format!(
                "Couldn't create directory: {}",
                root_directory.to_str().unwrap()
            ));
        }

        Self {
            root_directory: root_directory.clone(),
        }
    }

    pub fn save(&self, mut project: project::Project) -> Result<(), String> {
        project.info.last_saved = current_time();
        self.create_project_directory(&project.info.id)?;
        self.create_samples_directory(&project.info.id)?;
        // TODO: copy samples from cache
        self.write_project_json(project)?;
        Ok(())
    }

    pub fn load(&self, project_id: &ID) -> Result<project::Project, String> {
        self.read_project_json(project_id)
        // TODO:load samples into cache
    }

    fn create_project_directory(&self, project_id: &ID) -> Result<(), String> {
        let project_directory = self.directory_for_project(project_id);
        if !project_directory.exists() {
            match fs::create_dir_all(project_directory) {
                Ok(_) => (),
                Err(error) => return Err(error.to_string()),
            }
        }

        Ok(())
    }

    fn create_samples_directory(&self, project_id: &ID) -> Result<(), String> {
        let samples_directory = self.directory_for_project(project_id);
        if !samples_directory.exists() {
            match fs::create_dir_all(samples_directory) {
                Ok(_) => (),
                Err(error) => return Err(error.to_string()),
            }
        }

        Ok(())
    }

    fn directory_for_project(&self, project_id: &ID) -> PathBuf {
        let mut project_directory = self.root_directory.clone();
        project_directory.push(project_id.to_string());
        project_directory
    }

    fn project_json_path(&self, project_id: &ID) -> PathBuf {
        let mut json_path = self.directory_for_project(project_id);
        json_path.push("project.json");
        json_path
    }

    fn write_project_json(&self, project: project::Project) -> Result<(), String> {
        let json_path = self.project_json_path(&project.info.id);

        let file = match fs::File::create(json_path) {
            Ok(file) => file,
            Err(error) => return Err(error.to_string()),
        };

        match serde_json::to_writer_pretty(file, &project) {
            Ok(_) => (),
            Err(error) => return Err(error.to_string()),
        };

        Ok(())
    }

    fn read_project_json(&self, project_id: &ID) -> Result<project::Project, String> {
        let json_path = self.project_json_path(project_id);
        let file = match fs::File::open(json_path) {
            Ok(file) => file,
            Err(error) => return Err(error.to_string()),
        };
        let reader = BufReader::new(file);
        let project = match serde_json::from_reader(reader) {
            Ok(project) => project,
            Err(error) => return Err(error.to_string()),
        };
        Ok(project)
    }
}

#[cfg(test)]
mod tests {
    use crate::generators;

    use super::*;

    #[test]
    fn creates_directory() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let mut project_dir = temp_dir.into_path();
        project_dir.push("projects");

        assert!(!project_dir.exists());
        ProjectStore::new(&project_dir);
        assert!(project_dir.exists());

        fs::remove_dir_all(project_dir).expect("Failed to remove directory");
    }

    #[test]
    fn save_and_load_project() {
        let temp_dir = tempfile::TempDir::new().unwrap().into_path();
        let project_store = ProjectStore::new(&temp_dir);
        let project = generators::projects::generate_project(3, 4, 5);
        let project_id = project.info.id;
        project_store.save(project).unwrap();

        let project2 = project_store.load(&project_id).unwrap();
        assert_eq!(project2.channels.len(), 3);
        assert_eq!(project2.songs.len(), 4);
        assert_eq!(project2.sections.len(), 20);

        fs::remove_dir_all(temp_dir).unwrap();
    }
}
