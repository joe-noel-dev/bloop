use crate::{
    bloop::AudioFileFormat,
    control::zip::zip_directory,
    model::{Project, ProjectInfo, ID},
    samples::SamplesCache,
};
use anyhow::{anyhow, Context};
use log::{debug, error, info};
use protobuf::Message;
use std::{
    collections::{hash_map::Entry, HashMap},
    time::{SystemTime, UNIX_EPOCH},
};
use std::{convert::TryInto, str::FromStr};
use std::{
    fs,
    path::{Path, PathBuf},
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use super::zip::unzip_file;

pub struct ProjectStore {
    root_directory: PathBuf,
    temporary_directory: tempfile::TempDir,
    export_paths: HashMap<ID, tokio::fs::File>,
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
            temporary_directory: tempfile::TempDir::new().expect("Unable to create temporary directory"),
            export_paths: HashMap::new(),
        }
    }

    pub async fn save(&self, mut project: Project, samples_cache: &SamplesCache) -> anyhow::Result<()> {
        let info = project.info.as_mut().expect("Missing project info");
        info.last_saved = current_time();

        self.create_project_directory(project.info.id).await?;
        self.create_samples_directory(project.info.id).await?;
        self.copy_samples_from_cache(&project, samples_cache).await?;
        self.write_project_json(project).await?;
        Ok(())
    }

    async fn save_last_project(&self, project_id: ID) -> anyhow::Result<()> {
        let last_project_file = self.last_project_file();

        tokio::fs::write(last_project_file, project_id.to_string().as_bytes())
            .await
            .context("Error writing last project ID")
    }

    pub async fn load(&mut self, project_id: ID, samples_cache: &mut SamplesCache) -> anyhow::Result<Project> {
        let project = self.read_project_json(project_id).await?;
        self.load_samples_into_cache(project_id, samples_cache).await?;
        self.save_last_project(project_id).await?;
        Ok(project)
    }

    pub async fn load_last_project(&mut self, samples_cache: &mut SamplesCache) -> anyhow::Result<Project> {
        let last_project_file = self.last_project_file();
        let last_project_id = tokio::fs::read_to_string(last_project_file).await?;
        let last_project_id = ID::from_str(&last_project_id)?;
        self.load(last_project_id, samples_cache).await
    }

    fn last_project_file(&self) -> PathBuf {
        let mut last_project_file = self.root_directory.clone();
        last_project_file.push("last_project");
        last_project_file
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

            let id = ID::from_str(directory_name)?;

            let project = match self.read_project_json(id).await {
                Ok(project) => project,
                Err(error) => {
                    error!("Error reading project JSON: {error}");
                    continue;
                }
            };

            let project_info = project.info.as_ref().expect("Missing project info").clone();
            project_infos.push(project_info);
        }

        Ok(project_infos)
    }

    pub async fn remove_project(&self, project_id: ID) -> anyhow::Result<()> {
        let directory = self.directory_for_project(project_id);
        if !directory.is_dir() {
            return Ok(());
        }

        Ok(tokio::fs::remove_dir_all(directory).await?)
    }

    async fn create_project_directory(&self, project_id: ID) -> anyhow::Result<()> {
        let project_directory = self.directory_for_project(project_id);
        if !project_directory.exists() {
            tokio::fs::create_dir_all(project_directory)
                .await
                .with_context(|| format!("Unable to create project directory: {project_id}"))?;
        }

        Ok(())
    }

    async fn create_samples_directory(&self, project_id: ID) -> anyhow::Result<()> {
        let samples_directory = self.directory_for_samples(project_id);
        if !samples_directory.exists() {
            tokio::fs::create_dir_all(samples_directory)
                .await
                .with_context(|| format!("Unable to create samples directory: {project_id}"))?;
        }

        Ok(())
    }

    fn directory_for_project(&self, project_id: ID) -> PathBuf {
        let mut project_directory = self.root_directory.clone();
        project_directory.push(project_id.to_string());
        project_directory
    }

    fn directory_for_samples(&self, project_id: ID) -> PathBuf {
        let mut directory = self.directory_for_project(project_id);
        directory.push("samples");
        directory
    }

    fn project_json_path(&self, project_id: ID) -> PathBuf {
        let mut json_path = self.directory_for_project(project_id);
        json_path.push("project.bin");
        json_path
    }

    async fn write_project_json(&self, project: Project) -> anyhow::Result<()> {
        let json_path = self.project_json_path(project.info.id);

        let mut file = tokio::fs::File::create(json_path)
            .await
            .context("Failed to open project file for writing")?;

        let data = project.write_to_bytes()?;

        file.write(&data).await.context("Failed to write project")?;
        Ok(())
    }

    async fn read_project_json(&self, project_id: ID) -> anyhow::Result<Project> {
        let json_path = self.project_json_path(project_id);

        let data = tokio::fs::read(json_path)
            .await
            .with_context(|| format!("Failed to read project with ID: {project_id}"))?;

        let project = Project::parse_from_bytes(&data).context("Parsing project data")?;
        Ok(project)
    }

    fn sample_path(&self, project_id: ID, sample_id: ID) -> PathBuf {
        let mut path = self.directory_for_samples(project_id);
        let filename = sample_id.to_string() + ".wav"; // FIXME: Use correct format
        path.push(filename);
        path
    }

    async fn copy_samples_from_cache(&self, project: &Project, samples_cache: &SamplesCache) -> anyhow::Result<()> {
        let mut futures = vec![];

        for song in project.songs.iter() {
            let sample = match song.sample.as_ref() {
                Some(sample) => sample,
                None => continue,
            };

            let project_path = self.sample_path(project.info.id, sample.id);

            if project_path.is_file() {
                continue;
            }

            let cached_sample = match samples_cache.get_sample(sample.id) {
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
        project_id: ID,
        samples_cache: &mut SamplesCache,
    ) -> anyhow::Result<()> {
        samples_cache.clear();

        let samples_directory = self.directory_for_samples(project_id);

        if !samples_directory.is_dir() {
            tokio::fs::create_dir_all(&samples_directory)
                .await
                .with_context(|| format!("Error creating samples directory: {project_id}"))?;
        }

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

            let sample_id = match ID::from_str(filename) {
                Ok(id) => id,
                Err(error) => {
                    error!("Invalid file name ({}): {}", project_path.display(), error);
                    continue;
                }
            };

            samples_cache
                .add_sample_from_file(sample_id, AudioFileFormat::WAV, project_path.as_path())
                .await?;
        }

        Ok(())
    }

    pub async fn import(&mut self, project_id: ID, data: &[u8], more_coming: bool) -> anyhow::Result<()> {
        info!("Importing project_id={}", project_id);

        let import_file_path = self.temporary_directory.path().join(project_id.to_string() + ".bloop");
        if let Entry::Vacant(e) = self.export_paths.entry(project_id) {
            let file = tokio::fs::File::create(&import_file_path)
                .await
                .with_context(|| format!("Error creating import file: {}", import_file_path.display()))?;

            e.insert(file);
        }

        let file = self.export_paths.get_mut(&project_id).unwrap();

        file.write_all(data).await?;

        if !more_coming {
            let out_dir = self.directory_for_project(project_id);

            info!(
                "Finished importing project_id={} path={} project_dir={}",
                project_id,
                import_file_path.display(),
                out_dir.display()
            );

            tokio::fs::create_dir(&out_dir).await?;
            unzip_file(&import_file_path, &out_dir).await?;
            self.export_paths.remove(&project_id);
        }

        Ok(())
    }

    pub async fn export(&mut self, project_id: ID) -> anyhow::Result<(Vec<u8>, bool)> {
        let export_file = self.temporary_directory.path().join(project_id.to_string() + ".bloop");

        let project_dir = self.directory_for_project(project_id);

        if let Entry::Vacant(e) = self.export_paths.entry(project_id) {
            info!("Starting export project_id={}", project_id);

            if !project_dir.is_dir() {
                return Err(anyhow!("Project directory doesn't exist: {}", project_id));
            }

            zip_directory(&project_dir, &export_file).await?;

            let file = tokio::fs::File::open(&export_file)
                .await
                .with_context(|| format!("Error opening export file: {}", export_file.display()))?;

            e.insert(file);
        }

        let file = self
            .export_paths
            .get_mut(&project_id)
            .unwrap_or_else(|| panic!("Missing export path project_id={}", project_id));

        debug!("Exporting project_id={}", project_id);

        let chunk_size = 10 * 1024;

        let mut buffer = vec![0; chunk_size];
        let count = file.read(&mut buffer).await?;
        let more_coming = count == chunk_size;

        if !more_coming {
            info!(
                "Finished export project_id={} export_path={}",
                project_id,
                export_file.display()
            );
            self.export_paths.remove(&project_id);
        }

        Ok((buffer, more_coming))
    }
}

#[cfg(test)]
mod tests {

    use crate::{generators::generate_project, model::random_id};

    use super::*;

    struct Fixture {
        temp_dir: tempfile::TempDir,
        project_store: ProjectStore,
        samples_cache: SamplesCache,
    }

    impl Fixture {
        pub fn new() -> Self {
            let temp_dir = tempfile::TempDir::new().expect("Unable to create temporary directory");
            let root_dir = temp_dir.path();
            let project_directory = root_dir.join("projects");
            let samples_directory = root_dir.join("samples");

            Self {
                temp_dir,
                project_store: ProjectStore::new(&project_directory),
                samples_cache: SamplesCache::new(&samples_directory),
            }
        }

        pub fn root_dir(&self) -> PathBuf {
            self.temp_dir.path().to_path_buf()
        }

        pub async fn save(&self, project: Project) {
            self.project_store
                .save(project, &self.samples_cache)
                .await
                .expect("Unable to save project");
        }

        pub async fn load(&mut self, project_id: ID) -> Project {
            self.project_store
                .load(project_id, &mut self.samples_cache)
                .await
                .expect("Unable to load project")
        }

        pub async fn list(&self) -> Vec<ProjectInfo> {
            self.project_store.projects().await.expect("Unable to list projects")
        }

        pub async fn remove(&self, project_id: ID) {
            self.project_store
                .remove_project(project_id)
                .await
                .expect("Unable to remove project")
        }

        pub async fn export(&mut self, project_id: ID) -> Vec<u8> {
            let mut data = Vec::new();

            loop {
                let (chunk, more_coming) = self
                    .project_store
                    .export(project_id)
                    .await
                    .expect("Unable to export project");

                data.extend(chunk);

                if !more_coming {
                    break;
                }
            }

            data
        }

        pub async fn import(&mut self, project_id: ID, data: &[u8]) {
            let mut offset = 0;
            let mut more_coming = true;

            while more_coming {
                let chunk_size = 1024.min(data.len() - offset);
                let chunk = &data[offset..offset + chunk_size];
                more_coming = offset + chunk_size < data.len();

                self.project_store
                    .import(project_id, chunk, more_coming)
                    .await
                    .expect("Unable to import project");

                offset += chunk_size;
            }
        }
    }

    #[test]
    fn creates_directory() {
        let fixture = Fixture::new();
        let project_dir = fixture.root_dir().join("projects");
        assert!(project_dir.exists());
    }

    #[tokio::test]
    async fn save_and_load_project() {
        let mut fixture = Fixture::new();

        let song_count = 4;
        let section_count = 5;

        let project = generate_project(song_count, section_count);
        let project_id = project.info.id;
        fixture.save(project).await;

        let project2 = fixture.load(project_id).await;
        assert_eq!(project2.songs.len(), song_count);
        assert!(project2.songs.iter().all(|song| song.sections.len() == section_count));
    }

    #[tokio::test]
    async fn export_and_import_project() {
        let mut fixture = Fixture::new();
        let project = generate_project(4, 5);
        let project_id = project.info.id;
        fixture.save(project).await;
        let exported = fixture.export(project_id).await;
        fixture.remove(project_id).await;

        let new_project_id = random_id();
        fixture.import(new_project_id, &exported).await;
        let project2 = fixture.load(new_project_id).await;
        assert_eq!(project2.songs.len(), 4);
        assert!(project2.songs.iter().all(|song| song.sections.len() == 5));
    }

    #[tokio::test]
    async fn list_projects() {
        let fixture = Fixture::new();

        let project1 = generate_project(4, 5);
        let project2 = generate_project(4, 5);
        let project3 = generate_project(4, 5);

        let project1_id = project1.info.id;
        let project2_id = project1.info.id;
        let project3_id = project1.info.id;

        fixture.save(project1).await;
        fixture.save(project2).await;
        fixture.save(project3).await;

        let projects = fixture.list().await;

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
    }
}
