use std::sync::Arc;
use tokio::sync::Mutex;

use crate::backend::Auth;

use super::{Backend, DbProject};
use anyhow::{Context, Result};
use log::warn;
use reqwest::Response;
use serde::{Deserialize, Serialize};

pub struct PocketbaseBackend {
    host: String,
    auth: Arc<Mutex<dyn Auth + Send + Sync>>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PocketbaseProject {
    id: String,
    name: String,
    user_id: String,
    project: String,
    samples: Vec<String>,
    created: chrono::DateTime<chrono::Utc>,
    updated: chrono::DateTime<chrono::Utc>,
}

impl PocketbaseBackend {
    pub fn new(host: String, auth: Arc<Mutex<dyn Auth + Send + Sync>>) -> Self {
        Self { host, auth }
    }

    async fn get_token(&self) -> Result<String> {
        self.auth.lock().await.token().ok_or(anyhow::anyhow!("Not logged in"))
    }

    async fn update_project(&self, project_id: &str, form: reqwest::multipart::Form) -> Result<DbProject> {
        let token = self.get_token().await?;
        let url = format!("{}/api/collections/projects/records/{}", self.host, project_id);
        let client = reqwest::Client::new();
        let response = client
            .patch(&url)
            .header("Accept", "application/json")
            .bearer_auth(token)
            .multipart(form)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(handle_error_response(response, "Update Project").await);
        }

        Ok(response.json::<DbProject>().await?)
    }

    async fn read_project_file(&self, project_id: &str, project_filename: &str) -> Result<Vec<u8>> {
        let token = self.get_token().await?;
        let url = format!("{}/api/files/projects/{}/{}", self.host, project_id, project_filename);
        let client = reqwest::Client::new();

        let response = client
            .get(&url)
            .header("Accept", "application/json")
            .bearer_auth(token)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(handle_error_response(response, "Get Project File").await);
        }

        Ok(response.bytes().await?.to_vec())
    }
}

#[async_trait::async_trait]
impl Backend for PocketbaseBackend {
    async fn get_projects(&self) -> Result<Vec<DbProject>> {
        let token = self.get_token().await?;

        let url = format!("{}/api/collections/projects/records", self.host);
        let client = reqwest::Client::new();

        let response = client
            .get(&url)
            .header("Accept", "application/json")
            .bearer_auth(token)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(handle_error_response(response, "Get Projects").await);
        }

        let json: serde_json::Value = response.json().await?;
        let projects = json["items"]
            .as_array()
            .ok_or(anyhow::anyhow!("Missing items in response"))?;
        let projects = projects
            .iter()
            .filter_map(|item| {
                let project = serde_json::from_value(item.clone())
                    .context(anyhow::anyhow!("Unable to parse project in response"));
                match project {
                    Ok(project) => Some(project),
                    Err(error) => {
                        warn!("Failed to parse project (item = {item:?}, error = {error})");
                        None
                    }
                }
            })
            .collect();
        Ok(projects)
    }

    async fn read_project(&self, project_id: &str) -> Result<DbProject> {
        let token = self.get_token().await?;

        let url = format!("{}/api/collections/projects/records/{}", self.host, project_id);
        let client = reqwest::Client::new();

        let response = client
            .get(&url)
            .header("Accept", "application/json")
            .bearer_auth(token)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(handle_error_response(response, "Get Project").await);
        }

        Ok(response.json::<DbProject>().await?)
    }

    async fn create_project(&self, user_id: &str) -> Result<DbProject> {
        let token = self.get_token().await?;

        let url = format!("{}/api/collections/projects/records", self.host);
        let client = reqwest::Client::new();

        let project_json = serde_json::json!({
            "userId": user_id
        });

        let response = client
            .post(&url)
            .header("Accept", "application/json")
            .bearer_auth(token)
            .json(&project_json)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(handle_error_response(response, "Create Project").await);
        }

        Ok(response.json::<DbProject>().await?)
    }

    async fn update_project_name(&self, project_id: &str, name: &str) -> Result<DbProject> {
        let form = reqwest::multipart::Form::new().text("name", name.to_string());
        self.update_project(project_id, form).await
    }

    async fn update_project_file(&self, project_id: &str, project_bytes: &[u8]) -> Result<DbProject> {
        let form = reqwest::multipart::Form::new().part(
            "project",
            reqwest::multipart::Part::bytes(project_bytes.to_vec())
                .file_name("project.bin")
                .mime_str("application/octet-stream")
                .unwrap(),
        );
        self.update_project(project_id, form).await
    }

    async fn add_project_sample(&self, project_id: &str, sample_bytes: &[u8], sample_name: &str) -> Result<DbProject> {
        let form = reqwest::multipart::Form::new().part(
            "samples+",
            reqwest::multipart::Part::bytes(sample_bytes.to_vec())
                .file_name(sample_name.to_string())
                .mime_str("audio/wav")
                .unwrap(),
        );
        self.update_project(project_id, form).await
    }

    async fn remove_project_sample(&self, project_id: &str, sample_name: &str) -> Result<DbProject> {
        let form = reqwest::multipart::Form::new().text("samples-", sample_name.to_string());
        self.update_project(project_id, form).await
    }

    async fn remove_project(&self, project_id: &str) -> Result<()> {
        let token = self.get_token().await?;
        let url = format!("{}/api/collections/projects/records/{}", self.host, project_id);
        let client = reqwest::Client::new();
        let response = client
            .delete(&url)
            .header("Accept", "application/json")
            .bearer_auth(token)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(handle_error_response(response, "Remove Project").await);
        }

        Ok(())
    }

    async fn get_samples(&self, project_id: &str) -> Result<Vec<String>> {
        let token = self.get_token().await?;

        let url = format!("{}/api/collections/projects/records/{}", self.host, project_id);
        let client = reqwest::Client::new();

        let response = client
            .get(&url)
            .header("Accept", "application/json")
            .bearer_auth(token)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(handle_error_response(response, "Get Project").await);
        }

        let project = response.json::<PocketbaseProject>().await?;

        Ok(project
            .samples
            .iter()
            .filter_map(|sample| {
                sample.strip_suffix(".wav").map(|s| {
                    // Remove PocketBase ID suffix (e.g. "kick_52iwbgds7l" -> "kick")
                    // PocketBase adds IDs that are usually 10 characters at the end
                    if let Some(underscore_pos) = s.rfind('_') {
                        // Check if what follows the underscore looks like a PocketBase ID
                        let potential_id = &s[underscore_pos + 1..];
                        if potential_id.len() == 10 && potential_id.chars().all(|c| c.is_alphanumeric()) {
                            s[..underscore_pos].to_string()
                        } else {
                            s.to_string()
                        }
                    } else {
                        s.to_string()
                    }
                })
            })
            .collect())
    }

    async fn read_sample(&self, project_id: &str, sample_name: &str) -> Result<Vec<u8>> {
        let token = self.get_token().await?;

        let url = format!("{}/api/collections/projects/records/{}", self.host, project_id);
        let client = reqwest::Client::new();

        let response = client
            .get(&url)
            .header("Accept", "application/json")
            .bearer_auth(token)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(handle_error_response(response, "Get Project").await);
        }

        let project = response.json::<PocketbaseProject>().await?;

        let sample_path = project
            .samples
            .iter()
            .find(|s| s.starts_with(sample_name) && s.ends_with(".wav"))
            .ok_or_else(|| anyhow::anyhow!("Sample not found: {}", sample_name))?;

        self.read_project_file(project_id, sample_path).await
    }

    async fn read_project_file(&self, project_id: &str) -> Result<Vec<u8>> {
        let token = self.get_token().await?;

        let url = format!("{}/api/collections/projects/records/{}", self.host, project_id);
        let client = reqwest::Client::new();

        let response = client
            .get(&url)
            .header("Accept", "application/json")
            .bearer_auth(token)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(handle_error_response(response, "Get Project").await);
        }

        let project = response.json::<PocketbaseProject>().await?;

        self.read_project_file(project_id, &project.project).await
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrorResponse {
    status: u16,
    message: String,
}

impl Default for ErrorResponse {
    fn default() -> Self {
        Self {
            status: 0,
            message: "Unknown error".to_string(),
        }
    }
}

pub(crate) async fn handle_error_response(response: Response, request_name: &str) -> anyhow::Error {
    assert!(!response.status().is_success());

    let response = response.json::<ErrorResponse>().await.unwrap_or_default();

    let error_message = format!(
        "Request failed (request = {}, error code = {}, message = {})",
        request_name, response.status, response.message
    );

    warn!("{}", &error_message);
    anyhow::anyhow!("{}", &error_message)
}
