mod auth;
#[allow(clippy::module_inception)]
mod backend;
mod pocketbase;

use std::{path::Path, sync::Arc};
use tokio::sync::Mutex;

pub use anyhow::Result;
pub use auth::Auth;
pub use backend::Backend;
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

pub fn create_pocketbase_auth(host: Option<String>, root_directory: &Path) -> Arc<Mutex<dyn Auth + Send + Sync>> {
    Arc::new(Mutex::new(pocketbase::PocketbaseAuth::new(host, root_directory)))
}

pub fn create_pocketbase_backend(host: Option<String>, auth: Arc<Mutex<dyn Auth + Send + Sync>>) -> Box<impl Backend> {
    Box::new(pocketbase::PocketbaseBackend::new(host, auth))
}
