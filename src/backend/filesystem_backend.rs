use std::path::PathBuf;

use crate::{
    backend::{Backend, DbProject},
    bloop::Project,
};
use anyhow::{Context, Result};
use protobuf::Message;
use rand::Rng;

/**
 * FilesystemBackend is a backend implementation that interacts with the filesystem
 * to manage projects and their associated files.
 *
 * The directory structure is:
 * - projects/{project_id}/
 *   - project.json (the project metadata file)
 *   - project.bin (the binary project file)
 *   - samples/ (directory containing sample files)
 *     - sample_name (sample file)
 *
 * There is a special anonymous user with ID "anonymous" that can be used for
 * projects that do not yet have an associated user.
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
}

#[async_trait::async_trait]
impl Backend for FilesystemBackend {
    async fn get_projects(&self) -> Result<Vec<DbProject>> {
        // Implementation for getting projects from the filesystem
        unimplemented!()
    }

    async fn get_project(&self, project_id: &str) -> Result<DbProject> {
        // Implementation for getting a specific project from the filesystem
        unimplemented!()
    }

    async fn create_project(&self, user_id: &str) -> Result<DbProject> {
        let project_id = Self::generate_project_id();

        let db_project = DbProject {
            id: project_id.clone(),
            name: "New Project".to_string(),
            user_id: user_id.to_string(),
            project: "project.bin".to_string(),
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

        // Write an empty project file
        let project_file_path = project_dir.join("project.bin");
        let project = Project::empty().with_songs(1, 1);
        let project_bytes = project.write_to_bytes().context("Failed to serialize project")?;
        tokio::fs::write(&project_file_path, project_bytes)
            .await
            .context(format!(
                "Failed to write project file for user {} and project {}",
                user_id, project_id
            ))?;

        // Write the project metadata file
        let metadata_file_path = project_dir.join("project.json");
        let metadata_bytes = serde_json::to_vec(&db_project).context("Failed to serialize project metadata")?;
        tokio::fs::write(&metadata_file_path, metadata_bytes)
            .await
            .context(format!(
                "Failed to write project metadata file for user {} and project {}",
                user_id, project_id
            ))?;

        Ok(db_project)
    }

    async fn update_project_name(&self, project_id: &str, name: &str) -> Result<DbProject> {
        // Implementation for updating a project's name in the filesystem
        unimplemented!()
    }

    async fn update_project_file(&self, project_id: &str, project_bytes: &[u8]) -> Result<DbProject> {
        // Implementation for updating a project's file in the filesystem
        unimplemented!()
    }

    async fn add_project_sample(&self, project_id: &str, sample_bytes: &[u8], sample_name: &str) -> Result<DbProject> {
        // Implementation for adding a sample to a project in the filesystem
        unimplemented!()
    }

    async fn remove_project_sample(&self, project_id: &str, sample_name: &str) -> Result<DbProject> {
        // Implementation for removing a sample from a project in the filesystem
        unimplemented!()
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
        assert_eq!(db_project.project, "project.bin");
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

        // Verify the project.bin file was created
        let project_file = project_dir.join("project.bin");
        assert!(project_file.exists(), "Project bin file was not created");

        // Verify the project has some default content
        let project_bytes = tokio::fs::read(&project_file)
            .await
            .expect("Failed to read project file");

        let project = Project::parse_from_bytes(&project_bytes).expect("Failed to parse project file");
        assert_eq!(project.songs.len(), 1, "Project should have 1 song");

        // Verify the project.json metadata file was created
        let metadata_file = project_dir.join("project.json");
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
