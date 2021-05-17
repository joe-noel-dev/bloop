use crate::{
    model::{
        id::ID,
        project::{Project, ProjectInfo},
    },
    samples::{cache::SamplesCache, sample::CacheState},
    types::audio_file_format::AudioFileFormat,
};
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

    pub fn save(&self, mut project: Project, samples_cache: &SamplesCache) -> Result<(), String> {
        project.info.last_saved = current_time();
        self.create_project_directory(&project.info.id)?;
        self.create_samples_directory(&project.info.id)?;
        self.copy_samples_from_cache(&project, samples_cache)?;
        self.write_project_json(project)?;
        Ok(())
    }

    pub fn load(&mut self, project_id: &ID, samples_cache: &mut SamplesCache) -> Result<Project, String> {
        let project = self.read_project_json(project_id)?;
        self.load_samples_into_cache(project_id, samples_cache)?;
        Ok(project)
    }

    pub fn projects(&self) -> Result<Vec<ProjectInfo>, String> {
        let mut project_infos = vec![];

        for entry in match fs::read_dir(&self.root_directory) {
            Ok(read_dir) => read_dir,
            Err(_) => return Err("Failed to read projects directory".to_string()),
        } {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => continue,
            };

            let path = entry.path();
            if !path.is_dir() {
                continue;
            }

            let directory_name = match path.file_name() {
                Some(path) => path.to_str().unwrap(),
                None => continue,
            };

            let id = match ID::parse_str(directory_name) {
                Ok(id) => id,
                Err(_) => continue,
            };

            let project = match self.read_project_json(&id) {
                Ok(project) => project,
                Err(_) => continue,
            };

            project_infos.push(project.info);
        }

        Ok(project_infos)
    }

    pub fn remove_project(&self, project_id: &ID) -> Result<(), String> {
        let directory = self.directory_for_project(project_id);
        if !directory.is_dir() {
            return Ok(());
        }

        match fs::remove_dir_all(directory) {
            Ok(_) => Ok(()),
            Err(error) => Err(error.to_string()),
        }
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
        let samples_directory = self.directory_for_samples(project_id);
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

    fn directory_for_samples(&self, project_id: &ID) -> PathBuf {
        let mut directory = self.directory_for_project(project_id);
        directory.push("samples");
        directory
    }

    fn project_json_path(&self, project_id: &ID) -> PathBuf {
        let mut json_path = self.directory_for_project(project_id);
        json_path.push("project.json");
        json_path
    }

    fn write_project_json(&self, project: Project) -> Result<(), String> {
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

    fn read_project_json(&self, project_id: &ID) -> Result<Project, String> {
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

    fn sample_path(&self, project_id: &ID, sample_id: &ID) -> PathBuf {
        let mut path = self.directory_for_samples(project_id);
        let filename = sample_id.to_string() + ".wav"; // FIXME: Use correct format
        path.push(filename);
        path
    }

    fn copy_samples_from_cache(&self, project: &Project, samples_cache: &SamplesCache) -> Result<(), String> {
        let mut errors: Vec<String> = vec![];

        for sample in project.samples.iter() {
            let project_path = self.sample_path(&project.info.id, &sample.id);

            if project_path.is_file() {
                continue;
            }

            let cached_sample = match samples_cache.get_sample(&sample.id) {
                Some(sample) => sample,
                None => {
                    errors.push(format!("Missing sample in cache: {}", sample.id));
                    continue;
                }
            };

            if *cached_sample.get_cache_state() != CacheState::Cached {
                errors.push(format!("Sample isn't cached: {}", sample.id));
                continue;
            }

            let cached_sample_path = cached_sample.get_path();
            match fs::copy(cached_sample_path, project_path) {
                Ok(_) => (),
                Err(error) => {
                    errors.push(format!("Failed to copy sample from cache into project: {}", error));
                    continue;
                }
            }
        }

        if !errors.is_empty() {
            return Err(errors.join(" "));
        }

        Ok(())
    }

    fn load_samples_into_cache(&mut self, project_id: &ID, samples_cache: &mut SamplesCache) -> Result<(), String> {
        samples_cache.clear();

        let samples_directory = self.directory_for_samples(project_id);

        for entry in match fs::read_dir(samples_directory) {
            Ok(read_dir) => read_dir,
            Err(error) => return Err(format!("Unabled to read samples directory: {}", error)),
        } {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => continue,
            };

            let project_path = entry.path();
            if !project_path.is_file() {
                continue;
            }

            let filename = project_path.file_stem().unwrap().to_str().unwrap();

            let sample_id = match ID::parse_str(filename) {
                Ok(id) => id,
                Err(error) => {
                    println!("Invalid file name ({}): {}", project_path.display(), error);
                    continue;
                }
            };

            samples_cache.add_sample_from_file(&sample_id, &AudioFileFormat::Wav, project_path.as_path())?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use crate::generators;

    use super::*;

    #[test]
    fn creates_directory() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let root_dir = temp_dir.into_path();
        let project_directory: PathBuf = [root_dir.to_str().unwrap(), "projects"].iter().collect();

        assert!(!project_directory.exists());
        ProjectStore::new(&project_directory);
        assert!(project_directory.exists());

        fs::remove_dir_all(root_dir).expect("Failed to remove directory");
    }

    #[test]
    fn save_and_load_project() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let root_dir = temp_dir.into_path();
        let project_directory: PathBuf = [root_dir.to_str().unwrap(), "projects"].iter().collect();
        let samples_directory: PathBuf = [root_dir.to_str().unwrap(), "samples"].iter().collect();
        let mut samples_cache = SamplesCache::new(&samples_directory);

        let mut project_store = ProjectStore::new(&project_directory);

        let project = generators::projects::generate_project(3, 4, 5);
        let project_id = project.info.id;
        project_store.save(project, &samples_cache).unwrap();

        let project2 = project_store.load(&project_id, &mut samples_cache).unwrap();
        assert_eq!(project2.channels.len(), 3);
        assert_eq!(project2.songs.len(), 4);
        assert_eq!(project2.sections.len(), 20);

        fs::remove_dir_all(root_dir).unwrap();
    }

    #[test]
    fn list_projects() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let root_dir = temp_dir.into_path();
        let project_directory: PathBuf = [root_dir.to_str().unwrap(), "projects"].iter().collect();
        let samples_directory: PathBuf = [root_dir.to_str().unwrap(), "samples"].iter().collect();
        let samples_cache = SamplesCache::new(&samples_directory);

        let project_store = ProjectStore::new(&project_directory);

        let project1 = generators::projects::generate_project(3, 4, 5);
        let project2 = generators::projects::generate_project(3, 4, 5);
        let project3 = generators::projects::generate_project(3, 4, 5);

        let project1_id = project1.info.id;
        let project2_id = project1.info.id;
        let project3_id = project1.info.id;

        project_store.save(project1, &samples_cache).unwrap();
        project_store.save(project2, &samples_cache).unwrap();
        project_store.save(project3, &samples_cache).unwrap();

        let projects = project_store.projects().unwrap();

        assert_eq!(projects.len(), 3, "Should be 3 projects on disk");
        assert!(
            projects.iter().any(|info| info.id == project1_id),
            "Project 1 not found"
        );
        assert!(
            projects.iter().any(|info| info.id == project2_id),
            "Project 2 not found"
        );
        assert!(
            projects.iter().any(|info| info.id == project3_id),
            "Project 3 not found"
        );

        fs::remove_dir_all(root_dir).unwrap();
    }
}
