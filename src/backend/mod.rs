mod auth;
#[allow(clippy::module_inception)]
mod backend;
mod backend_sync;
mod filesystem_backend;
mod pocketbase;
mod pocketbase_auth;

use std::{path::Path, sync::Arc};
use tokio::sync::Mutex;

pub use anyhow::Result;
pub use auth::Auth;
pub use backend::Backend;
pub use backend_sync::sync_project;
use chrono::{DateTime, Utc};
pub use filesystem_backend::FilesystemBackend;
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
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

pub fn create_pocketbase_auth(host: String, root_directory: &Path) -> Arc<Mutex<dyn Auth + Send + Sync>> {
    Arc::new(Mutex::new(pocketbase_auth::PocketbaseAuth::new(host, root_directory)))
}

pub fn create_pocketbase_backend(host: String, auth: Arc<Mutex<dyn Auth + Send + Sync>>) -> Arc<impl Backend> {
    Arc::new(pocketbase::PocketbaseBackend::new(host, auth))
}

pub fn create_filesystem_backend(root_directory: &Path) -> Arc<dyn Backend> {
    Arc::new(FilesystemBackend::new(root_directory))
}
