use anyhow::Result;

use super::{DbProject, DbUser};

#[async_trait::async_trait]
pub trait Backend {
    async fn log_in(&mut self, username: String, password: String) -> Result<DbUser>;
    async fn log_out(&mut self) -> Result<()>;
    async fn refresh_auth(&mut self) -> Result<DbUser>;
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
