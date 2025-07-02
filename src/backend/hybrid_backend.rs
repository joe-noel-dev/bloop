use std::sync::Arc;

use crate::backend::{Backend, DbProject};
use anyhow::Result;

struct HybridBackend {
    local: Arc<dyn Backend + Send + Sync>,
    remote: Arc<dyn Backend + Send + Sync>,
}

impl HybridBackend {
    pub fn new(local: Arc<dyn Backend + Send + Sync>, remote: Arc<dyn Backend + Send + Sync>) -> Self {
        HybridBackend { local, remote }
    }
}

#[async_trait::async_trait]
impl Backend for HybridBackend {
    async fn get_projects(&self) -> Result<Vec<DbProject>> {
        // Get local projects
        let local_projects = self.local.get_projects().await?;

        // Try to get remote projects, but ignore errors (e.g. if offline)
        let remote_projects = self.remote.get_projects().await.unwrap_or_default();

        // Create a map to merge projects by ID, preferring the latest one
        let mut projects_map = std::collections::HashMap::new();

        // Add local projects first
        for project in local_projects {
            projects_map.insert(project.id.clone(), project);
        }

        // Add remote projects, replacing local ones if remote is newer
        for project in remote_projects {
            match projects_map.get(&project.id) {
                Some(existing) => {
                    // Compare timestamps and keep the more recent one
                    if project.updated > existing.updated {
                        projects_map.insert(project.id.clone(), project);
                    }
                    // If existing is newer or equal, keep it (no action needed)
                }
                None => {
                    // No conflict, add the remote project
                    projects_map.insert(project.id.clone(), project);
                }
            }
        }

        // Convert map values back to vector
        let mut projects: Vec<DbProject> = projects_map.into_values().collect();

        // Sort by updated timestamp, most recent first
        projects.sort_by(|a, b| b.updated.cmp(&a.updated));

        Ok(projects)
    }

    async fn read_project(&self, project_id: &str) -> Result<DbProject> {
        unimplemented!()
    }

    async fn create_project(&self, user_id: &str) -> Result<DbProject> {
        unimplemented!()
    }

    async fn update_project_name(&self, project_id: &str, name: &str) -> Result<DbProject> {
        unimplemented!()
    }

    async fn update_project_file(&self, project_id: &str, project_bytes: &[u8]) -> Result<DbProject> {
        unimplemented!()
    }

    async fn add_project_sample(&self, project_id: &str, sample_bytes: &[u8], sample_name: &str) -> Result<DbProject> {
        unimplemented!()
    }

    async fn remove_project_sample(&self, project_id: &str, sample_name: &str) -> Result<DbProject> {
        unimplemented!()
    }

    async fn remove_project(&self, project_id: &str) -> Result<()> {
        unimplemented!()
    }

    async fn read_project_file(&self, project_id: &str, project_filename: &str) -> Result<Vec<u8>> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Utc};
    use std::sync::Arc;

    // Mock backend that returns predefined projects
    struct MockBackend {
        projects: Vec<DbProject>,
        should_fail: bool,
    }

    impl MockBackend {
        fn new(projects: Vec<DbProject>) -> Self {
            Self {
                projects,
                should_fail: false,
            }
        }

        fn new_failing() -> Self {
            Self {
                projects: Vec::new(),
                should_fail: true,
            }
        }
    }

    #[async_trait::async_trait]
    impl Backend for MockBackend {
        async fn get_projects(&self) -> Result<Vec<DbProject>> {
            if self.should_fail {
                return Err(anyhow::anyhow!("Backend unavailable"));
            }
            Ok(self.projects.clone())
        }

        async fn read_project(&self, _project_id: &str) -> Result<DbProject> {
            unimplemented!()
        }

        async fn create_project(&self, _user_id: &str) -> Result<DbProject> {
            unimplemented!()
        }

        async fn update_project_name(&self, _project_id: &str, _name: &str) -> Result<DbProject> {
            unimplemented!()
        }

        async fn update_project_file(&self, _project_id: &str, _project_bytes: &[u8]) -> Result<DbProject> {
            unimplemented!()
        }

        async fn add_project_sample(
            &self,
            _project_id: &str,
            _sample_bytes: &[u8],
            _sample_name: &str,
        ) -> Result<DbProject> {
            unimplemented!()
        }

        async fn remove_project_sample(&self, _project_id: &str, _sample_name: &str) -> Result<DbProject> {
            unimplemented!()
        }

        async fn remove_project(&self, _project_id: &str) -> Result<()> {
            unimplemented!()
        }

        async fn read_project_file(&self, _project_id: &str, _project_filename: &str) -> Result<Vec<u8>> {
            unimplemented!()
        }
    }

    fn create_test_project(id: &str, name: &str, updated: DateTime<Utc>) -> DbProject {
        DbProject {
            id: id.to_string(),
            name: name.to_string(),
            user_id: "test_user".to_string(),
            project: "project.bin".to_string(),
            samples: Vec::new(),
            created: updated,
            updated,
        }
    }

    #[tokio::test]
    async fn test_get_projects_only_local() {
        let now = Utc::now();
        let local_projects = vec![
            create_test_project("proj1", "Local Project 1", now),
            create_test_project("proj2", "Local Project 2", now),
        ];

        let local_backend = Arc::new(MockBackend::new(local_projects.clone()));
        let remote_backend = Arc::new(MockBackend::new(Vec::new()));

        let hybrid = HybridBackend::new(local_backend, remote_backend);
        let result = hybrid.get_projects().await.unwrap();

        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|p| p.id == "proj1"));
        assert!(result.iter().any(|p| p.id == "proj2"));
    }

    #[tokio::test]
    async fn test_get_projects_only_remote() {
        let now = Utc::now();
        let remote_projects = vec![
            create_test_project("proj3", "Remote Project 1", now),
            create_test_project("proj4", "Remote Project 2", now),
        ];

        let local_backend = Arc::new(MockBackend::new(Vec::new()));
        let remote_backend = Arc::new(MockBackend::new(remote_projects.clone()));

        let hybrid = HybridBackend::new(local_backend, remote_backend);
        let result = hybrid.get_projects().await.unwrap();

        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|p| p.id == "proj3"));
        assert!(result.iter().any(|p| p.id == "proj4"));
    }

    #[tokio::test]
    async fn test_get_projects_no_conflicts() {
        let now = Utc::now();
        let local_projects = vec![create_test_project("proj1", "Local Project 1", now)];
        let remote_projects = vec![create_test_project("proj2", "Remote Project 1", now)];

        let local_backend = Arc::new(MockBackend::new(local_projects));
        let remote_backend = Arc::new(MockBackend::new(remote_projects));

        let hybrid = HybridBackend::new(local_backend, remote_backend);
        let result = hybrid.get_projects().await.unwrap();

        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|p| p.id == "proj1"));
        assert!(result.iter().any(|p| p.id == "proj2"));
    }

    #[tokio::test]
    async fn test_get_projects_prefer_newer_remote() {
        let base_time = Utc::now();
        let older_time = base_time - chrono::Duration::seconds(3600); // 1 hour ago
        let newer_time = base_time;

        let local_projects = vec![create_test_project("proj1", "Local Project (older)", older_time)];
        let remote_projects = vec![create_test_project("proj1", "Remote Project (newer)", newer_time)];

        let local_backend = Arc::new(MockBackend::new(local_projects));
        let remote_backend = Arc::new(MockBackend::new(remote_projects));

        let hybrid = HybridBackend::new(local_backend, remote_backend);
        let result = hybrid.get_projects().await.unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, "proj1");
        assert_eq!(result[0].name, "Remote Project (newer)");
        assert_eq!(result[0].updated, newer_time);
    }

    #[tokio::test]
    async fn test_get_projects_prefer_newer_local() {
        let base_time = Utc::now();
        let older_time = base_time - chrono::Duration::seconds(3600); // 1 hour ago
        let newer_time = base_time;

        let local_projects = vec![create_test_project("proj1", "Local Project (newer)", newer_time)];
        let remote_projects = vec![create_test_project("proj1", "Remote Project (older)", older_time)];

        let local_backend = Arc::new(MockBackend::new(local_projects));
        let remote_backend = Arc::new(MockBackend::new(remote_projects));

        let hybrid = HybridBackend::new(local_backend, remote_backend);
        let result = hybrid.get_projects().await.unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, "proj1");
        assert_eq!(result[0].name, "Local Project (newer)");
        assert_eq!(result[0].updated, newer_time);
    }

    #[tokio::test]
    async fn test_get_projects_remote_failure() {
        let now = Utc::now();
        let local_projects = vec![create_test_project("proj1", "Local Project", now)];

        let local_backend = Arc::new(MockBackend::new(local_projects));
        let remote_backend = Arc::new(MockBackend::new_failing());

        let hybrid = HybridBackend::new(local_backend, remote_backend);
        let result = hybrid.get_projects().await.unwrap();

        // Should still return local projects even when remote fails
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, "proj1");
        assert_eq!(result[0].name, "Local Project");
    }

    #[tokio::test]
    async fn test_get_projects_local_failure() {
        let now = Utc::now();
        let remote_projects = vec![create_test_project("proj1", "Remote Project", now)];

        let local_backend = Arc::new(MockBackend::new_failing());
        let remote_backend = Arc::new(MockBackend::new(remote_projects));

        let hybrid = HybridBackend::new(local_backend, remote_backend);
        let result = hybrid.get_projects().await;

        // Should fail if local backend fails (as per current implementation)
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_projects_sorted_by_updated() {
        let base_time = Utc::now();
        let time1 = base_time - chrono::Duration::seconds(7200); // 2 hours ago
        let time2 = base_time - chrono::Duration::seconds(3600); // 1 hour ago
        let time3 = base_time; // now

        let local_projects = vec![
            create_test_project("proj1", "Project 1", time1),
            create_test_project("proj3", "Project 3", time3),
        ];
        let remote_projects = vec![create_test_project("proj2", "Project 2", time2)];

        let local_backend = Arc::new(MockBackend::new(local_projects));
        let remote_backend = Arc::new(MockBackend::new(remote_projects));

        let hybrid = HybridBackend::new(local_backend, remote_backend);
        let result = hybrid.get_projects().await.unwrap();

        assert_eq!(result.len(), 3);
        // Should be sorted by updated time, most recent first
        assert_eq!(result[0].id, "proj3"); // most recent
        assert_eq!(result[1].id, "proj2"); // middle
        assert_eq!(result[2].id, "proj1"); // oldest
    }

    #[tokio::test]
    async fn test_get_projects_complex_scenario() {
        let base_time = Utc::now();
        let time1 = base_time - chrono::Duration::seconds(7200); // 2 hours ago
        let time2 = base_time - chrono::Duration::seconds(3600); // 1 hour ago
        let time3 = base_time; // now

        let local_projects = vec![
            create_test_project("proj1", "Local Project 1 (older)", time1),
            create_test_project("proj2", "Local Project 2", time2),
            create_test_project("proj4", "Local Only Project", time3),
        ];
        let remote_projects = vec![
            create_test_project("proj1", "Remote Project 1 (newer)", time3), // Conflicts with local, but newer
            create_test_project("proj3", "Remote Only Project", time2),
        ];

        let local_backend = Arc::new(MockBackend::new(local_projects));
        let remote_backend = Arc::new(MockBackend::new(remote_projects));

        let hybrid = HybridBackend::new(local_backend, remote_backend);
        let result = hybrid.get_projects().await.unwrap();

        assert_eq!(result.len(), 4);

        // Find each project and verify
        let proj1 = result.iter().find(|p| p.id == "proj1").unwrap();
        assert_eq!(proj1.name, "Remote Project 1 (newer)"); // Remote should win

        let proj2 = result.iter().find(|p| p.id == "proj2").unwrap();
        assert_eq!(proj2.name, "Local Project 2"); // Local only

        let proj3 = result.iter().find(|p| p.id == "proj3").unwrap();
        assert_eq!(proj3.name, "Remote Only Project"); // Remote only

        let proj4 = result.iter().find(|p| p.id == "proj4").unwrap();
        assert_eq!(proj4.name, "Local Only Project"); // Local only

        // Verify sorting (most recent first)
        assert!(result[0].updated >= result[1].updated);
        assert!(result[1].updated >= result[2].updated);
        assert!(result[2].updated >= result[3].updated);
    }
}
