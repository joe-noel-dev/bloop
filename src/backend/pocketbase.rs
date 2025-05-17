use super::{Backend, DbProject, DbUser};
use anyhow::{Context, Result};
use log::{info, warn};
use reqwest::Response;
use serde::{Deserialize, Serialize};

const DEFAULT_HOST: &str = "https://joe-noel-dev-bloop.fly.dev/";

pub struct PocketbaseBackend {
    host: String,
    token: Option<String>,
}

impl PocketbaseBackend {
    pub fn new(host: Option<String>) -> Self {
        Self {
            host: host.unwrap_or(String::from(DEFAULT_HOST)),
            token: None,
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

        if !response.status().is_success() {
            return Err(handle_error_response(response, "Log In").await);
        }

        let json: serde_json::Value = response.json().await?;
        info!("Logged in as user: {}", json["name"].as_str().unwrap_or("Unknown"));
        self.token = json["token"].as_str().map(|s| s.to_string());

        let user = json
            .get("record")
            .ok_or(anyhow::anyhow!("Missing record in response"))?;
        let user = serde_json::from_value(user.clone()).context(anyhow::anyhow!("Unable to parse user in response"))?;
        Ok(user)
    }

    async fn log_out(&mut self) -> anyhow::Result<()> {
        if self.token.is_none() {
            warn!("No user is logged in");
            return Ok(());
        }

        info!("Logging out");
        self.token = None;
        Ok(())
    }

    async fn get_user(&self, user_id: &str) -> Result<DbUser> {
        let url = format!("{}/api/collections/users/records/{}", self.host, user_id);
        let client = reqwest::Client::new();

        let response = client.get(&url).header("Accept", "application/json").send().await?;

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
