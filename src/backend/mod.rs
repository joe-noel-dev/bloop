mod pocketbase;

pub use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DbUser {
    pub id: String,
    pub email: String,
    pub email_visibility: bool,
    pub verified: bool,
    pub name: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DbProject {
    pub id: String,
    pub name: String,
    pub user_id: String,
    pub project: String,
    pub samples: Vec<String>,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[async_trait::async_trait]
pub trait Backend {
    async fn log_in(&mut self, username: String, password: String) -> Result<DbUser>;
    async fn log_out(&mut self) -> Result<()>;
    async fn get_user(&self, user_id: &str) -> Result<DbUser>;

    async fn get_projects(&self) -> Result<Vec<DbProject>>;
    async fn get_project(&self, project_id: &str) -> Result<DbProject>;
    async fn create_project(&self, user_id: &str) -> Result<DbProject>;
    async fn update_project_name(&self, project_id: &str, name: &str) -> Result<DbProject>;
    async fn update_project_file(&self, project_id: &str, project_bytes: &[u8]) -> Result<DbProject>;
    async fn add_project_sample(&self, project_id: &str, sample_bytes: &[u8], sample_name: &str) -> Result<DbProject>;
    async fn remove_project_sample(&self, project_id: &str, sample_name: &str) -> Result<DbProject>;
    async fn remove_project(&self, project_id: &str) -> Result<()>;

    async fn get_project_file(&self, project_id: &str, project_filename: &str) -> Result<Vec<u8>>;
}

pub fn create_pocketbase_backend(host: Option<String>) -> Box<impl Backend> {
    Box::new(pocketbase::PocketbaseBackend::new(host))
}
