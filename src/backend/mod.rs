mod pocketbase;

pub use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub email: String,
    pub email_visibility: bool,
    pub verified: bool,
    pub name: String,
    pub created: String,
    pub updated: String,
}

#[async_trait::async_trait]
pub trait Backend {
    async fn log_in(&mut self, username: String, password: String) -> Result<User>;
    async fn log_out(&mut self) -> Result<()>;
    async fn get_user(&self, user_id: &str) -> Result<User>;
}

pub fn create_pocketbase_backend(host: Option<String>) -> Box<impl Backend> {
    Box::new(pocketbase::PocketbaseBackend::new(host))
}
