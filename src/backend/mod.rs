#[allow(clippy::module_inception)]
mod backend;
mod pocketbase;

use std::path::Path;

pub use anyhow::Result;
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

pub fn create_pocketbase_backend(host: Option<String>, root_directory: &Path) -> Box<impl Backend> {
    Box::new(pocketbase::PocketbaseBackend::new(host, root_directory))
}
