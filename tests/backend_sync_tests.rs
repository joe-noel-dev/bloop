use anyhow::Result;
use bloop::backend::{sync_project, Backend, DbProject};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
struct MockBackendProject {
    name: String,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
    user_id: String,
    project_file: Vec<u8>,
    samples: HashMap<String, Vec<u8>>,
}

fn convert_project(id: &str, project: &MockBackendProject) -> DbProject {
    DbProject {
        id: id.to_string(),
        name: project.name.clone(),
        user_id: project.user_id.clone(),
        created: project.created,
        updated: project.updated,
    }
}

struct MockBackend {
    projects: Mutex<HashMap<String, MockBackendProject>>,
    next_id: Mutex<usize>,
}

impl MockBackend {
    async fn next_id(&self) -> String {
        let mut next_id = self.next_id.lock().await;
        let id = next_id.to_string();
        *next_id += 1;
        id
    }
}

#[async_trait::async_trait]
impl Backend for MockBackend {
    async fn get_projects(&self) -> Result<Vec<DbProject>> {
        let projects = self.projects.lock().await;
        let result = projects
            .iter()
            .map(|(id, project)| convert_project(id, project))
            .collect();

        Ok(result)
    }

    async fn read_project(&self, project_id: &str) -> Result<DbProject> {
        let projects = self.projects.lock().await;
        if let Some(project) = projects.get(project_id) {
            Ok(convert_project(project_id, project))
        } else {
            Err(anyhow::anyhow!("Project not found"))
        }
    }

    async fn create_project(&self, user_id: &str) -> Result<DbProject> {
        let project_id = self.next_id().await;
        let now = Utc::now();
        let project = MockBackendProject {
            name: "New Project".to_string(),
            created: now,
            updated: now,
            user_id: user_id.to_string(),
            project_file: Vec::new(),
            samples: HashMap::new(),
        };

        {
            let mut projects = self.projects.lock().await;
            projects.insert(project_id.clone(), project.clone());
        }

        Ok(convert_project(&project_id, &project))
    }

    async fn update_project_name(&self, project_id: &str, name: &str) -> Result<DbProject> {
        let mut projects = self.projects.lock().await;
        if let Some(project) = projects.get_mut(project_id) {
            project.name = name.to_string();
            project.updated = Utc::now();
            Ok(convert_project(project_id, project))
        } else {
            Err(anyhow::anyhow!("Project not found"))
        }
    }

    async fn update_project_file(&self, project_id: &str, project_bytes: &[u8]) -> Result<DbProject> {
        let mut projects = self.projects.lock().await;
        if let Some(project) = projects.get_mut(project_id) {
            project.project_file = project_bytes.to_vec();
            project.updated = Utc::now();
            Ok(convert_project(project_id, project))
        } else {
            Err(anyhow::anyhow!("Project not found"))
        }
    }

    async fn add_project_sample(&self, project_id: &str, sample_bytes: &[u8], sample_name: &str) -> Result<DbProject> {
        let mut projects = self.projects.lock().await;
        if let Some(project) = projects.get_mut(project_id) {
            project.samples.insert(sample_name.to_string(), sample_bytes.to_vec());
            project.updated = Utc::now();
            Ok(convert_project(project_id, project))
        } else {
            Err(anyhow::anyhow!("Project not found"))
        }
    }

    async fn remove_project_sample(&self, project_id: &str, sample_name: &str) -> Result<DbProject> {
        let mut projects = self.projects.lock().await;
        if let Some(project) = projects.get_mut(project_id) {
            project.samples.remove(sample_name);
            project.updated = Utc::now();
            Ok(convert_project(project_id, project))
        } else {
            Err(anyhow::anyhow!("Project not found"))
        }
    }

    async fn remove_project(&self, project_id: &str) -> Result<()> {
        let mut projects = self.projects.lock().await;
        if projects.remove(project_id).is_some() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Project not found"))
        }
    }

    async fn get_samples(&self, project_id: &str) -> Result<Vec<String>> {
        let projects = self.projects.lock().await;
        if let Some(project) = projects.get(project_id) {
            Ok(project.samples.keys().cloned().collect())
        } else {
            Err(anyhow::anyhow!("Project not found"))
        }
    }

    async fn read_sample(&self, project_id: &str, sample_name: &str) -> Result<Vec<u8>> {
        let projects = self.projects.lock().await;
        if let Some(project) = projects.get(project_id) {
            if let Some(sample) = project.samples.get(sample_name) {
                Ok(sample.clone())
            } else {
                Err(anyhow::anyhow!("Sample not found"))
            }
        } else {
            Err(anyhow::anyhow!("Project not found"))
        }
    }

    async fn read_project_file(&self, project_id: &str) -> Result<Vec<u8>> {
        let projects = self.projects.lock().await;
        if let Some(project) = projects.get(project_id) {
            Ok(project.project_file.clone())
        } else {
            Err(anyhow::anyhow!("Project not found"))
        }
    }
}

#[tokio::test]
async fn test_sync_backend_push_project_happy_path() -> Result<()> {
    let local_backend = MockBackend {
        projects: Mutex::new(HashMap::new()),
        next_id: Mutex::new(1),
    };

    let remote_backend = MockBackend {
        projects: Mutex::new(HashMap::new()),
        next_id: Mutex::new(1),
    };

    let user_id = "test_user";

    // Create a project in the local backend
    let local_project = local_backend.create_project(user_id).await?;
    let project_id = &local_project.id;

    // Update the project name
    local_backend.update_project_name(project_id, "My Test Project").await?;

    // Add some project file data
    let project_data = b"test project file content";
    local_backend.update_project_file(project_id, project_data).await?;

    // Add some samples
    let sample1_data = b"sample1 audio data";
    let sample2_data = b"sample2 audio data";
    local_backend
        .add_project_sample(project_id, sample1_data, "sample1")
        .await?;
    local_backend
        .add_project_sample(project_id, sample2_data, "sample2")
        .await?;

    // Sync the project to the remote backend
    sync_project(user_id, project_id, &local_backend, &remote_backend).await?;

    // Verify the project exists in remote backend
    let remote_project = remote_backend.read_project(project_id).await?;
    assert_eq!(remote_project.name, "My Test Project");
    assert_eq!(remote_project.user_id, user_id);

    // Verify project file was synced
    let remote_project_file = remote_backend.read_project_file(project_id).await?;
    assert_eq!(remote_project_file, project_data);

    // Verify samples were synced
    let remote_samples = remote_backend.get_samples(project_id).await?;
    assert_eq!(remote_samples.len(), 2);
    assert!(remote_samples.contains(&"sample1".to_string()));
    assert!(remote_samples.contains(&"sample2".to_string()));

    // Verify sample content
    let remote_sample1 = remote_backend.read_sample(project_id, "sample1").await?;
    let remote_sample2 = remote_backend.read_sample(project_id, "sample2").await?;
    assert_eq!(remote_sample1, sample1_data);
    assert_eq!(remote_sample2, sample2_data);

    Ok(())
}
