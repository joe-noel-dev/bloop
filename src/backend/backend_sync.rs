use std::sync::Arc;

use crate::backend::Backend;
use anyhow::Result;

#[allow(dead_code)]
pub struct BackendSync {
    local: Arc<dyn Backend + Send + Sync>,
    remote: Arc<dyn Backend + Send + Sync>,
}

impl BackendSync {
    pub fn new(local: Arc<dyn Backend + Send + Sync>, remote: Arc<dyn Backend + Send + Sync>) -> Self {
        BackendSync { local, remote }
    }

    pub async fn push_project(&self, user_id: &str, project_id: &str) -> Result<()> {
        let local_project = self.local.read_project(project_id).await?;

        let remote_project = match self.remote.read_project(project_id).await {
            Ok(project) => Some(project),
            Err(_) => None,
        };

        // If remote project exists and has the same last update time, do nothing
        if let Some(remote) = &remote_project {
            if remote.updated == local_project.updated {
                return Ok(());
            }
        }

        // Create or update the remote project
        let remote_project = if let Some(remote) = remote_project {
            remote
        } else {
            self.remote.create_project(user_id).await?
        };

        // Update metadata (name, updated timestamp)
        if remote_project.name != local_project.name {
            self.remote.update_project_name(project_id, &local_project.name).await?;
        }

        // Sync samples
        self.push_samples(project_id).await?;

        // Update the project file if it exists locally
        if let Ok(project_bytes) = self.local.read_project_file(project_id).await {
            self.remote.update_project_file(project_id, &project_bytes).await?;
        }

        Ok(())
    }

    async fn push_samples(&self, project_id: &str) -> Result<()> {
        let local_samples = self.local.get_samples(project_id).await?;
        let remote_samples = self.remote.get_samples(project_id).await?;

        // Add samples that exist locally but not remotely
        for sample in local_samples.iter() {
            if !remote_samples.contains(sample) {
                // Upload the sample to the remote backend
                let sample_bytes = self.local.read_sample(project_id, sample).await?;
                self.remote
                    .add_project_sample(project_id, &sample_bytes, sample)
                    .await?;
            }
        }

        // Remove samples that exist remotely but not locally
        for sample in remote_samples.iter() {
            if !local_samples.contains(sample) {
                // Remove the sample from the remote backend
                self.remote.remove_project_sample(project_id, sample).await?;
            }
        }

        Ok(())
    }

    #[allow(unused_variables)]
    pub async fn pull_project(&self, project_id: &str) -> Result<()> {
        unimplemented!()
    }
}
