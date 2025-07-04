use std::path::{Path, PathBuf};

use crate::backend::{Backend, DbProject};
use anyhow::{Context, Result};
use rand::Rng;

/**
 * FilesystemBackend is a backend implementation that interacts with the filesystem
 * to manage projects and their associated files.
 *
 * The directory structure is:
 * - projects/{project_id}/
 *   - project.json (the project metadata file)
 *   - project.bin (the binary project file)
 *   - samples/
 *     - sample_1.wav    
 *     - ...
 *
 */
pub struct FilesystemBackend {
    root_directory: PathBuf,
}

impl FilesystemBackend {
    pub fn new(root_directory: &Path) -> Self {
        Self {
            root_directory: PathBuf::from(root_directory),
        }
    }

    fn directory_for_project(&self, project_id: &str) -> PathBuf {
        self.root_directory.join(project_id)
    }

    fn get_metadata_file(&self, project_id: &str) -> PathBuf {
        self.directory_for_project(project_id).join("project.json")
    }

    fn generate_project_id() -> String {
        const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
        const ID_LENGTH: usize = 15;

        let mut rng = rand::rng();
        (0..ID_LENGTH)
            .map(|_| {
                let idx = rng.random_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }

    async fn write_metadata(&self, project_id: &str, db_project: &DbProject) -> Result<()> {
        let metadata_file_path = self.get_metadata_file(project_id);
        let metadata_bytes = serde_json::to_vec(db_project).context("Failed to serialize project metadata")?;
        tokio::fs::write(&metadata_file_path, metadata_bytes)
            .await
            .context(format!("Failed to write project metadata file for {}", project_id))?;
        Ok(())
    }

    async fn read_metadata(&self, project_id: &str) -> Result<DbProject> {
        let metadata_file_path = self.get_metadata_file(project_id);
        let metadata_bytes = tokio::fs::read(&metadata_file_path)
            .await
            .context(format!("Failed to read project metadata for {}", project_id))?;
        let db_project: DbProject =
            serde_json::from_slice(&metadata_bytes).context("Failed to deserialize project metadata")?;
        Ok(db_project)
    }
}

#[async_trait::async_trait]
impl Backend for FilesystemBackend {
    async fn get_projects(&self) -> Result<Vec<DbProject>> {
        // Read the projects directory
        if !self.root_directory.exists() {
            return Ok(Vec::new());
        }

        let mut projects = Vec::new();

        let mut entries = tokio::fs::read_dir(&self.root_directory)
            .await
            .context("Failed to read projects directory")?;

        while let Some(entry) = entries.next_entry().await.context("Failed to read project entry")? {
            if entry.file_type().await?.is_dir() {
                let project_id = entry.file_name().to_string_lossy().to_string();
                let project = self.read_project(&project_id).await?;
                projects.push(project);
            }
        }

        Ok(projects)
    }

    async fn read_project(&self, project_id: &str) -> Result<DbProject> {
        let project_dir = self.directory_for_project(project_id);

        // Check if the project directory exists
        if !project_dir.exists() {
            return Err(anyhow::anyhow!("Project {} does not exist", project_id));
        }

        // Read the project metadata file
        let db_project = self.read_metadata(project_id).await?;
        Ok(db_project)
    }

    async fn create_project(&self, user_id: &str) -> Result<DbProject> {
        let project_id = Self::generate_project_id();

        let db_project = DbProject {
            id: project_id.clone(),
            name: "New Project".to_string(),
            user_id: user_id.to_string(),
            created: chrono::Utc::now(),
            updated: chrono::Utc::now(),
        };

        let project_dir = self.directory_for_project(&project_id);

        // Create the project directory
        tokio::fs::create_dir_all(&project_dir).await.context(format!(
            "Failed to create project directory for user {} and project {}",
            user_id, project_id
        ))?;

        // Write the project metadata file
        self.write_metadata(&project_id, &db_project).await?;

        Ok(db_project)
    }

    async fn update_project_name(&self, project_id: &str, name: &str) -> Result<DbProject> {
        let project_dir = self.directory_for_project(project_id);

        // Check if the project directory exists
        if !project_dir.exists() {
            return Err(anyhow::anyhow!("Project {} does not exist", project_id));
        }

        // Read the current project metadata
        let mut db_project = self.read_project(project_id).await?;

        // Update the name and updated timestamp
        db_project.name = name.to_string();
        db_project.updated = chrono::Utc::now();

        // Write the updated metadata back to the file
        self.write_metadata(project_id, &db_project).await?;

        Ok(db_project)
    }

    async fn update_project_file(&self, project_id: &str, project_bytes: &[u8]) -> Result<DbProject> {
        let project_dir = self.directory_for_project(project_id);

        // Check if the project directory exists
        if !project_dir.exists() {
            return Err(anyhow::anyhow!("Project {} does not exist", project_id));
        }

        // Read the current project metadata
        let mut db_project = self.read_metadata(project_id).await?;

        // Write the project bytes to project.bin
        let project_file_path = project_dir.join("project.bin");
        tokio::fs::write(&project_file_path, project_bytes)
            .await
            .context(format!("Failed to write project file for {}", project_id))?;

        // Update the project field and updated timestamp
        db_project.updated = chrono::Utc::now();

        // Write the updated metadata back to the file
        self.write_metadata(project_id, &db_project).await?;

        Ok(db_project)
    }

    async fn add_project_sample(&self, project_id: &str, sample_bytes: &[u8], sample_name: &str) -> Result<DbProject> {
        // Get the project directory and check if it exists
        let project_dir = self.directory_for_project(project_id);
        if !project_dir.exists() {
            return Err(anyhow::anyhow!("Project {} does not exist", project_id));
        }

        // Read the current project metadata
        let mut project = self.read_metadata(project_id).await?;

        // Create samples directory if it doesn't exist
        let samples_dir = project_dir.join("samples");
        if !samples_dir.exists() {
            tokio::fs::create_dir_all(&samples_dir)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to create samples directory: {}", e))?;
        }

        // Write the sample file
        let sample_file_path = samples_dir.join(format!("{}.wav", sample_name));
        tokio::fs::write(&sample_file_path, sample_bytes)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to write sample file: {}", e))?;

        // Update timestamp
        project.updated = chrono::Utc::now();

        // Write updated metadata
        self.write_metadata(project_id, &project).await?;

        Ok(project)
    }

    async fn remove_project_sample(&self, project_id: &str, sample_name: &str) -> Result<DbProject> {
        // Get the project directory and check if it exists
        let project_dir = self.directory_for_project(project_id);
        if !project_dir.exists() {
            return Err(anyhow::anyhow!("Project {} does not exist", project_id));
        }

        // Read the current project metadata
        let mut project = self.read_metadata(project_id).await?;

        // Remove the sample file from the filesystem
        let sample_file_path = project_dir.join("samples").join(format!("{}.wav", sample_name));
        if !sample_file_path.exists() {
            return Err(anyhow::anyhow!("Sample {} does not exist", sample_name));
        }

        tokio::fs::remove_file(&sample_file_path)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to remove sample file: {}", e))?;

        // Update timestamp
        project.updated = chrono::Utc::now();

        // Write updated metadata
        self.write_metadata(project_id, &project).await?;

        Ok(project)
    }

    async fn remove_project(&self, project_id: &str) -> Result<()> {
        let project_dir = self.directory_for_project(project_id);

        // Only attempt to remove if the directory exists
        // Not existing is not considered an error
        if project_dir.exists() {
            tokio::fs::remove_dir_all(&project_dir)
                .await
                .context(format!("Failed to remove project directory for {}", project_id))?;
        }

        Ok(())
    }

    async fn get_samples(&self, project_id: &str) -> Result<Vec<String>> {
        let project_dir = self.directory_for_project(project_id);

        // Check if project directory exists
        if !project_dir.exists() {
            return Err(anyhow::anyhow!("Project {} does not exist", project_id));
        }

        let samples_dir = project_dir.join("samples");

        // If samples directory doesn't exist, return empty vector
        if !samples_dir.exists() {
            return Ok(Vec::new());
        }

        let mut sample_names = Vec::new();
        let mut entries = tokio::fs::read_dir(&samples_dir)
            .await
            .context(format!("Failed to read samples directory for project {}", project_id))?;

        while let Some(entry) = entries.next_entry().await.context("Failed to read sample entry")? {
            if entry.file_type().await?.is_file() {
                let file_name = entry.file_name().to_string_lossy().to_string();

                // Extract sample name by removing .wav extension
                if file_name.ends_with(".wav") {
                    let sample_name = file_name.trim_end_matches(".wav").to_string();
                    sample_names.push(sample_name);
                }
            }
        }

        // Sort the sample names for consistent ordering
        sample_names.sort();

        Ok(sample_names)
    }

    async fn read_sample(&self, project_id: &str, sample_name: &str) -> Result<Vec<u8>> {
        let project_dir = self.directory_for_project(project_id);

        // Check if project directory exists
        if !project_dir.exists() {
            return Err(anyhow::anyhow!("Project {} does not exist", project_id));
        }

        // Validate sample_name to prevent path traversal
        if sample_name.contains("..")
            || sample_name.contains("/")
            || sample_name.contains("\\")
            || sample_name.starts_with(".")
        {
            return Err(anyhow::anyhow!("Invalid sample name: '{}'", sample_name));
        }

        let sample_file_path = project_dir.join("samples").join(format!("{}.wav", sample_name));

        // Check if sample file exists
        if !sample_file_path.exists() {
            return Err(anyhow::anyhow!("Sample {} does not exist", sample_name));
        }

        // Read and return sample contents
        tokio::fs::read(&sample_file_path)
            .await
            .context(format!("Failed to read sample file for sample '{}'", sample_name))
    }

    async fn read_project_file(&self, project_id: &str) -> Result<Vec<u8>> {
        let project_dir = self.directory_for_project(project_id);

        // Check if project directory exists
        if !project_dir.exists() {
            return Err(anyhow::anyhow!("Project {} does not exist", project_id));
        }

        let project_file_path = project_dir.join("project.bin");

        // Check if project file exists
        if !project_file_path.exists() {
            return Err(anyhow::anyhow!(
                "Project file does not exist for project {}",
                project_id
            ));
        }

        // Read and return project file contents
        tokio::fs::read(&project_file_path)
            .await
            .context(format!("Failed to read project file for project '{}'", project_id))
    }
}
