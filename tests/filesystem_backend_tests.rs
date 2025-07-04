use bloop::backend::Backend;
use bloop::backend::DbProject;
use bloop::backend::FilesystemBackend;
use tempfile::TempDir;

struct Fixture {
    temp_dir: TempDir,
    backend: FilesystemBackend,
}

impl Fixture {
    fn new() -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let backend = FilesystemBackend::new(temp_dir.path());
        Self { temp_dir, backend }
    }

    fn expected_project_dir(&self, project_id: &str) -> std::path::PathBuf {
        self.temp_dir.path().join(project_id)
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
    let project_dir = fixture.expected_project_dir(&db_project.id);
    assert!(project_dir.exists(), "Project directory was not created");

    // Verify the project.bin file was NOT created (since we don't create it anymore)
    let project_file = project_dir.join("project.bin");
    assert!(!project_file.exists(), "Project bin file should not be created");

    // Verify the project.json metadata file was created
    let metadata_file = fixture.expected_project_dir(&db_project.id).join("project.json");
    assert!(metadata_file.exists(), "Project metadata file was not created");

    // Read and verify the metadata file contains correct data
    let metadata_content = tokio::fs::read(&metadata_file)
        .await
        .expect("Failed to read metadata file");
    let loaded_project: DbProject = serde_json::from_slice(&metadata_content).expect("Failed to parse metadata file");

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
    let invalid_file = fixture.temp_dir.path().join("not_a_directory.txt");
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
    let result = fixture.backend.read_project(&created_project.id).await;
    assert!(result.is_ok(), "Failed to get project: {:?}", result.err());

    let project = result.unwrap();
    assert_eq!(project.id, created_project.id);
    assert_eq!(project.name, created_project.name);
    assert_eq!(project.user_id, created_project.user_id);
}

#[tokio::test]
async fn test_get_project_not_found() {
    let fixture = Fixture::new();

    // Test getting a project that doesn't exist
    let result = fixture.backend.read_project("nonexistent_project_id").await;
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
        .read_project(&created_project.id)
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
        .read_project(&created_project.id)
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
    let project_dir = fixture.expected_project_dir(&created_project.id);
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

    // Verify other fields remain unchanged except updated timestamp
    assert_eq!(updated_project.id, created_project.id);
    assert_eq!(updated_project.name, created_project.name);
    assert_eq!(updated_project.user_id, created_project.user_id);
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
        .read_project(&created_project.id)
        .await
        .expect("Failed to retrieve updated project");

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

    // Verify the empty file was created
    let project_dir = fixture.expected_project_dir(&created_project.id);
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

    // Verify the file contains the second content (not the first)
    let project_dir = fixture.expected_project_dir(&created_project.id);
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
    let project_dir = fixture.expected_project_dir(&created_project.id);
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
        .read_project(&created_project.id)
        .await
        .expect("Failed to retrieve updated project");

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

    // Verify both files exist
    let project_dir = fixture.expected_project_dir(&created_project.id);
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

    // Verify the file was created correctly
    let project_dir = fixture.expected_project_dir(&created_project.id);
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

    // Verify the file contains the second content (overwritten)
    let project_dir = fixture.expected_project_dir(&created_project.id);
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
    let project_dir = fixture.expected_project_dir(&created_project.id);
    let sample_file_path = project_dir.join("samples").join(format!("{}.wav", sample_name));
    assert!(sample_file_path.exists(), "Sample file should exist");

    // Test removing the project sample
    let result = fixture
        .backend
        .remove_project_sample(&created_project.id, sample_name)
        .await;
    assert!(result.is_ok(), "Failed to remove project sample: {:?}", result.err());

    let updated_project = result.unwrap();

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
        .read_project(&created_project.id)
        .await
        .expect("Failed to retrieve updated project");

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

    // Remove the middle sample
    let result = fixture
        .backend
        .remove_project_sample(&created_project.id, sample2_name)
        .await;
    assert!(result.is_ok(), "Failed to remove sample");

    // Verify the files on filesystem
    let project_dir = fixture.expected_project_dir(&created_project.id);
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

    // Verify the file was removed correctly
    let project_dir = fixture.expected_project_dir(&created_project.id);
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

    let project_dir = fixture.expected_project_dir(&created_project.id);
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

#[tokio::test]
async fn test_remove_project() {
    let fixture = Fixture::new();
    let user_id = "test_user";

    // Create a project first
    let created_project = fixture
        .backend
        .create_project(user_id)
        .await
        .expect("Failed to create project");

    // Add some content to make the project more realistic
    fixture
        .backend
        .update_project_name(&created_project.id, "Test Project")
        .await
        .expect("Failed to update project name");

    fixture
        .backend
        .update_project_file(&created_project.id, b"test project content")
        .await
        .expect("Failed to update project file");

    fixture
        .backend
        .add_project_sample(&created_project.id, b"sample content", "test_sample")
        .await
        .expect("Failed to add sample");

    // Verify the project and its files exist
    let project_dir = fixture.expected_project_dir(&created_project.id);
    assert!(project_dir.exists(), "Project directory should exist");

    let metadata_file = fixture.expected_project_dir(&created_project.id).join("project.json");
    assert!(metadata_file.exists(), "Metadata file should exist");

    let project_file = project_dir.join("project.bin");
    assert!(project_file.exists(), "Project file should exist");

    let sample_file = project_dir.join("samples").join("test_sample.wav");
    assert!(sample_file.exists(), "Sample file should exist");

    // Verify the project is accessible
    let retrieved_project = fixture
        .backend
        .read_project(&created_project.id)
        .await
        .expect("Project should be retrievable");
    assert_eq!(retrieved_project.name, "Test Project");

    // Test removing the project
    let result = fixture.backend.remove_project(&created_project.id).await;
    assert!(result.is_ok(), "Failed to remove project: {:?}", result.err());

    // Verify the entire project directory was removed
    assert!(!project_dir.exists(), "Project directory should be removed");
    assert!(!metadata_file.exists(), "Metadata file should be removed");
    assert!(!project_file.exists(), "Project file should be removed");
    assert!(!sample_file.exists(), "Sample file should be removed");

    // Verify the project is no longer accessible
    let result = fixture.backend.read_project(&created_project.id).await;
    assert!(result.is_err(), "Project should no longer be accessible");
}

#[tokio::test]
async fn test_remove_project_not_found() {
    let fixture = Fixture::new();

    // Test removing a project that doesn't exist - should not be an error
    let result = fixture.backend.remove_project("nonexistent_project_id").await;
    assert!(result.is_ok(), "Removing non-existent project should not be an error");
}

#[tokio::test]
async fn test_remove_project_empty_project() {
    let fixture = Fixture::new();
    let user_id = "test_user";

    // Create a minimal project with no additional content
    let created_project = fixture
        .backend
        .create_project(user_id)
        .await
        .expect("Failed to create project");

    // Verify the project exists
    let project_dir = fixture.expected_project_dir(&created_project.id);
    assert!(project_dir.exists(), "Project directory should exist");

    // Test removing the empty project
    let result = fixture.backend.remove_project(&created_project.id).await;
    assert!(result.is_ok(), "Failed to remove empty project: {:?}", result.err());

    // Verify the project directory was removed
    assert!(!project_dir.exists(), "Project directory should be removed");
}

#[tokio::test]
async fn test_remove_project_multiple_projects() {
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
        .create_project("user3")
        .await
        .expect("Failed to create project 3");

    // Verify all projects exist
    let all_projects = fixture.backend.get_projects().await.expect("Failed to get projects");
    assert_eq!(all_projects.len(), 3, "Should have three projects");

    // Remove the middle project
    let result = fixture.backend.remove_project(&project2.id).await;
    assert!(result.is_ok(), "Failed to remove project 2");

    // Verify only project 2 was removed
    let project1_dir = fixture.expected_project_dir(&project1.id);
    let project2_dir = fixture.expected_project_dir(&project2.id);
    let project3_dir = fixture.expected_project_dir(&project3.id);

    assert!(project1_dir.exists(), "Project 1 should still exist");
    assert!(!project2_dir.exists(), "Project 2 should be removed");
    assert!(project3_dir.exists(), "Project 3 should still exist");

    // Verify remaining projects are still accessible
    let remaining_projects = fixture
        .backend
        .get_projects()
        .await
        .expect("Failed to get remaining projects");
    assert_eq!(remaining_projects.len(), 2, "Should have two remaining projects");

    let remaining_ids: Vec<String> = remaining_projects.iter().map(|p| p.id.clone()).collect();
    assert!(remaining_ids.contains(&project1.id), "Should still contain project 1");
    assert!(!remaining_ids.contains(&project2.id), "Should not contain project 2");
    assert!(remaining_ids.contains(&project3.id), "Should still contain project 3");
}

#[tokio::test]
async fn test_remove_project_with_nested_content() {
    let fixture = Fixture::new();
    let user_id = "test_user";

    // Create a project with various content
    let created_project = fixture
        .backend
        .create_project(user_id)
        .await
        .expect("Failed to create project");

    // Add multiple samples to create nested directory structure
    fixture
        .backend
        .add_project_sample(&created_project.id, b"kick content", "kick")
        .await
        .expect("Failed to add kick sample");
    fixture
        .backend
        .add_project_sample(&created_project.id, b"snare content", "snare")
        .await
        .expect("Failed to add snare sample");
    fixture
        .backend
        .add_project_sample(&created_project.id, b"hihat content", "hihat")
        .await
        .expect("Failed to add hihat sample");

    // Add project file
    fixture
        .backend
        .update_project_file(&created_project.id, b"complex project content")
        .await
        .expect("Failed to update project file");

    let project_dir = fixture.expected_project_dir(&created_project.id);
    let samples_dir = project_dir.join("samples");

    // Verify complex structure exists
    assert!(project_dir.exists(), "Project directory should exist");
    assert!(samples_dir.exists(), "Samples directory should exist");
    assert!(samples_dir.join("kick.wav").exists(), "Kick sample should exist");
    assert!(samples_dir.join("snare.wav").exists(), "Snare sample should exist");
    assert!(samples_dir.join("hihat.wav").exists(), "Hihat sample should exist");
    assert!(project_dir.join("project.bin").exists(), "Project file should exist");
    assert!(project_dir.join("project.json").exists(), "Metadata file should exist");

    // Remove the project
    let result = fixture.backend.remove_project(&created_project.id).await;
    assert!(result.is_ok(), "Failed to remove project with nested content");

    // Verify everything was removed recursively
    assert!(!project_dir.exists(), "Project directory should be completely removed");
    assert!(!samples_dir.exists(), "Samples directory should be removed");
}

#[tokio::test]
async fn test_remove_project_special_characters() {
    let fixture = Fixture::new();
    let user_id = "test_user";

    // Create a project
    let created_project = fixture
        .backend
        .create_project(user_id)
        .await
        .expect("Failed to create project");

    // Add content with special characters
    fixture
        .backend
        .update_project_name(&created_project.id, "Project with 特殊字符 and émojis 🎵!")
        .await
        .expect("Failed to update project name");

    fixture
        .backend
        .add_project_sample(&created_project.id, b"special content", "sample-with_特殊🎵")
        .await
        .expect("Failed to add sample with special characters");

    let project_dir = fixture.expected_project_dir(&created_project.id);
    assert!(project_dir.exists(), "Project directory should exist");

    // Remove the project
    let result = fixture.backend.remove_project(&created_project.id).await;
    assert!(result.is_ok(), "Should handle special characters in project removal");

    // Verify removal was successful
    assert!(
        !project_dir.exists(),
        "Project directory with special content should be removed"
    );
}

#[tokio::test]
async fn test_remove_project_twice() {
    let fixture = Fixture::new();
    let user_id = "test_user";

    // Create a project
    let created_project = fixture
        .backend
        .create_project(user_id)
        .await
        .expect("Failed to create project");

    let project_dir = fixture.expected_project_dir(&created_project.id);
    assert!(project_dir.exists(), "Project directory should exist");

    // Remove the project first time
    let result1 = fixture.backend.remove_project(&created_project.id).await;
    assert!(result1.is_ok(), "First removal should succeed");
    assert!(!project_dir.exists(), "Project directory should be removed");

    // Remove the project second time (should not be an error)
    let result2 = fixture.backend.remove_project(&created_project.id).await;
    assert!(result2.is_ok(), "Second removal should not be an error");
}

#[tokio::test]
async fn test_get_project_file_project_bin() {
    let fixture = Fixture::new();
    let user_id = "test_user";

    // Create a project first
    let created_project = fixture
        .backend
        .create_project(user_id)
        .await
        .expect("Failed to create project");

    // Add project file content
    let project_content = b"test project binary content";
    fixture
        .backend
        .update_project_file(&created_project.id, project_content)
        .await
        .expect("Failed to update project file");

    // Test retrieving the project.bin file
    let result = fixture.backend.read_project_file(&created_project.id).await;
    assert!(result.is_ok(), "Failed to get project file: {:?}", result.err());

    let retrieved_content = result.unwrap();
    assert_eq!(retrieved_content, project_content, "Retrieved content should match");
}

#[tokio::test]
async fn test_get_project_file_sample() {
    let fixture = Fixture::new();
    let user_id = "test_user";

    // Create a project first
    let created_project = fixture
        .backend
        .create_project(user_id)
        .await
        .expect("Failed to create project");

    // Add a sample
    let sample_content = b"fake WAV file content for testing";
    let sample_name = "kick_drum";
    fixture
        .backend
        .add_project_sample(&created_project.id, sample_content, sample_name)
        .await
        .expect("Failed to add sample");

    // Test retrieving the sample file
    let result = fixture.backend.read_sample(&created_project.id, sample_name).await;
    assert!(result.is_ok(), "Failed to get sample file: {:?}", result.err());

    let retrieved_content = result.unwrap();
    assert_eq!(
        retrieved_content, sample_content,
        "Retrieved sample content should match"
    );
}

#[tokio::test]
async fn test_get_project_file_metadata() {
    let fixture = Fixture::new();
    let user_id = "test_user";

    // Create a project first
    let created_project = fixture
        .backend
        .create_project(user_id)
        .await
        .expect("Failed to create project");

    // Test retrieving the project.json metadata file
    let result = fixture.backend.read_project(&created_project.id).await;
    assert!(result.is_ok(), "Failed to get metadata file: {:?}", result.err());

    // Verify the metadata contains the expected fields
    let parsed_project = result.unwrap();
    assert_eq!(parsed_project.id, created_project.id);
    assert_eq!(parsed_project.name, created_project.name);
    assert_eq!(parsed_project.user_id, created_project.user_id);
}

#[tokio::test]
async fn test_get_project_file_multiple_samples() {
    let fixture = Fixture::new();
    let user_id = "test_user";

    // Create a project first
    let created_project = fixture
        .backend
        .create_project(user_id)
        .await
        .expect("Failed to create project");

    // Add multiple samples with different content
    let kick_content = b"kick drum sample content";
    let snare_content = b"snare drum sample content - different";
    let hihat_content = b"hihat sample with unique content";

    fixture
        .backend
        .add_project_sample(&created_project.id, kick_content, "kick")
        .await
        .expect("Failed to add kick sample");
    fixture
        .backend
        .add_project_sample(&created_project.id, snare_content, "snare")
        .await
        .expect("Failed to add snare sample");
    fixture
        .backend
        .add_project_sample(&created_project.id, hihat_content, "hihat")
        .await
        .expect("Failed to add hihat sample");

    // Test retrieving each sample file individually
    let kick_result = fixture.backend.read_sample(&created_project.id, "kick").await;
    assert!(kick_result.is_ok(), "Failed to get kick sample");
    assert_eq!(kick_result.unwrap(), kick_content);

    let snare_result = fixture.backend.read_sample(&created_project.id, "snare").await;
    assert!(snare_result.is_ok(), "Failed to get snare sample");
    assert_eq!(snare_result.unwrap(), snare_content);

    let hihat_result = fixture.backend.read_sample(&created_project.id, "hihat").await;
    assert!(hihat_result.is_ok(), "Failed to get hihat sample");
    assert_eq!(hihat_result.unwrap(), hihat_content);
}

#[tokio::test]
async fn test_get_project_file_project_not_found() {
    let fixture = Fixture::new();

    // Test getting a file from a project that doesn't exist
    let result = fixture.backend.read_project_file("nonexistent_project_id").await;
    assert!(result.is_err(), "Should fail when project doesn't exist");

    let error = result.unwrap_err();
    assert!(error
        .to_string()
        .contains("Project nonexistent_project_id does not exist"));
}

#[tokio::test]
async fn test_get_project_file_sample_not_found() {
    let fixture = Fixture::new();
    let user_id = "test_user";

    // Create a project first
    let created_project = fixture
        .backend
        .create_project(user_id)
        .await
        .expect("Failed to create project");

    // Add one sample
    fixture
        .backend
        .add_project_sample(&created_project.id, b"sample content", "existing_sample")
        .await
        .expect("Failed to add sample");

    // Test getting a sample that doesn't exist
    let result = fixture
        .backend
        .read_sample(&created_project.id, "nonexistent_sample")
        .await;
    assert!(result.is_err(), "Should fail when sample file doesn't exist");
}

#[tokio::test]
async fn test_get_project_file_empty_file() {
    let fixture = Fixture::new();
    let user_id = "test_user";

    // Create a project first
    let created_project = fixture
        .backend
        .create_project(user_id)
        .await
        .expect("Failed to create project");

    // Add project file with empty content
    let empty_content = b"";
    fixture
        .backend
        .update_project_file(&created_project.id, empty_content)
        .await
        .expect("Failed to update project file with empty content");

    // Test retrieving the empty project.bin file
    let result = Backend::read_project_file(&fixture.backend, &created_project.id).await;
    assert!(result.is_ok(), "Should handle empty files");

    let retrieved_content = result.unwrap();
    assert_eq!(
        retrieved_content, empty_content,
        "Should retrieve empty content correctly"
    );
}

#[tokio::test]
async fn test_get_project_file_large_file() {
    let fixture = Fixture::new();
    let user_id = "test_user";

    // Create a project first
    let created_project = fixture
        .backend
        .create_project(user_id)
        .await
        .expect("Failed to create project");

    // Create a large file content (10KB)
    let large_content = vec![0xAB; 10 * 1024];
    fixture
        .backend
        .update_project_file(&created_project.id, &large_content)
        .await
        .expect("Failed to update project file with large content");

    // Test retrieving the large project.bin file
    let result = Backend::read_project_file(&fixture.backend, &created_project.id).await;
    assert!(result.is_ok(), "Should handle large files");

    let retrieved_content = result.unwrap();
    assert_eq!(
        retrieved_content, large_content,
        "Should retrieve large content correctly"
    );
    assert_eq!(retrieved_content.len(), 10 * 1024, "Content size should match");
}

#[tokio::test]
async fn test_get_project_file_special_characters() {
    let fixture = Fixture::new();
    let user_id = "test_user";

    // Create a project first
    let created_project = fixture
        .backend
        .create_project(user_id)
        .await
        .expect("Failed to create project");

    // Add a sample with special characters in the name
    let sample_content = b"sample with special chars in filename";
    let sample_name = "drum_loop-01_🥁";
    fixture
        .backend
        .add_project_sample(&created_project.id, sample_content, sample_name)
        .await
        .expect("Failed to add sample with special characters");

    // Test retrieving the sample with special characters
    let result = fixture.backend.read_sample(&created_project.id, sample_name).await;
    assert!(result.is_ok(), "Should handle special characters in filenames");

    let retrieved_content = result.unwrap();
    assert_eq!(
        retrieved_content, sample_content,
        "Retrieved sample content should match"
    );
}

#[tokio::test]
async fn test_get_project_file_path_traversal_security() {
    let fixture = Fixture::new();
    let user_id = "test_user";

    // Create a project first
    let created_project = fixture
        .backend
        .create_project(user_id)
        .await
        .expect("Failed to create project");

    // Test various path traversal attempts - these should all fail
    let malicious_paths = vec![
        "../../../etc/passwd",
        "..\\..\\..\\windows\\system32\\config\\sam",
        "samples/../../../sensitive_file.txt",
        "samples/..\\..\\..\\sensitive_file.txt",
        "/etc/passwd",
        "C:\\windows\\system32\\config\\sam",
    ];

    for malicious_path in malicious_paths {
        let result = fixture.backend.read_sample(&created_project.id, malicious_path).await;
        assert!(
            result.is_err(),
            "Path traversal attempt should fail for path: {}",
            malicious_path
        );
    }
}

#[tokio::test]
async fn test_get_project_file_binary_content() {
    let fixture = Fixture::new();
    let user_id = "test_user";

    // Create a project first
    let created_project = fixture
        .backend
        .create_project(user_id)
        .await
        .expect("Failed to create project");

    // Create binary content with various byte values
    let binary_content: Vec<u8> = (0..=255).cycle().take(1000).collect();
    fixture
        .backend
        .add_project_sample(&created_project.id, &binary_content, "binary_sample")
        .await
        .expect("Failed to add binary sample");

    // Test retrieving the binary content
    let result = fixture.backend.read_sample(&created_project.id, "binary_sample").await;
    assert!(result.is_ok(), "Should handle binary content");

    let retrieved_content = result.unwrap();
    assert_eq!(
        retrieved_content, binary_content,
        "Binary content should be preserved exactly"
    );
}

#[tokio::test]
async fn test_get_samples_empty_project() {
    let fixture = Fixture::new();
    let user_id = "test_user";

    // Create a project with no samples
    let created_project = fixture
        .backend
        .create_project(user_id)
        .await
        .expect("Failed to create project");

    // Test getting samples from empty project
    let result = fixture.backend.get_samples(&created_project.id).await;
    assert!(result.is_ok(), "Failed to get samples: {:?}", result.err());

    let samples = result.unwrap();
    assert!(
        samples.is_empty(),
        "Should return empty vector for project with no samples"
    );
}

#[tokio::test]
async fn test_get_samples_single_sample() {
    let fixture = Fixture::new();
    let user_id = "test_user";

    // Create a project
    let created_project = fixture
        .backend
        .create_project(user_id)
        .await
        .expect("Failed to create project");

    // Add a sample
    let sample_bytes = b"test sample content";
    let sample_name = "kick_drum";
    fixture
        .backend
        .add_project_sample(&created_project.id, sample_bytes, sample_name)
        .await
        .expect("Failed to add sample");

    // Test getting samples
    let result = fixture.backend.get_samples(&created_project.id).await;
    assert!(result.is_ok(), "Failed to get samples: {:?}", result.err());

    let samples = result.unwrap();
    assert_eq!(samples.len(), 1, "Should return one sample");
    assert_eq!(samples[0], sample_name, "Sample name should match");
}

#[tokio::test]
async fn test_get_samples_multiple_samples() {
    let fixture = Fixture::new();
    let user_id = "test_user";

    // Create a project
    let created_project = fixture
        .backend
        .create_project(user_id)
        .await
        .expect("Failed to create project");

    // Add multiple samples
    let samples_to_add = vec!["kick", "snare", "hihat", "crash"];
    for sample_name in &samples_to_add {
        fixture
            .backend
            .add_project_sample(&created_project.id, b"sample content", sample_name)
            .await
            .expect("Failed to add sample");
    }

    // Test getting samples
    let result = fixture.backend.get_samples(&created_project.id).await;
    assert!(result.is_ok(), "Failed to get samples: {:?}", result.err());

    let mut samples = result.unwrap();
    samples.sort(); // Ensure consistent ordering for comparison
    let mut expected = samples_to_add.clone();
    expected.sort();

    assert_eq!(samples.len(), 4, "Should return four samples");
    assert_eq!(samples, expected, "Sample names should match");
}

#[tokio::test]
async fn test_get_samples_project_not_found() {
    let fixture = Fixture::new();

    // Test getting samples from non-existent project
    let result = fixture.backend.get_samples("nonexistent_project_id").await;
    assert!(result.is_err(), "Should fail when project doesn't exist");

    let error = result.unwrap_err();
    assert!(error
        .to_string()
        .contains("Project nonexistent_project_id does not exist"));
}

#[tokio::test]
async fn test_get_samples_special_characters() {
    let fixture = Fixture::new();
    let user_id = "test_user";

    // Create a project
    let created_project = fixture
        .backend
        .create_project(user_id)
        .await
        .expect("Failed to create project");

    // Add samples with special characters
    let special_sample_name = "drum_loop-01_🥁";
    fixture
        .backend
        .add_project_sample(&created_project.id, b"special sample", special_sample_name)
        .await
        .expect("Failed to add special sample");

    // Test getting samples
    let result = fixture.backend.get_samples(&created_project.id).await;
    assert!(result.is_ok(), "Should handle special characters in sample names");

    let samples = result.unwrap();
    assert_eq!(samples.len(), 1, "Should return one sample");
    assert_eq!(samples[0], special_sample_name, "Should preserve special characters");
}

#[tokio::test]
async fn test_read_sample() {
    let fixture = Fixture::new();
    let user_id = "test_user";

    // Create a project
    let created_project = fixture
        .backend
        .create_project(user_id)
        .await
        .expect("Failed to create project");

    // Add a sample
    let sample_bytes = b"test sample content with unique data";
    let sample_name = "test_sample";
    fixture
        .backend
        .add_project_sample(&created_project.id, sample_bytes, sample_name)
        .await
        .expect("Failed to add sample");

    // Test reading the sample
    let result = fixture.backend.read_sample(&created_project.id, sample_name).await;
    assert!(result.is_ok(), "Failed to read sample: {:?}", result.err());

    let retrieved_bytes = result.unwrap();
    assert_eq!(
        retrieved_bytes, sample_bytes,
        "Retrieved sample content should match original"
    );
}

#[tokio::test]
async fn test_read_sample_project_not_found() {
    let fixture = Fixture::new();

    // Test reading sample from non-existent project
    let result = fixture
        .backend
        .read_sample("nonexistent_project_id", "test_sample")
        .await;
    assert!(result.is_err(), "Should fail when project doesn't exist");

    let error = result.unwrap_err();
    assert!(error
        .to_string()
        .contains("Project nonexistent_project_id does not exist"));
}

#[tokio::test]
async fn test_read_sample_not_found() {
    let fixture = Fixture::new();
    let user_id = "test_user";

    // Create a project
    let created_project = fixture
        .backend
        .create_project(user_id)
        .await
        .expect("Failed to create project");

    // Test reading non-existent sample
    let result = fixture
        .backend
        .read_sample(&created_project.id, "nonexistent_sample")
        .await;
    assert!(result.is_err(), "Should fail when sample doesn't exist");
}

#[tokio::test]
async fn test_read_sample_path_traversal_protection() {
    let fixture = Fixture::new();
    let user_id = "test_user";

    // Create a project
    let created_project = fixture
        .backend
        .create_project(user_id)
        .await
        .expect("Failed to create project");

    // Test various path traversal attempts

    let malicious_names = vec![
        "../../../etc/passwd",
        "..\\..\\windows\\system32",
        "/etc/passwd",
        "sample../other",
        "sample/nested",
        ".hidden_file",
    ];

    for malicious_name in malicious_names {
        let result = fixture.backend.read_sample(&created_project.id, malicious_name).await;
        assert!(
            result.is_err(),
            "Should reject malicious sample name: {}",
            malicious_name
        );
    }
}

#[tokio::test]
async fn test_read_sample_binary_content() {
    let fixture = Fixture::new();
    let user_id = "test_user";

    // Create a project
    let created_project = fixture
        .backend
        .create_project(user_id)
        .await
        .expect("Failed to create project");

    // Create binary content with various byte values
    let binary_content: Vec<u8> = (0..=255).cycle().take(1000).collect();
    fixture
        .backend
        .add_project_sample(&created_project.id, &binary_content, "binary_sample")
        .await
        .expect("Failed to add binary sample");

    // Test reading the binary content
    let result = fixture.backend.read_sample(&created_project.id, "binary_sample").await;
    assert!(result.is_ok(), "Should handle binary content");

    let retrieved_content = result.unwrap();
    assert_eq!(
        retrieved_content, binary_content,
        "Binary content should be preserved exactly"
    );
}

#[tokio::test]
async fn test_read_project_file() {
    let fixture = Fixture::new();
    let user_id = "test_user";

    // Create a project
    let created_project = fixture
        .backend
        .create_project(user_id)
        .await
        .expect("Failed to create project");

    // Update project file with test content
    let project_bytes = b"test project file content";
    fixture
        .backend
        .update_project_file(&created_project.id, project_bytes)
        .await
        .expect("Failed to update project file");

    // Test reading the project file
    let result = Backend::read_project_file(&fixture.backend, &created_project.id).await;
    assert!(result.is_ok(), "Failed to read project file: {:?}", result.err());

    let retrieved_bytes = result.unwrap();
    assert_eq!(
        retrieved_bytes, project_bytes,
        "Retrieved project file content should match original"
    );
}

#[tokio::test]
async fn test_read_project_file_project_not_found() {
    let fixture = Fixture::new();

    // Test reading project file from non-existent project
    let result = Backend::read_project_file(&fixture.backend, "nonexistent_project_id").await;
    assert!(result.is_err(), "Should fail when project doesn't exist");

    let error = result.unwrap_err();
    assert!(error
        .to_string()
        .contains("Project nonexistent_project_id does not exist"));
}

#[tokio::test]
async fn test_read_project_file_not_found() {
    let fixture = Fixture::new();
    let user_id = "test_user";

    // Create a project without updating project file
    let created_project = fixture
        .backend
        .create_project(user_id)
        .await
        .expect("Failed to create project");

    // Test reading non-existent project file
    let result = Backend::read_project_file(&fixture.backend, &created_project.id).await;
    assert!(result.is_err(), "Should fail when project file doesn't exist");
}

#[tokio::test]
async fn test_read_project_file_binary_content() {
    let fixture = Fixture::new();
    let user_id = "test_user";

    // Create a project
    let created_project = fixture
        .backend
        .create_project(user_id)
        .await
        .expect("Failed to create project");

    // Create binary content with various byte values
    let binary_content: Vec<u8> = (0..=255).cycle().take(2000).collect();
    fixture
        .backend
        .update_project_file(&created_project.id, &binary_content)
        .await
        .expect("Failed to update project file with binary content");

    // Test reading the binary content
    let result = Backend::read_project_file(&fixture.backend, &created_project.id).await;
    assert!(result.is_ok(), "Should handle binary content");

    let retrieved_content = result.unwrap();
    assert_eq!(
        retrieved_content, binary_content,
        "Binary content should be preserved exactly"
    );
}

#[tokio::test]
async fn test_read_project_file_empty_content() {
    let fixture = Fixture::new();
    let user_id = "test_user";

    // Create a project
    let created_project = fixture
        .backend
        .create_project(user_id)
        .await
        .expect("Failed to create project");

    // Update project file with empty content
    let empty_content = b"";
    fixture
        .backend
        .update_project_file(&created_project.id, empty_content)
        .await
        .expect("Failed to update project file with empty content");

    // Test reading the empty content
    let result = Backend::read_project_file(&fixture.backend, &created_project.id).await;
    assert!(result.is_ok(), "Should handle empty content");

    let retrieved_content = result.unwrap();
    assert_eq!(retrieved_content, empty_content, "Empty content should be preserved");
}
