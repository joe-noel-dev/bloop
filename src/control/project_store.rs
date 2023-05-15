use crate::{
    model::{Project, ProjectInfo, ID},
    samples::SamplesCache,
    types::AudioFileFormat,
};
use anyhow::{anyhow, Context};
use std::convert::TryInto;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{
    fs,
    path::{Path, PathBuf},
};
use tokio::io::AsyncWriteExt;

pub struct ProjectStore {
    root_directory: PathBuf,
}

fn current_time() -> i64 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    since_the_epoch.as_millis().try_into().unwrap()
}

impl ProjectStore {
    pub fn new(root_directory: &Path) -> Self {
        if !root_directory.exists() {
            fs::create_dir_all(root_directory)
                .unwrap_or_else(|_| panic!("Couldn't create directory: {}", root_directory.to_str().unwrap()));
        }

        Self {
            root_directory: PathBuf::from(root_directory),
        }
    }

    pub async fn save(&self, mut project: Project, samples_cache: &SamplesCache) -> anyhow::Result<()> {
        project.info.last_saved = current_time();
        self.create_project_directory(&project.info.id).await?;
        self.create_samples_directory(&project.info.id).await?;
        self.copy_samples_from_cache(&project, samples_cache).await?;
        self.write_project_json(project).await?;
        Ok(())
    }

    pub async fn load(&mut self, project_id: &ID, samples_cache: &mut SamplesCache) -> anyhow::Result<Project> {
        let project = self.read_project_json(project_id).await?;
        self.load_samples_into_cache(project_id, samples_cache).await?;
        Ok(project)
    }

    pub async fn projects(&self) -> anyhow::Result<Vec<ProjectInfo>> {
        let mut project_infos = vec![];
        let mut read_dir = tokio::fs::read_dir(&self.root_directory)
            .await
            .context("Error reading projects directory")?;

        while let Some(entry) = read_dir
            .next_entry()
            .await
            .context("Error iterating through projects directory")?
        {
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

            let project = match self.read_project_json(&id).await {
                Ok(project) => project,
                Err(error) => {
                    println!("Error reading project JSON: {error}");
                    continue;
                }
            };

            project_infos.push(project.info);
        }

        Ok(project_infos)
    }

    pub async fn remove_project(&self, project_id: &ID) -> anyhow::Result<()> {
        let directory = self.directory_for_project(project_id);
        if !directory.is_dir() {
            return Ok(());
        }

        Ok(tokio::fs::remove_dir_all(directory).await?)
    }

    async fn create_project_directory(&self, project_id: &ID) -> anyhow::Result<()> {
        let project_directory = self.directory_for_project(project_id);
        if !project_directory.exists() {
            tokio::fs::create_dir_all(project_directory)
                .await
                .with_context(|| format!("Unable to create project directory: {project_id}"))?;
        }

        Ok(())
    }

    async fn create_samples_directory(&self, project_id: &ID) -> anyhow::Result<()> {
        let samples_directory = self.directory_for_samples(project_id);
        if !samples_directory.exists() {
            tokio::fs::create_dir_all(samples_directory)
                .await
                .with_context(|| format!("Unable to create samples directory: {project_id}"))?;
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

    async fn write_project_json(&self, project: Project) -> anyhow::Result<()> {
        let json_path = self.project_json_path(&project.info.id);

        let mut file = tokio::fs::File::create(json_path)
            .await
            .context("Failed to open project JSON file for writing")?;

        let json = serde_json::to_string(&project).context("Failed to convert project to JSON")?;

        file.write(json.as_bytes())
            .await
            .context("Failed to write project JSON")?;
        Ok(())
    }

    async fn read_project_json(&self, project_id: &ID) -> anyhow::Result<Project> {
        let json_path = self.project_json_path(project_id);

        let data = tokio::fs::read_to_string(json_path)
            .await
            .with_context(|| format!("Failed to read project with ID: {project_id}"))?;

        serde_json::from_str::<Project>(&data).with_context(|| format!("Failed to parse project JSON: {project_id}"))
    }

    fn sample_path(&self, project_id: &ID, sample_id: &ID) -> PathBuf {
        let mut path = self.directory_for_samples(project_id);
        let filename = sample_id.to_string() + ".wav"; // FIXME: Use correct format
        path.push(filename);
        path
    }

    async fn copy_samples_from_cache(&self, project: &Project, samples_cache: &SamplesCache) -> anyhow::Result<()> {
        let mut futures = vec![];

        for sample in project.samples.iter() {
            let project_path = self.sample_path(&project.info.id, &sample.id);

            if project_path.is_file() {
                continue;
            }

            let cached_sample = match samples_cache.get_sample(&sample.id) {
                Some(sample) => sample,
                None => {
                    return Err(anyhow!("Missing sample in cache: {}", sample.id));
                }
            };

            if !cached_sample.is_cached() {
                return Err(anyhow!("Sample isn't cached: {}", sample.id));
            }

            let cached_sample_path = cached_sample.get_path();
            let future = tokio::fs::copy(cached_sample_path, project_path);
            futures.push(future);
        }

        for future in futures {
            future.await?;
        }

        Ok(())
    }

    async fn load_samples_into_cache(
        &mut self,
        project_id: &ID,
        samples_cache: &mut SamplesCache,
    ) -> anyhow::Result<()> {
        samples_cache.clear();

        let samples_directory = self.directory_for_samples(project_id);

        let mut read_dir = tokio::fs::read_dir(samples_directory)
            .await
            .with_context(|| format!("Error reading samples directory: {project_id}"))?;

        while let Some(entry) = read_dir
            .next_entry()
            .await
            .with_context(|| format!("Error iterating samples directory: {project_id}"))?
        {
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

            samples_cache
                .add_sample_from_file(&sample_id, &AudioFileFormat::Wav, project_path.as_path())
                .await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use crate::generators::generate_project;

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

    #[tokio::test]
    async fn save_and_load_project() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let root_dir = temp_dir.into_path();
        let project_directory: PathBuf = [root_dir.to_str().unwrap(), "projects"].iter().collect();
        let samples_directory: PathBuf = [root_dir.to_str().unwrap(), "samples"].iter().collect();
        let mut samples_cache = SamplesCache::new(&samples_directory);

        let mut project_store = ProjectStore::new(&project_directory);

        let project = generate_project(3, 4, 5);
        let project_id = project.info.id;
        project_store.save(project, &samples_cache).await.unwrap();

        let project2 = project_store.load(&project_id, &mut samples_cache).await.unwrap();
        assert_eq!(project2.channels.len(), 3);
        assert_eq!(project2.songs.len(), 4);
        assert!(project2.songs.iter().all(|song| song.sections.len() == 5));

        fs::remove_dir_all(root_dir).unwrap();
    }

    #[tokio::test]
    async fn list_projects() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let root_dir = temp_dir.into_path();
        let project_directory: PathBuf = [root_dir.to_str().unwrap(), "projects"].iter().collect();
        let samples_directory: PathBuf = [root_dir.to_str().unwrap(), "samples"].iter().collect();
        let samples_cache = SamplesCache::new(&samples_directory);

        let project_store = ProjectStore::new(&project_directory);

        let project1 = generate_project(3, 4, 5);
        let project2 = generate_project(3, 4, 5);
        let project3 = generate_project(3, 4, 5);

        let project1_id = project1.info.id;
        let project2_id = project1.info.id;
        let project3_id = project1.info.id;

        project_store.save(project1, &samples_cache).await.unwrap();
        project_store.save(project2, &samples_cache).await.unwrap();
        project_store.save(project3, &samples_cache).await.unwrap();

        let projects = project_store.projects().await.unwrap();

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
