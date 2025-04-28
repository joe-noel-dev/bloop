use super::{Backend, User};
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
}

#[async_trait::async_trait]
impl Backend for PocketbaseBackend {
    async fn log_in(&mut self, username: String, password: String) -> Result<User> {
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

    async fn get_user(&self, user_id: &str) -> Result<User> {
        let url = format!("{}/api/collections/users/records/{}", self.host, user_id);
        let client = reqwest::Client::new();

        let response = client.get(&url).header("Accept", "application/json").send().await?;

        if !response.status().is_success() {
            return Err(handle_error_response(response, "Get User").await);
        }

        Ok(response.json::<User>().await?)
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
