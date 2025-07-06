use std::path::{Path, PathBuf};

use crate::backend::{pocketbase::handle_error_response, Auth};

use super::DbUser;
use anyhow::{Context, Result};
use log::{info, warn};
use reqwest::Response;

pub struct PocketbaseAuth {
    host: String,
    token: Option<String>,
    root_directory: PathBuf,
}

impl PocketbaseAuth {
    pub fn new(host: String, root_directory: &Path) -> Self {
        let token = read_token(root_directory);
        Self {
            host,
            token,
            root_directory: root_directory.to_path_buf(),
        }
    }

    async fn handle_login(&mut self, response: Response, request_name: &str) -> Result<DbUser> {
        if !response.status().is_success() {
            return Err(handle_error_response(response, request_name).await);
        }

        let json: serde_json::Value = response.json().await?;
        self.token = json["token"].as_str().map(|s| s.to_string());

        if let Some(token) = &self.token {
            write_token(&self.root_directory, token);
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
impl Auth for PocketbaseAuth {
    fn token(&self) -> Option<String> {
        self.token.clone()
    }

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
            warn!("Failed to remove token file: {e}");
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
}

fn write_token(root_directory: &Path, token: &str) {
    if !root_directory.exists() {
        if let Err(e) = std::fs::create_dir_all(root_directory) {
            warn!("Failed to create directory: {e}");
            return;
        }
    }

    let token_path = root_directory.join("token");
    if let Err(e) = std::fs::write(&token_path, token) {
        warn!("Failed to write token to disk: {e}");
    }
}

fn read_token(root_directory: &Path) -> Option<String> {
    let token_path = root_directory.join("token");
    std::fs::read_to_string(&token_path).ok().map(|s| s.trim().to_string())
}
