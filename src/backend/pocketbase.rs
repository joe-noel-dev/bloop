use std::path::{Path, PathBuf};

use super::{Backend, DbProject, DbUser};
use anyhow::{Context, Result};
use log::{info, warn};
use reqwest::Response;
use serde::{Deserialize, Serialize};

const DEFAULT_HOST: &str = "https://joe-noel-dev-bloop.fly.dev";

pub struct PocketbaseBackend {
    host: String,
    token: Option<String>,
    root_directory: PathBuf,
}

impl PocketbaseBackend {
    pub fn new(host: Option<String>, root_directory: &Path) -> Self {
        let token = Self::read_token(root_directory);
        Self {
            host: host.unwrap_or(String::from(DEFAULT_HOST)),
            token,
            root_directory: root_directory.to_path_buf(),
        }
    }

    async fn update_project(&self, project_id: &str, form: reqwest::multipart::Form) -> Result<DbProject> {
        let token = self.token.as_ref().ok_or(anyhow::anyhow!("Not logged in"))?;
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

    fn read_token(root_directory: &Path) -> Option<String> {
        let token_path = root_directory.join("token");
        std::fs::read_to_string(&token_path).ok().map(|s| s.trim().to_string())
    }

    fn write_token(root_directory: &Path, token: &str) {
        if !root_directory.exists() {
            if let Err(e) = std::fs::create_dir_all(root_directory) {
                warn!("Failed to create directory: {}", e);
                return;
            }
        }

        let token_path = root_directory.join("token");
        if let Err(e) = std::fs::write(&token_path, token) {
            warn!("Failed to write token to disk: {}", e);
        }
    }

    async fn handle_login(&mut self, response: Response, request_name: &str) -> Result<DbUser> {
        if !response.status().is_success() {
            return Err(handle_error_response(response, request_name).await);
        }

        let json: serde_json::Value = response.json().await?;
        self.token = json["token"].as_str().map(|s| s.to_string());

        if let Some(token) = &self.token {
            Self::write_token(&self.root_directory, token);
        }

        let user = json
            .get("record")
            .ok_or(anyhow::anyhow!("Missing record in response"))?;
        let user: DbUser =
            serde_json::from_value(user.clone()).context(anyhow::anyhow!("Unable to parse user in response"))?;
        info!("Logged in a user: {}", user.name);
        Ok(user)
    }
}

#[async_trait::async_trait]
impl Backend for PocketbaseBackend {
    async fn log_in(&mut self, username: String, password: String) -> Result<DbUser> {
        let client = reqwest::Client::new();
        let url = format!("{}/api/collections/users/auth-with-password", self.host);

        let response = client
            .post(&url)
            .json(&serde_json::json!({
                "identity": username,
                "password": password,
            }))
            .send()
            .await?;

        self.handle_login(response, "Log in").await
    }

    async fn log_out(&mut self) -> anyhow::Result<()> {
        if self.token.is_none() {
            warn!("No user is logged in");
            return Ok(());
        }

        info!("Logging out");
        self.token = None;

        let token_path = self.root_directory.join("token");
        if let Err(e) = std::fs::remove_file(&token_path) {
            warn!("Failed to remove token file: {}", e);
        } else {
            info!("Token file removed successfully");
        }

        Ok(())
    }

    async fn refresh_auth(&mut self) -> Result<DbUser> {
        if self.token.is_none() {
            return Err(anyhow::anyhow!("Not logged in"));
        }

        let client = reqwest::Client::new();
        let url = format!("{}/api/collections/users/auth-refresh", self.host);

        let response = client
            .post(&url)
            .header("Accept", "application/json")
            .bearer_auth(self.token.as_ref().unwrap())
            .send()
            .await?;

        self.handle_login(response, "Auth refresh").await
    }

    async fn get_user(&self, user_id: &str) -> Result<DbUser> {
        let token = self.token.as_ref().ok_or(anyhow::anyhow!("Not logged in"))?;

        let url = format!("{}/api/collections/users/records/{}", self.host, user_id);
        let client = reqwest::Client::new();

        let response = client
            .get(&url)
            .header("Accept", "application/json")
            .bearer_auth(token)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(handle_error_response(response, "Get User").await);
        }

        Ok(response.json::<DbUser>().await?)
    }

    async fn get_projects(&self) -> Result<Vec<DbProject>> {
        let token = self.token.as_ref().ok_or(anyhow::anyhow!("Not logged in"))?;

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
                        warn!("Failed to parse project (item = {:?}, error = {})", item, error);
                        None
                    }
                }
            })
            .collect();
        Ok(projects)
    }

    async fn get_project(&self, project_id: &str) -> Result<DbProject> {
        let token = self.token.as_ref().ok_or(anyhow::anyhow!("Not logged in"))?;

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
        let token = self.token.as_ref().ok_or(anyhow::anyhow!("Not logged in"))?;

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
        let token = self.token.as_ref().ok_or(anyhow::anyhow!("Not logged in"))?;
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

    async fn get_project_file(&self, project_id: &str, project_filename: &str) -> Result<Vec<u8>> {
        let token = self.token.as_ref().ok_or(anyhow::anyhow!("Not logged in"))?;
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

async fn handle_error_response(response: Response, request_name: &str) -> anyhow::Error {
    assert!(!response.status().is_success());

    let response = response.json::<ErrorResponse>().await.unwrap_or_default();

    let error_message = format!(
        "Request failed (request = {}, error code = {}, message = {})",
        request_name, response.status, response.message
    );

    warn!("{}", &error_message);
    anyhow::anyhow!("{}", &error_message)
}
