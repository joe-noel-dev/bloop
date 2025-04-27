use super::Backend;
use anyhow::Result;
use log::{info, warn};

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
}

#[async_trait::async_trait]
impl Backend for PocketbaseBackend {
    async fn log_in(&mut self, username: String, password: String) -> Result<()> {
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
            let status = response.status();
            let json: serde_json::Value = response.json().await?;
            let error_message = json["message"].as_str().unwrap_or("Unknown error");

            warn!(
                "Failed to log in (error code = {}, message = {})",
                status, error_message
            );

            return Err(anyhow::anyhow!("Failed to log in: {}", error_message));
        }

        let json: serde_json::Value = response.json().await?;
        info!("Logged in as user: {}", json["name"].as_str().unwrap_or("Unknown"));
        self.token = json["token"].as_str().map(|s| s.to_string());
        Ok(())
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
}
