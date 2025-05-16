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
    async fn update_project(
        &self,
        project_id: &str,
        name: Option<&str>,
        project: Option<&[u8]>,
        samples: Option<&[Vec<u8>]>,
    ) -> Result<DbProject>;
}

pub fn create_pocketbase_backend(host: Option<String>) -> Box<impl Backend> {
    Box::new(pocketbase::PocketbaseBackend::new(host))
}
