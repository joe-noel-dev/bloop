use std::path::PathBuf;

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
struct FilesystemBackend {
    root_directory: PathBuf,
}

impl FilesystemBackend {
    pub fn new(root_directory: PathBuf) -> Self {
        Self { root_directory }
    }

    fn directory_for_project(&self, project_id: &str) -> PathBuf {
        self.root_directory.join("projects").join(project_id)
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
        let mut projects = Vec::new();

        // Read the projects directory
        let projects_dir = self.root_directory.join("projects");
        if !projects_dir.exists() {
            return Ok(projects);
        }

        let mut entries = tokio::fs::read_dir(&projects_dir)
            .await
            .context("Failed to read projects directory")?;

        while let Some(entry) = entries.next_entry().await.context("Failed to read project entry")? {
            if entry.file_type().await?.is_dir() {
                let project_id = entry.file_name().to_string_lossy().to_string();
                let project = self.get_project(&project_id).await?;
                projects.push(project);
            }
        }

        Ok(projects)
    }

    async fn get_project(&self, project_id: &str) -> Result<DbProject> {
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
            project: "".to_string(),
            samples: Vec::new(),
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
        let mut db_project = self.get_project(project_id).await?;

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
        db_project.project = "project.bin".to_string();
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

        // Add sample path to metadata (avoid duplicates)
        let sample_path = format!("samples/{}.wav", sample_name);
        if !project.samples.contains(&sample_path) {
            project.samples.push(sample_path);
        }

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

        // Check if the sample exists in the metadata
        let sample_path = format!("samples/{}.wav", sample_name);
        if !project.samples.contains(&sample_path) {
            return Err(anyhow::anyhow!("Sample {} does not exist", sample_name));
        }

        // Remove the sample file from the filesystem
        let sample_file_path = project_dir.join("samples").join(format!("{}.wav", sample_name));
        if sample_file_path.exists() {
            tokio::fs::remove_file(&sample_file_path)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to remove sample file: {}", e))?;
        }

        // Remove the sample path from the metadata
        project.samples.retain(|sample| sample != &sample_path);

        // Update timestamp
        project.updated = chrono::Utc::now();

        // Write updated metadata
        self.write_metadata(project_id, &project).await?;

        Ok(project)
    }

    async fn remove_project(&self, project_id: &str) -> Result<()> {
        // Implementation for removing a project from the filesystem
        unimplemented!()
    }

    async fn get_project_file(&self, project_id: &str, project_filename: &str) -> Result<Vec<u8>> {
        // Implementation for retrieving a project's file from the filesystem
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use tokio;

    struct Fixture {
        temp_dir: TempDir,
        backend: FilesystemBackend,
    }

    impl Fixture {
        fn new() -> Self {
            let temp_dir = TempDir::new().expect("Failed to create temp directory");
            let backend = FilesystemBackend::new(temp_dir.path().to_path_buf());
            Self { temp_dir, backend }
        }
    }

    #[tokio::test]
    async fn test_create_project() {
        let fixture = Fixture::new();
        let user_id = "test_user";

        // Test creating a project
        let result = fixture.backend.create_project(user_id).await;
        assert!(result.is_ok(), "Failed to create project: {:?}", result.err());

        let db_project = result.unwrap();

        // Verify the project properties
        assert_eq!(db_project.name, "New Project");
        assert_eq!(db_project.user_id, user_id);
        assert_eq!(db_project.project, "");
        assert!(db_project.samples.is_empty());
        assert_eq!(db_project.id.len(), 15); // ID should be 15 characters

        // Verify ID contains only valid characters (a-z, 0-9)
        for ch in db_project.id.chars() {
            assert!(
                ch.is_ascii_lowercase() || ch.is_ascii_digit(),
                "Invalid character '{}' in project ID",
                ch
            );
        }

        // Verify the directory structure was created
        let project_dir = fixture.backend.directory_for_project(&db_project.id);
        assert!(project_dir.exists(), "Project directory was not created");

        // Verify the project.bin file was NOT created (since we don't create it anymore)
        let project_file = project_dir.join("project.bin");
        assert!(!project_file.exists(), "Project bin file should not be created");

        // Verify the project.json metadata file was created
        let metadata_file = fixture.backend.get_metadata_file(&db_project.id);
        assert!(metadata_file.exists(), "Project metadata file was not created");

        // Read and verify the metadata file contains correct data
        let metadata_content = tokio::fs::read(&metadata_file)
            .await
            .expect("Failed to read metadata file");
        let loaded_project: DbProject =
            serde_json::from_slice(&metadata_content).expect("Failed to parse metadata file");

        assert_eq!(loaded_project.id, db_project.id);
        assert_eq!(loaded_project.name, db_project.name);
        assert_eq!(loaded_project.user_id, db_project.user_id);
    }

    #[tokio::test]
    async fn test_get_projects_empty_directory() {
        let fixture = Fixture::new();

        // Test getting projects when no projects exist
        let result = fixture.backend.get_projects().await;
        assert!(result.is_ok(), "Failed to get projects: {:?}", result.err());

        let projects = result.unwrap();
        assert!(projects.is_empty(), "Should return empty vector when no projects exist");
    }

    #[tokio::test]
    async fn test_get_projects_single_project() {
        let fixture = Fixture::new();
        let user_id = "test_user";

        // Create a project first
        let created_project = fixture
            .backend
            .create_project(user_id)
            .await
            .expect("Failed to create project");

        // Test getting all projects
        let result = fixture.backend.get_projects().await;
        assert!(result.is_ok(), "Failed to get projects: {:?}", result.err());

        let projects = result.unwrap();
        assert_eq!(projects.len(), 1, "Should return one project");

        let project = &projects[0];
        assert_eq!(project.id, created_project.id);
        assert_eq!(project.name, created_project.name);
        assert_eq!(project.user_id, created_project.user_id);
        assert_eq!(project.project, created_project.project);
    }

    #[tokio::test]
    async fn test_get_projects_multiple_projects() {
        let fixture = Fixture::new();

        // Create multiple projects
        let project1 = fixture
            .backend
            .create_project("user1")
            .await
            .expect("Failed to create project 1");
        let project2 = fixture
            .backend
            .create_project("user2")
            .await
            .expect("Failed to create project 2");
        let project3 = fixture
            .backend
            .create_project("")
            .await // anonymous user
            .expect("Failed to create project 3");

        // Test getting all projects
        let result = fixture.backend.get_projects().await;
        assert!(result.is_ok(), "Failed to get projects: {:?}", result.err());

        let projects = result.unwrap();
        assert_eq!(projects.len(), 3, "Should return three projects");

        // Verify all projects are returned (order may vary)
        let project_ids: Vec<String> = projects.iter().map(|p| p.id.clone()).collect();
        assert!(project_ids.contains(&project1.id), "Should contain project 1");
        assert!(project_ids.contains(&project2.id), "Should contain project 2");
        assert!(project_ids.contains(&project3.id), "Should contain project 3");

        // Verify project details for each
        for project in &projects {
            match project.id.as_str() {
                id if id == project1.id => {
                    assert_eq!(project.user_id, "user1");
                    assert_eq!(project.name, "New Project");
                }
                id if id == project2.id => {
                    assert_eq!(project.user_id, "user2");
                    assert_eq!(project.name, "New Project");
                }
                id if id == project3.id => {
                    assert_eq!(project.user_id, "");
                    assert_eq!(project.name, "New Project");
                }
                _ => panic!("Unexpected project ID: {}", project.id),
            }
        }
    }

    #[tokio::test]
    async fn test_get_projects_with_invalid_directory() {
        let fixture = Fixture::new();

        // Create a project first
        let _project = fixture
            .backend
            .create_project("test_user")
            .await
            .expect("Failed to create project");

        // Create an invalid entry (file instead of directory) in the projects directory
        let projects_dir = fixture.temp_dir.path().join("projects");
        let invalid_file = projects_dir.join("not_a_directory.txt");
        tokio::fs::write(&invalid_file, "this is not a project directory")
            .await
            .expect("Failed to create invalid file");

        // Test getting projects - should ignore the invalid file and return only valid projects
        let result = fixture.backend.get_projects().await;
        assert!(result.is_ok(), "Failed to get projects: {:?}", result.err());

        let projects = result.unwrap();
        assert_eq!(projects.len(), 1, "Should return only valid project directories");
    }

    #[tokio::test]
    async fn test_get_project() {
        let fixture = Fixture::new();
        let user_id = "test_user";

        // Create a project first
        let created_project = fixture
            .backend
            .create_project(user_id)
            .await
            .expect("Failed to create project");

        // Test getting the specific project
        let result = fixture.backend.get_project(&created_project.id).await;
        assert!(result.is_ok(), "Failed to get project: {:?}", result.err());

        let project = result.unwrap();
        assert_eq!(project.id, created_project.id);
        assert_eq!(project.name, created_project.name);
        assert_eq!(project.user_id, created_project.user_id);
        assert_eq!(project.project, created_project.project);
        assert_eq!(project.samples, created_project.samples);
    }

    #[tokio::test]
    async fn test_get_project_not_found() {
        let fixture = Fixture::new();

        // Test getting a project that doesn't exist
        let result = fixture.backend.get_project("nonexistent_project_id").await;
        assert!(result.is_err(), "Should fail when project doesn't exist");

        let error = result.unwrap_err();
        assert!(error
            .to_string()
            .contains("Project nonexistent_project_id does not exist"));
    }

    #[tokio::test]
    async fn test_update_project_name() {
        let fixture = Fixture::new();
        let user_id = "test_user";

        // Create a project first
        let created_project = fixture
            .backend
            .create_project(user_id)
            .await
            .expect("Failed to create project");

        let original_name = created_project.name.clone();
        let original_updated = created_project.updated;
        let new_name = "Updated Project Name";

        // Test updating the project name
        let result = fixture.backend.update_project_name(&created_project.id, new_name).await;
        assert!(result.is_ok(), "Failed to update project name: {:?}", result.err());

        let updated_project = result.unwrap();

        // Verify the project name was updated
        assert_eq!(updated_project.name, new_name);
        assert_ne!(updated_project.name, original_name);

        // Verify the updated timestamp was changed
        assert!(
            updated_project.updated > original_updated,
            "Updated timestamp should be newer"
        );

        // Verify the changes were persisted to the filesystem
        let retrieved_project = fixture
            .backend
            .get_project(&created_project.id)
            .await
            .expect("Failed to retrieve updated project");

        assert_eq!(retrieved_project.name, new_name);
    }

    #[tokio::test]
    async fn test_update_project_name_not_found() {
        let fixture = Fixture::new();

        // Test updating a project that doesn't exist
        let result = fixture
            .backend
            .update_project_name("nonexistent_project_id", "New Name")
            .await;
        assert!(result.is_err(), "Should fail when project doesn't exist");

        let error = result.unwrap_err();
        assert!(error
            .to_string()
            .contains("Project nonexistent_project_id does not exist"));
    }

    #[tokio::test]
    async fn test_update_project_name_special_characters() {
        let fixture = Fixture::new();
        let user_id = "test_user";

        // Create a project first
        let created_project = fixture
            .backend
            .create_project(user_id)
            .await
            .expect("Failed to create project");

        // Test updating with special characters
        let special_name = "My Project! @#$%^&*() - 测试 🎵";
        let result = fixture
            .backend
            .update_project_name(&created_project.id, special_name)
            .await;
        assert!(result.is_ok(), "Should handle special characters in project names");

        let updated_project = result.unwrap();
        assert_eq!(updated_project.name, special_name);

        // Verify it persists correctly
        let retrieved_project = fixture
            .backend
            .get_project(&created_project.id)
            .await
            .expect("Failed to retrieve project with special characters");
        assert_eq!(retrieved_project.name, special_name);
    }

    #[tokio::test]
    async fn test_update_project_file() {
        let fixture = Fixture::new();
        let user_id = "test_user";

        // Create a project first
        let created_project = fixture
            .backend
            .create_project(user_id)
            .await
            .expect("Failed to create project");

        // Verify initial state - project field should be empty and no project.bin file
        assert_eq!(created_project.project, "");
        let project_dir = fixture.backend.directory_for_project(&created_project.id);
        let project_file_path = project_dir.join("project.bin");
        assert!(
            !project_file_path.exists(),
            "Project bin file should not exist initially"
        );

        // Create some test project bytes
        let test_project_bytes = b"test project file content";
        let original_updated = created_project.updated;

        // Test updating the project file
        let result = fixture
            .backend
            .update_project_file(&created_project.id, test_project_bytes)
            .await;
        assert!(result.is_ok(), "Failed to update project file: {:?}", result.err());

        let updated_project = result.unwrap();

        // Verify the project field was updated to point to project.bin
        assert_eq!(updated_project.project, "project.bin");
        assert_ne!(updated_project.project, created_project.project);

        // Verify other fields remain unchanged except updated timestamp
        assert_eq!(updated_project.id, created_project.id);
        assert_eq!(updated_project.name, created_project.name);
        assert_eq!(updated_project.user_id, created_project.user_id);
        assert_eq!(updated_project.samples, created_project.samples);
        assert_eq!(updated_project.created, created_project.created);

        // Verify the updated timestamp was changed
        assert!(
            updated_project.updated > original_updated,
            "Updated timestamp should be newer"
        );

        // Verify the project.bin file was created with the correct content
        assert!(project_file_path.exists(), "Project bin file should be created");
        let written_bytes = tokio::fs::read(&project_file_path)
            .await
            .expect("Failed to read project bin file");
        assert_eq!(written_bytes, test_project_bytes, "Project file content should match");

        // Verify the changes were persisted to the metadata file
        let retrieved_project = fixture
            .backend
            .get_project(&created_project.id)
            .await
            .expect("Failed to retrieve updated project");

        assert_eq!(retrieved_project.project, "project.bin");
        assert!(retrieved_project.updated > original_updated);
    }

    #[tokio::test]
    async fn test_update_project_file_not_found() {
        let fixture = Fixture::new();

        // Test updating a project file for a project that doesn't exist
        let test_project_bytes = b"test project file content";
        let result = fixture
            .backend
            .update_project_file("nonexistent_project_id", test_project_bytes)
            .await;
        assert!(result.is_err(), "Should fail when project doesn't exist");

        let error = result.unwrap_err();
        assert!(error
            .to_string()
            .contains("Project nonexistent_project_id does not exist"));
    }

    #[tokio::test]
    async fn test_update_project_file_empty_bytes() {
        let fixture = Fixture::new();
        let user_id = "test_user";

        // Create a project first
        let created_project = fixture
            .backend
            .create_project(user_id)
            .await
            .expect("Failed to create project");

        // Test updating with empty bytes
        let empty_bytes = b"";
        let result = fixture
            .backend
            .update_project_file(&created_project.id, empty_bytes)
            .await;
        assert!(result.is_ok(), "Should handle empty project files");

        let updated_project = result.unwrap();
        assert_eq!(updated_project.project, "project.bin");

        // Verify the empty file was created
        let project_dir = fixture.backend.directory_for_project(&created_project.id);
        let project_file_path = project_dir.join("project.bin");
        assert!(project_file_path.exists(), "Empty project bin file should be created");

        let written_bytes = tokio::fs::read(&project_file_path)
            .await
            .expect("Failed to read empty project bin file");
        assert_eq!(written_bytes, empty_bytes, "Empty project file should be empty");
    }

    #[tokio::test]
    async fn test_update_project_file_overwrite_existing() {
        let fixture = Fixture::new();
        let user_id = "test_user";

        // Create a project first
        let created_project = fixture
            .backend
            .create_project(user_id)
            .await
            .expect("Failed to create project");

        // First update with some content
        let first_content = b"first version of project";
        let result1 = fixture
            .backend
            .update_project_file(&created_project.id, first_content)
            .await;
        assert!(result1.is_ok(), "First update should succeed");

        // Now update with different content (should overwrite)
        let second_content = b"second version of project - much longer content";
        let result2 = fixture
            .backend
            .update_project_file(&created_project.id, second_content)
            .await;
        assert!(result2.is_ok(), "Second update should succeed");

        let updated_project = result2.unwrap();
        assert_eq!(updated_project.project, "project.bin");

        // Verify the file contains the second content (not the first)
        let project_dir = fixture.backend.directory_for_project(&created_project.id);
        let project_file_path = project_dir.join("project.bin");
        let written_bytes = tokio::fs::read(&project_file_path)
            .await
            .expect("Failed to read overwritten project bin file");
        assert_eq!(
            written_bytes, second_content,
            "Project file should contain the latest content"
        );
    }

    #[tokio::test]
    async fn test_add_project_sample() {
        let fixture = Fixture::new();
        let user_id = "test_user";

        // Create a project first
        let created_project = fixture
            .backend
            .create_project(user_id)
            .await
            .expect("Failed to create project");

        // Verify initial state - no samples
        assert!(
            created_project.samples.is_empty(),
            "Project should start with no samples"
        );
        let project_dir = fixture.backend.directory_for_project(&created_project.id);
        let samples_dir = project_dir.join("samples");
        assert!(!samples_dir.exists(), "Samples directory should not exist initially");

        // Create some test sample bytes
        let test_sample_bytes = b"fake WAV file content for testing";
        let sample_name = "kick_drum";
        let original_updated = created_project.updated;

        // Test adding a project sample
        let result = fixture
            .backend
            .add_project_sample(&created_project.id, test_sample_bytes, sample_name)
            .await;
        assert!(result.is_ok(), "Failed to add project sample: {:?}", result.err());

        let updated_project = result.unwrap();

        // Verify the sample was added to the metadata
        assert_eq!(updated_project.samples.len(), 1, "Project should have one sample");
        let expected_sample_path = format!("samples/{}.wav", sample_name);
        assert!(
            updated_project.samples.contains(&expected_sample_path),
            "Project samples should contain '{}'",
            expected_sample_path
        );

        // Verify the updated timestamp was changed
        assert!(
            updated_project.updated > original_updated,
            "Updated timestamp should be newer"
        );

        // Verify the samples directory was created
        assert!(samples_dir.exists(), "Samples directory should be created");

        // Verify the sample file was created with the correct content
        let sample_file_path = samples_dir.join(format!("{}.wav", sample_name));
        assert!(sample_file_path.exists(), "Sample file should be created");
        let written_bytes = tokio::fs::read(&sample_file_path)
            .await
            .expect("Failed to read sample file");
        assert_eq!(written_bytes, test_sample_bytes, "Sample file content should match");

        // Verify the changes were persisted to the metadata file
        let retrieved_project = fixture
            .backend
            .get_project(&created_project.id)
            .await
            .expect("Failed to retrieve updated project");

        assert_eq!(retrieved_project.samples.len(), 1);
        assert!(retrieved_project.samples.contains(&expected_sample_path));
        assert!(retrieved_project.updated > original_updated);
    }

    #[tokio::test]
    async fn test_add_project_sample_multiple() {
        let fixture = Fixture::new();
        let user_id = "test_user";

        // Create a project first
        let created_project = fixture
            .backend
            .create_project(user_id)
            .await
            .expect("Failed to create project");

        // Add first sample
        let sample1_bytes = b"first sample content";
        let sample1_name = "kick";
        let result1 = fixture
            .backend
            .add_project_sample(&created_project.id, sample1_bytes, sample1_name)
            .await;
        assert!(result1.is_ok(), "Failed to add first sample");

        // Add second sample
        let sample2_bytes = b"second sample content - different";
        let sample2_name = "snare";
        let result2 = fixture
            .backend
            .add_project_sample(&created_project.id, sample2_bytes, sample2_name)
            .await;
        assert!(result2.is_ok(), "Failed to add second sample");

        let updated_project = result2.unwrap();

        // Verify both samples are in the metadata
        assert_eq!(updated_project.samples.len(), 2, "Project should have two samples");
        assert!(
            updated_project.samples.contains(&"samples/kick.wav".to_string()),
            "Should contain first sample"
        );
        assert!(
            updated_project.samples.contains(&"samples/snare.wav".to_string()),
            "Should contain second sample"
        );

        // Verify both files exist
        let project_dir = fixture.backend.directory_for_project(&created_project.id);
        let samples_dir = project_dir.join("samples");

        let sample1_file = samples_dir.join("kick.wav");
        let sample2_file = samples_dir.join("snare.wav");

        assert!(sample1_file.exists(), "First sample file should exist");
        assert!(sample2_file.exists(), "Second sample file should exist");

        // Verify file contents
        let sample1_content = tokio::fs::read(&sample1_file)
            .await
            .expect("Failed to read first sample");
        let sample2_content = tokio::fs::read(&sample2_file)
            .await
            .expect("Failed to read second sample");

        assert_eq!(sample1_content, sample1_bytes);
        assert_eq!(sample2_content, sample2_bytes);
    }

    #[tokio::test]
    async fn test_add_project_sample_not_found() {
        let fixture = Fixture::new();

        // Test adding a sample to a project that doesn't exist
        let test_sample_bytes = b"test sample content";
        let result = fixture
            .backend
            .add_project_sample("nonexistent_project_id", test_sample_bytes, "test_sample")
            .await;
        assert!(result.is_err(), "Should fail when project doesn't exist");

        let error = result.unwrap_err();
        assert!(error
            .to_string()
            .contains("Project nonexistent_project_id does not exist"));
    }

    #[tokio::test]
    async fn test_add_project_sample_special_characters() {
        let fixture = Fixture::new();
        let user_id = "test_user";

        // Create a project first
        let created_project = fixture
            .backend
            .create_project(user_id)
            .await
            .expect("Failed to create project");

        // Test adding a sample with special characters in name
        let test_sample_bytes = b"sample with special chars";
        let sample_name = "drum_loop-01_🥁";
        let result = fixture
            .backend
            .add_project_sample(&created_project.id, test_sample_bytes, sample_name)
            .await;
        assert!(result.is_ok(), "Should handle special characters in sample names");

        let updated_project = result.unwrap();
        let expected_sample_path = format!("samples/{}.wav", sample_name);
        assert!(
            updated_project.samples.contains(&expected_sample_path),
            "Should contain sample with special characters"
        );

        // Verify the file was created correctly
        let project_dir = fixture.backend.directory_for_project(&created_project.id);
        let sample_file_path = project_dir.join("samples").join(format!("{}.wav", sample_name));
        assert!(
            sample_file_path.exists(),
            "Sample file with special characters should exist"
        );
    }

    #[tokio::test]
    async fn test_add_project_sample_overwrite_existing() {
        let fixture = Fixture::new();
        let user_id = "test_user";

        // Create a project first
        let created_project = fixture
            .backend
            .create_project(user_id)
            .await
            .expect("Failed to create project");

        let sample_name = "test_sample";

        // Add first version of sample
        let first_content = b"first version of sample";
        let result1 = fixture
            .backend
            .add_project_sample(&created_project.id, first_content, sample_name)
            .await;
        assert!(result1.is_ok(), "First sample add should succeed");

        // Add same sample name again (should overwrite file but not duplicate in metadata)
        let second_content = b"second version - much longer content";
        let result2 = fixture
            .backend
            .add_project_sample(&created_project.id, second_content, sample_name)
            .await;
        assert!(result2.is_ok(), "Second sample add should succeed");

        let updated_project = result2.unwrap();

        // Should still only have one entry in samples list (no duplicates)
        assert_eq!(updated_project.samples.len(), 1, "Should not duplicate sample entries");

        let expected_sample_path = format!("samples/{}.wav", sample_name);
        assert!(updated_project.samples.contains(&expected_sample_path));

        // Verify the file contains the second content (overwritten)
        let project_dir = fixture.backend.directory_for_project(&created_project.id);
        let sample_file_path = project_dir.join("samples").join(format!("{}.wav", sample_name));
        let written_bytes = tokio::fs::read(&sample_file_path)
            .await
            .expect("Failed to read overwritten sample file");
        assert_eq!(
            written_bytes, second_content,
            "Sample file should contain the latest content"
        );
    }

    #[tokio::test]
    async fn test_remove_project_sample() {
        let fixture = Fixture::new();
        let user_id = "test_user";

        // Create a project first
        let created_project = fixture
            .backend
            .create_project(user_id)
            .await
            .expect("Failed to create project");

        // Add a sample first
        let test_sample_bytes = b"fake WAV file content for testing";
        let sample_name = "kick_drum";
        let add_result = fixture
            .backend
            .add_project_sample(&created_project.id, test_sample_bytes, sample_name)
            .await;
        assert!(add_result.is_ok(), "Failed to add project sample");

        let project_with_sample = add_result.unwrap();
        let original_updated = project_with_sample.updated;

        // Verify the sample exists
        assert_eq!(project_with_sample.samples.len(), 1, "Project should have one sample");
        let expected_sample_path = format!("samples/{}.wav", sample_name);
        assert!(
            project_with_sample.samples.contains(&expected_sample_path),
            "Project should contain the sample"
        );

        let project_dir = fixture.backend.directory_for_project(&created_project.id);
        let sample_file_path = project_dir.join("samples").join(format!("{}.wav", sample_name));
        assert!(sample_file_path.exists(), "Sample file should exist");

        // Test removing the project sample
        let result = fixture
            .backend
            .remove_project_sample(&created_project.id, sample_name)
            .await;
        assert!(result.is_ok(), "Failed to remove project sample: {:?}", result.err());

        let updated_project = result.unwrap();

        // Verify the sample was removed from the metadata
        assert_eq!(updated_project.samples.len(), 0, "Project should have no samples");
        assert!(
            !updated_project.samples.contains(&expected_sample_path),
            "Project samples should not contain '{}'",
            expected_sample_path
        );

        // Verify the updated timestamp was changed
        assert!(
            updated_project.updated > original_updated,
            "Updated timestamp should be newer"
        );

        // Verify the sample file was removed from the filesystem
        assert!(!sample_file_path.exists(), "Sample file should be removed");

        // Verify the changes were persisted to the metadata file
        let retrieved_project = fixture
            .backend
            .get_project(&created_project.id)
            .await
            .expect("Failed to retrieve updated project");

        assert_eq!(retrieved_project.samples.len(), 0);
        assert!(!retrieved_project.samples.contains(&expected_sample_path));
        assert!(retrieved_project.updated > original_updated);
    }

    #[tokio::test]
    async fn test_remove_project_sample_multiple() {
        let fixture = Fixture::new();
        let user_id = "test_user";

        // Create a project first
        let created_project = fixture
            .backend
            .create_project(user_id)
            .await
            .expect("Failed to create project");

        // Add multiple samples
        let sample1_bytes = b"first sample content";
        let sample1_name = "kick";
        let sample2_bytes = b"second sample content";
        let sample2_name = "snare";
        let sample3_bytes = b"third sample content";
        let sample3_name = "hihat";

        fixture
            .backend
            .add_project_sample(&created_project.id, sample1_bytes, sample1_name)
            .await
            .expect("Failed to add first sample");
        fixture
            .backend
            .add_project_sample(&created_project.id, sample2_bytes, sample2_name)
            .await
            .expect("Failed to add second sample");
        fixture
            .backend
            .add_project_sample(&created_project.id, sample3_bytes, sample3_name)
            .await
            .expect("Failed to add third sample");

        // Verify all samples exist
        let project_with_samples = fixture
            .backend
            .get_project(&created_project.id)
            .await
            .expect("Failed to get project");
        assert_eq!(
            project_with_samples.samples.len(),
            3,
            "Project should have three samples"
        );

        // Remove the middle sample
        let result = fixture
            .backend
            .remove_project_sample(&created_project.id, sample2_name)
            .await;
        assert!(result.is_ok(), "Failed to remove sample");

        let updated_project = result.unwrap();

        // Verify only the middle sample was removed
        assert_eq!(updated_project.samples.len(), 2, "Project should have two samples");
        assert!(
            updated_project.samples.contains(&"samples/kick.wav".to_string()),
            "Should still contain first sample"
        );
        assert!(
            !updated_project.samples.contains(&"samples/snare.wav".to_string()),
            "Should not contain removed sample"
        );
        assert!(
            updated_project.samples.contains(&"samples/hihat.wav".to_string()),
            "Should still contain third sample"
        );

        // Verify the files on filesystem
        let project_dir = fixture.backend.directory_for_project(&created_project.id);
        let samples_dir = project_dir.join("samples");

        let sample1_file = samples_dir.join("kick.wav");
        let sample2_file = samples_dir.join("snare.wav");
        let sample3_file = samples_dir.join("hihat.wav");

        assert!(sample1_file.exists(), "First sample file should still exist");
        assert!(!sample2_file.exists(), "Second sample file should be removed");
        assert!(sample3_file.exists(), "Third sample file should still exist");
    }

    #[tokio::test]
    async fn test_remove_project_sample_not_found() {
        let fixture = Fixture::new();

        // Test removing a sample from a project that doesn't exist
        let result = fixture
            .backend
            .remove_project_sample("nonexistent_project_id", "test_sample")
            .await;
        assert!(result.is_err(), "Should fail when project doesn't exist");

        let error = result.unwrap_err();
        assert!(error
            .to_string()
            .contains("Project nonexistent_project_id does not exist"));
    }

    #[tokio::test]
    async fn test_remove_project_sample_sample_not_found() {
        let fixture = Fixture::new();
        let user_id = "test_user";

        // Create a project first
        let created_project = fixture
            .backend
            .create_project(user_id)
            .await
            .expect("Failed to create project");

        // Add one sample
        let test_sample_bytes = b"sample content";
        let sample_name = "existing_sample";
        fixture
            .backend
            .add_project_sample(&created_project.id, test_sample_bytes, sample_name)
            .await
            .expect("Failed to add sample");

        // Try to remove a different sample that doesn't exist
        let result = fixture
            .backend
            .remove_project_sample(&created_project.id, "nonexistent_sample")
            .await;
        assert!(result.is_err(), "Should fail when sample doesn't exist");

        let error = result.unwrap_err();
        assert!(
            error.to_string().contains("Sample nonexistent_sample does not exist")
                || error.to_string().contains("Sample not found")
        );
    }

    #[tokio::test]
    async fn test_remove_project_sample_empty_project() {
        let fixture = Fixture::new();
        let user_id = "test_user";

        // Create a project with no samples
        let created_project = fixture
            .backend
            .create_project(user_id)
            .await
            .expect("Failed to create project");

        // Try to remove a sample from an empty project
        let result = fixture
            .backend
            .remove_project_sample(&created_project.id, "nonexistent_sample")
            .await;
        assert!(result.is_err(), "Should fail when trying to remove from empty project");

        let error = result.unwrap_err();
        assert!(
            error.to_string().contains("Sample nonexistent_sample does not exist")
                || error.to_string().contains("Sample not found")
        );
    }

    #[tokio::test]
    async fn test_remove_project_sample_special_characters() {
        let fixture = Fixture::new();
        let user_id = "test_user";

        // Create a project first
        let created_project = fixture
            .backend
            .create_project(user_id)
            .await
            .expect("Failed to create project");

        // Add a sample with special characters in name
        let test_sample_bytes = b"sample with special chars";
        let sample_name = "drum_loop-01_🥁";
        fixture
            .backend
            .add_project_sample(&created_project.id, test_sample_bytes, sample_name)
            .await
            .expect("Failed to add sample with special characters");

        // Remove the sample with special characters
        let result = fixture
            .backend
            .remove_project_sample(&created_project.id, sample_name)
            .await;
        assert!(result.is_ok(), "Should handle special characters in sample names");

        let updated_project = result.unwrap();
        let expected_sample_path = format!("samples/{}.wav", sample_name);
        assert!(
            !updated_project.samples.contains(&expected_sample_path),
            "Should not contain sample with special characters"
        );

        // Verify the file was removed correctly
        let project_dir = fixture.backend.directory_for_project(&created_project.id);
        let sample_file_path = project_dir.join("samples").join(format!("{}.wav", sample_name));
        assert!(
            !sample_file_path.exists(),
            "Sample file with special characters should be removed"
        );
    }

    #[tokio::test]
    async fn test_remove_project_sample_leaves_samples_directory() {
        let fixture = Fixture::new();
        let user_id = "test_user";

        // Create a project first
        let created_project = fixture
            .backend
            .create_project(user_id)
            .await
            .expect("Failed to create project");

        // Add a sample
        let test_sample_bytes = b"sample content";
        let sample_name = "test_sample";
        fixture
            .backend
            .add_project_sample(&created_project.id, test_sample_bytes, sample_name)
            .await
            .expect("Failed to add sample");

        let project_dir = fixture.backend.directory_for_project(&created_project.id);
        let samples_dir = project_dir.join("samples");
        assert!(samples_dir.exists(), "Samples directory should exist");

        // Remove the sample
        let result = fixture
            .backend
            .remove_project_sample(&created_project.id, sample_name)
            .await;
        assert!(result.is_ok(), "Failed to remove sample");

        // Verify the samples directory still exists (even if empty)
        assert!(
            samples_dir.exists(),
            "Samples directory should still exist after removing last sample"
        );
    }

    #[test]
    fn test_generate_project_id() {
        // Test that generate_project_id creates valid IDs
        for _ in 0..100 {
            let id = FilesystemBackend::generate_project_id();

            // Check length
            assert_eq!(id.len(), 15, "Project ID should be 15 characters long");

            // Check characters are valid (a-z, 0-9)
            for ch in id.chars() {
                assert!(
                    ch.is_ascii_lowercase() || ch.is_ascii_digit(),
                    "Invalid character '{}' in project ID '{}'",
                    ch,
                    id
                );
            }
        }

        // Test that IDs are unique (very high probability with 36^15 combinations)
        let id1 = FilesystemBackend::generate_project_id();
        let id2 = FilesystemBackend::generate_project_id();
        assert_ne!(id1, id2, "Generated IDs should be unique");
    }
}
