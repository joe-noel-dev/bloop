use crate::{
    backend::Backend,
    bloop::{AudioFileFormat, ProjectRemovalTarget},
    model::{Project, ProjectInfo, ID},
    samples::SamplesCache,
};
use anyhow::{anyhow, Context};
use log::{debug, error, info};
use protobuf::Message;
use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};
use std::{str::FromStr, sync::Arc};

pub struct ProjectStore {
    root_directory: PathBuf,
    temporary_directory: tempfile::TempDir,
    backend: Arc<dyn Backend>,
    remote_backend: Arc<dyn Backend>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CloudRefresh {
    Current,
    Updated,
}

impl ProjectStore {
    pub fn new(root_directory: &Path, backend: Arc<dyn Backend>, remote_backend: Arc<dyn Backend>) -> Self {
        if !root_directory.exists() {
            fs::create_dir_all(root_directory)
                .unwrap_or_else(|_| panic!("Couldn't create directory: {}", root_directory.to_str().unwrap()));
        }
        Self::recover_interrupted_refreshes(root_directory);

        Self {
            root_directory: PathBuf::from(root_directory),
            temporary_directory: tempfile::TempDir::new().expect("Unable to create temporary directory"),
            backend,
            remote_backend,
        }
    }

    fn recover_interrupted_refreshes(root_directory: &Path) {
        let Ok(entries) = fs::read_dir(root_directory) else {
            return;
        };

        for entry in entries.flatten() {
            let filename = entry.file_name().to_string_lossy().to_string();
            if filename.starts_with(".cloud-project-") {
                let _ = fs::remove_dir_all(entry.path());
                continue;
            }

            let Some(project_id) = filename.strip_prefix('.').and_then(|name| name.strip_suffix(".backup")) else {
                continue;
            };
            let destination = root_directory.join(project_id);
            if destination.exists() {
                let _ = fs::remove_dir_all(entry.path());
            } else {
                let _ = fs::rename(entry.path(), destination);
            }
        }
    }

    pub async fn save(
        &mut self,
        project_id: Option<String>,
        project: Project,
        samples_cache: &SamplesCache,
        user_id: &str,
    ) -> anyhow::Result<String> {
        let project_id = match project_id {
            Some(id) => id,
            None => {
                let new_project = self.backend.create_project(user_id, None).await?;
                new_project.id
            }
        };

        self.copy_samples_from_cache(&project_id, &project, samples_cache)
            .await?;
        self.write_project_file(&project_id, project).await?;
        Ok(project_id)
    }

    async fn save_last_project(&self, project_id: &str) -> anyhow::Result<()> {
        let last_project_file = self.last_project_file();

        tokio::fs::write(last_project_file, project_id.to_string().as_bytes())
            .await
            .context("Error writing last project ID")
    }

    pub async fn load(
        &mut self,
        project_id: &str,
        samples_cache: &mut SamplesCache,
    ) -> anyhow::Result<(Project, ProjectInfo)> {
        let (project, project_info) = self.read_project_file(project_id).await?;
        info!("Project loaded: id = {}", project_info.id);
        self.load_samples_into_cache(project_id, &project, samples_cache)
            .await?;
        self.save_last_project(project_id).await?;
        Ok((project, project_info))
    }

    pub async fn load_last_project(
        &mut self,
        samples_cache: &mut SamplesCache,
    ) -> anyhow::Result<(ProjectInfo, Project)> {
        let last_project_file = self.last_project_file();
        let last_project_id = tokio::fs::read_to_string(last_project_file)
            .await
            .context("Opening last project file")?;
        let (project, project_info) = self.load(&last_project_id, samples_cache).await?;
        Ok((project_info, project))
    }

    fn last_project_file(&self) -> PathBuf {
        let mut last_project_file = self.root_directory.clone();
        last_project_file.push("last_project");
        last_project_file
    }

    pub async fn projects(&self) -> anyhow::Result<Vec<ProjectInfo>> {
        projects_from_backend(self.backend.clone()).await
    }

    pub async fn cloud_projects(&self) -> anyhow::Result<Vec<ProjectInfo>> {
        projects_from_backend(self.remote_backend.clone()).await
    }

    pub fn is_cloud_mirror(&self, project_id: &str) -> bool {
        self.directory_for_project(project_id).join("cloud_revision").is_file()
    }

    pub async fn refresh_from_cloud(&self, project_id: &str, user_id: &str) -> anyhow::Result<CloudRefresh> {
        let cloud_project = self
            .remote_backend
            .read_project(project_id)
            .await
            .context("Reading cloud project metadata")?;
        let cloud_revision = cloud_project.updated.to_rfc3339();

        if self.cloud_revision(project_id).await.as_deref() == Some(cloud_revision.as_str())
            && self.backend.read_project(project_id).await.is_ok()
        {
            return Ok(CloudRefresh::Current);
        }

        let project_bytes = self
            .remote_backend
            .read_project_file(project_id)
            .await
            .context("Downloading cloud project")?;
        let project = Project::parse_from_bytes(&project_bytes).context("Parsing cloud project")?;
        let referenced_samples = project
            .songs
            .iter()
            .filter_map(|song| song.sample.as_ref().map(|sample| sample.id.to_string()))
            .collect::<HashSet<_>>();
        let cloud_samples = self
            .remote_backend
            .get_samples(project_id)
            .await
            .context("Listing cloud project audio")?;

        let staging_root = tempfile::Builder::new()
            .prefix(".cloud-project-")
            .tempdir_in(&self.root_directory)
            .context("Creating cloud project staging directory")?;
        let staging_backend = crate::backend::FilesystemBackend::new(staging_root.path());
        staging_backend
            .create_project(user_id, Some(project_id.to_string()))
            .await?;
        staging_backend
            .update_project_name(project_id, &cloud_project.name)
            .await?;

        for sample_id in &referenced_samples {
            let cloud_sample = cloud_samples
                .iter()
                .find(|sample_name| *sample_name == sample_id)
                .with_context(|| format!("Cloud project is missing referenced audio: {sample_id}"))?;
            let sample_bytes = self
                .remote_backend
                .read_sample(project_id, cloud_sample)
                .await
                .with_context(|| format!("Downloading cloud project audio: {sample_id}"))?;
            staging_backend
                .add_project_sample(project_id, &sample_bytes, sample_id)
                .await?;
        }

        staging_backend.update_project_file(project_id, &project_bytes).await?;
        tokio::fs::write(
            staging_root.path().join(project_id).join("cloud_revision"),
            cloud_revision.as_bytes(),
        )
        .await
        .context("Writing cloud project revision")?;

        self.replace_local_project(project_id, staging_root.path().join(project_id))
            .await?;
        Ok(CloudRefresh::Updated)
    }

    async fn cloud_revision(&self, project_id: &str) -> Option<String> {
        tokio::fs::read_to_string(self.directory_for_project(project_id).join("cloud_revision"))
            .await
            .ok()
    }

    fn directory_for_project(&self, project_id: &str) -> PathBuf {
        self.root_directory.join(project_id)
    }

    async fn replace_local_project(&self, project_id: &str, staged_project: PathBuf) -> anyhow::Result<()> {
        let destination = self.directory_for_project(project_id);
        let backup = self.root_directory.join(format!(".{project_id}.backup"));

        if backup.exists() {
            if destination.exists() {
                tokio::fs::remove_dir_all(&backup)
                    .await
                    .context("Removing stale project backup")?;
            } else {
                tokio::fs::rename(&backup, &destination)
                    .await
                    .context("Recovering interrupted project refresh")?;
            }
        }
        if destination.exists() {
            tokio::fs::rename(&destination, &backup)
                .await
                .context("Backing up local project before refresh")?;
        }

        if let Err(error) = tokio::fs::rename(&staged_project, &destination).await {
            if backup.exists() {
                let _ = tokio::fs::rename(&backup, &destination).await;
            }
            return Err(error).context("Installing refreshed cloud project");
        }

        if backup.exists() {
            tokio::fs::remove_dir_all(&backup)
                .await
                .context("Removing replaced local project")?;
        }
        Ok(())
    }

    pub async fn remove_project(&self, project_id: &str, targets: &[ProjectRemovalTarget]) -> anyhow::Result<()> {
        let remove_local = targets.is_empty() || targets.contains(&ProjectRemovalTarget::PROJECT_REMOVAL_TARGET_LOCAL);
        let remove_remote =
            targets.is_empty() || targets.contains(&ProjectRemovalTarget::PROJECT_REMOVAL_TARGET_REMOTE);

        if !remove_local && !remove_remote {
            anyhow::bail!("No project removal targets specified");
        }

        if remove_remote {
            self.remote_backend
                .remove_project(project_id)
                .await
                .context(format!("Removing remote project: {project_id}"))?;
        }

        if remove_local {
            self.backend
                .remove_project(project_id)
                .await
                .context(format!("Removing local project: {project_id}"))?;
        }

        info!("Project removed: id = {project_id}, targets = {targets:?}");
        Ok(())
    }

    pub async fn rename_project(&self, project_id: &str, name: &str) -> anyhow::Result<()> {
        self.backend
            .update_project_name(project_id, name)
            .await
            .context(format!("Renaming project: {project_id}"))?;

        info!("Project renamed: id = {project_id}");
        Ok(())
    }

    async fn write_project_file(&mut self, project_id: &str, project: Project) -> anyhow::Result<()> {
        let data = project.write_to_bytes()?;
        self.backend
            .update_project_file(project_id, &data)
            .await
            .context("Updating project file")?;

        Ok(())
    }

    async fn read_project_file(&self, project_id: &str) -> anyhow::Result<(Project, ProjectInfo)> {
        let project = self.backend.read_project(project_id).await.context("Get project")?;

        let project_info = ProjectInfo {
            id: project.id.clone(),
            name: project.name.clone(),
            last_saved: project.updated.to_rfc3339(),
            ..Default::default()
        };

        let project_data = self
            .backend
            .read_project_file(project_id)
            .await
            .context("Get project file")?;
        let project = Project::parse_from_bytes(&project_data).context("Parse project data")?;

        Ok((project, project_info))
    }

    async fn copy_samples_from_cache(
        &self,
        project_id: &str,
        project: &Project,
        samples_cache: &SamplesCache,
    ) -> anyhow::Result<()> {
        let samples = self
            .backend
            .get_samples(project_id)
            .await
            .context("Failed to get samples from cache")?;

        for song in project.songs.iter() {
            let sample = match song.sample.as_ref() {
                Some(sample) => sample,
                None => continue,
            };

            let sample_id = samples
                .iter()
                .find(|sample_name| *sample_name == &sample.id.to_string());

            if sample_id.is_some() {
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

            debug!("Reading sample from cache: {}", cached_sample_path.display());
            let cached_sample_bytes = tokio::fs::read(&cached_sample_path)
                .await
                .context(format!("Error reading cached sample: {}", cached_sample_path.display()))?;

            debug!("Adding sample to project: {}", cached_sample_path.display());
            self.backend
                .add_project_sample(project_id, &cached_sample_bytes, &sample.id.to_string())
                .await?;
        }

        Ok(())
    }

    async fn load_samples_into_cache(
        &mut self,
        project_id: &str,
        project: &Project,
        samples_cache: &mut SamplesCache,
    ) -> anyhow::Result<()> {
        let samples = self
            .backend
            .get_samples(project_id)
            .await
            .context("Failed to get samples from cache")?;

        let referenced_sample_ids = project
            .songs
            .iter()
            .filter_map(|song| song.sample.as_ref().map(|sample| sample.id))
            .collect::<HashSet<_>>();

        for sample_id in &referenced_sample_ids {
            let sample_id_str = samples
                .iter()
                .find(|sample_name| *sample_name == &sample_id.to_string())
                .with_context(|| format!("Project is missing referenced audio: {sample_id}"))?;
            let parsed_sample_id = match ID::from_str(sample_id_str) {
                std::result::Result::Ok(id) => id,
                Err(error) => {
                    error!("Invalid sample ID ({sample_id_str}): {error}");
                    continue;
                }
            };

            if samples_cache.get_sample(parsed_sample_id).is_some() {
                debug!("Sample already in cache: {parsed_sample_id}");
                continue;
            }

            debug!("Fetching sample: {parsed_sample_id}");

            let sample_bytes = self
                .backend
                .read_sample(project_id, sample_id_str)
                .await
                .context(format!("Error getting project file: {sample_id_str}"))?;

            let sample_path = self.temporary_directory.path().join(format!("{parsed_sample_id}.wav"));

            tokio::fs::write(&sample_path, &sample_bytes)
                .await
                .context(format!("Error writing sample file: {}", sample_path.display()))?;

            debug!("Adding sample to cache: {sample_id}");

            samples_cache
                .add_sample_from_file(parsed_sample_id, AudioFileFormat::WAV, &sample_path)
                .await?;
        }

        samples_cache.retain(&referenced_sample_ids);

        Ok(())
    }
}

async fn projects_from_backend(backend: Arc<dyn Backend>) -> anyhow::Result<Vec<ProjectInfo>> {
    let projects = backend.get_projects().await?;

    let projects_info = projects
        .iter()
        .map(|db_project| ProjectInfo {
            id: db_project.id.clone(),
            name: db_project.name.clone(),
            last_saved: db_project.updated.to_rfc3339(),
            ..Default::default()
        })
        .collect();

    Ok(projects_info)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{backend::DbProject, bloop::Sample};
    use protobuf::Message;
    use std::sync::{
        atomic::{AtomicBool, Ordering},
        Mutex,
    };

    #[derive(Default)]
    struct RecordingBackend {
        removed_projects: Mutex<Vec<String>>,
        fail_remove: AtomicBool,
    }

    #[async_trait::async_trait]
    impl Backend for RecordingBackend {
        async fn get_projects(&self) -> anyhow::Result<Vec<DbProject>> {
            Ok(Vec::new())
        }

        async fn read_project(&self, _project_id: &str) -> anyhow::Result<DbProject> {
            anyhow::bail!("Unexpected read_project call")
        }

        async fn create_project(&self, _user_id: &str, _project_id: Option<String>) -> anyhow::Result<DbProject> {
            anyhow::bail!("Unexpected create_project call")
        }

        async fn update_project_name(&self, _project_id: &str, _name: &str) -> anyhow::Result<DbProject> {
            anyhow::bail!("Unexpected update_project_name call")
        }

        async fn update_project_file(&self, _project_id: &str, _project_bytes: &[u8]) -> anyhow::Result<DbProject> {
            anyhow::bail!("Unexpected update_project_file call")
        }

        async fn add_project_sample(
            &self,
            _project_id: &str,
            _sample_bytes: &[u8],
            _sample_name: &str,
        ) -> anyhow::Result<DbProject> {
            anyhow::bail!("Unexpected add_project_sample call")
        }

        async fn remove_project_sample(&self, _project_id: &str, _sample_name: &str) -> anyhow::Result<DbProject> {
            anyhow::bail!("Unexpected remove_project_sample call")
        }

        async fn remove_project(&self, project_id: &str) -> anyhow::Result<()> {
            if self.fail_remove.load(Ordering::Relaxed) {
                anyhow::bail!("Configured removal failure");
            }
            self.removed_projects.lock().unwrap().push(project_id.to_string());
            Ok(())
        }

        async fn get_samples(&self, _project_id: &str) -> anyhow::Result<Vec<String>> {
            anyhow::bail!("Unexpected get_samples call")
        }

        async fn read_sample(&self, _project_id: &str, _sample_name: &str) -> anyhow::Result<Vec<u8>> {
            anyhow::bail!("Unexpected read_sample call")
        }

        async fn read_project_file(&self, _project_id: &str) -> anyhow::Result<Vec<u8>> {
            anyhow::bail!("Unexpected read_project_file call")
        }
    }

    #[tokio::test]
    async fn remove_project_local_target_only_removes_local_project() {
        let local_backend = Arc::new(RecordingBackend::default());
        let remote_backend = Arc::new(RecordingBackend::default());
        let root_directory = tempfile::tempdir().unwrap();
        let store = ProjectStore::new(root_directory.path(), local_backend.clone(), remote_backend.clone());

        store
            .remove_project("project-id", &[ProjectRemovalTarget::PROJECT_REMOVAL_TARGET_LOCAL])
            .await
            .unwrap();

        assert_eq!(
            local_backend.removed_projects.lock().unwrap().as_slice(),
            ["project-id"]
        );
        assert!(remote_backend.removed_projects.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn remove_project_remote_target_only_removes_remote_project() {
        let local_backend = Arc::new(RecordingBackend::default());
        let remote_backend = Arc::new(RecordingBackend::default());
        let root_directory = tempfile::tempdir().unwrap();
        let store = ProjectStore::new(root_directory.path(), local_backend.clone(), remote_backend.clone());

        store
            .remove_project("project-id", &[ProjectRemovalTarget::PROJECT_REMOVAL_TARGET_REMOTE])
            .await
            .unwrap();

        assert!(local_backend.removed_projects.lock().unwrap().is_empty());
        assert_eq!(
            remote_backend.removed_projects.lock().unwrap().as_slice(),
            ["project-id"]
        );
    }

    #[tokio::test]
    async fn remove_project_local_and_remote_targets_remove_local_and_remote_projects() {
        let local_backend = Arc::new(RecordingBackend::default());
        let remote_backend = Arc::new(RecordingBackend::default());
        let root_directory = tempfile::tempdir().unwrap();
        let store = ProjectStore::new(root_directory.path(), local_backend.clone(), remote_backend.clone());

        store
            .remove_project(
                "project-id",
                &[
                    ProjectRemovalTarget::PROJECT_REMOVAL_TARGET_LOCAL,
                    ProjectRemovalTarget::PROJECT_REMOVAL_TARGET_REMOTE,
                ],
            )
            .await
            .unwrap();

        assert_eq!(
            local_backend.removed_projects.lock().unwrap().as_slice(),
            ["project-id"]
        );
        assert_eq!(
            remote_backend.removed_projects.lock().unwrap().as_slice(),
            ["project-id"]
        );
    }

    #[tokio::test]
    async fn remove_project_empty_targets_remove_local_and_remote_projects_for_compatibility() {
        let local_backend = Arc::new(RecordingBackend::default());
        let remote_backend = Arc::new(RecordingBackend::default());
        let root_directory = tempfile::tempdir().unwrap();
        let store = ProjectStore::new(root_directory.path(), local_backend.clone(), remote_backend.clone());

        store.remove_project("project-id", &[]).await.unwrap();

        assert_eq!(
            local_backend.removed_projects.lock().unwrap().as_slice(),
            ["project-id"]
        );
        assert_eq!(
            remote_backend.removed_projects.lock().unwrap().as_slice(),
            ["project-id"]
        );
    }

    #[tokio::test]
    async fn failed_remote_removal_preserves_local_project() {
        let local_backend = Arc::new(RecordingBackend::default());
        let remote_backend = Arc::new(RecordingBackend::default());
        remote_backend.fail_remove.store(true, Ordering::Relaxed);
        let root_directory = tempfile::tempdir().unwrap();
        let store = ProjectStore::new(root_directory.path(), local_backend.clone(), remote_backend);

        let result = store
            .remove_project(
                "project-id",
                &[
                    ProjectRemovalTarget::PROJECT_REMOVAL_TARGET_LOCAL,
                    ProjectRemovalTarget::PROJECT_REMOVAL_TARGET_REMOTE,
                ],
            )
            .await;

        assert!(result.is_err());
        assert!(local_backend.removed_projects.lock().unwrap().is_empty());
    }

    #[test]
    fn project_store_recovers_interrupted_refresh_backup() {
        let root_directory = tempfile::tempdir().unwrap();
        let backup = root_directory.path().join(".project-id.backup");
        fs::create_dir_all(&backup).unwrap();
        fs::write(backup.join("project.bin"), b"project").unwrap();

        let _store = ProjectStore::new(
            root_directory.path(),
            Arc::new(RecordingBackend::default()),
            Arc::new(RecordingBackend::default()),
        );

        assert!(!backup.exists());
        assert_eq!(
            fs::read(root_directory.path().join("project-id/project.bin")).unwrap(),
            b"project"
        );
    }

    fn project_bytes_with_sample(sample_id: ID) -> Vec<u8> {
        let mut project = Project::empty().with_songs(1, 1);
        project.songs[0].sample = Some(Sample {
            id: sample_id,
            ..Default::default()
        })
        .into();
        project.write_to_bytes().unwrap()
    }

    #[tokio::test]
    async fn cloud_refresh_is_transactional_and_only_copies_referenced_audio() {
        let local_root = tempfile::tempdir().unwrap();
        let remote_root = tempfile::tempdir().unwrap();
        let local_backend = Arc::new(crate::backend::FilesystemBackend::new(local_root.path()));
        let remote_backend = Arc::new(crate::backend::FilesystemBackend::new(remote_root.path()));
        remote_backend
            .create_project("user-id", Some("project-id".to_string()))
            .await
            .unwrap();
        let original_bytes = project_bytes_with_sample(42);
        remote_backend
            .add_project_sample("project-id", b"referenced", "42")
            .await
            .unwrap();
        remote_backend
            .add_project_sample("project-id", b"stale", "77")
            .await
            .unwrap();
        remote_backend
            .update_project_file("project-id", &original_bytes)
            .await
            .unwrap();
        let store = ProjectStore::new(local_root.path(), local_backend.clone(), remote_backend.clone());

        assert_eq!(
            store.refresh_from_cloud("project-id", "user-id").await.unwrap(),
            CloudRefresh::Updated
        );
        assert_eq!(local_backend.get_samples("project-id").await.unwrap(), ["42"]);

        let broken_bytes = project_bytes_with_sample(99);
        remote_backend
            .update_project_file("project-id", &broken_bytes)
            .await
            .unwrap();
        assert!(store.refresh_from_cloud("project-id", "user-id").await.is_err());
        assert_eq!(
            local_backend.read_project_file("project-id").await.unwrap(),
            original_bytes
        );
        assert_eq!(local_backend.get_samples("project-id").await.unwrap(), ["42"]);
    }

    #[tokio::test]
    async fn unchanged_cloud_revision_uses_existing_local_mirror() {
        let local_root = tempfile::tempdir().unwrap();
        let remote_root = tempfile::tempdir().unwrap();
        let local_backend = Arc::new(crate::backend::FilesystemBackend::new(local_root.path()));
        let remote_backend = Arc::new(crate::backend::FilesystemBackend::new(remote_root.path()));
        remote_backend
            .create_project("user-id", Some("project-id".to_string()))
            .await
            .unwrap();
        remote_backend
            .update_project_file("project-id", &Project::empty().write_to_bytes().unwrap())
            .await
            .unwrap();
        let store = ProjectStore::new(local_root.path(), local_backend, remote_backend);
        store.refresh_from_cloud("project-id", "user-id").await.unwrap();
        tokio::fs::remove_file(remote_root.path().join("project-id/project.bin"))
            .await
            .unwrap();

        assert_eq!(
            store.refresh_from_cloud("project-id", "user-id").await.unwrap(),
            CloudRefresh::Current
        );
    }
}
