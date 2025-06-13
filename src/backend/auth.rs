use crate::backend::DbUser;
use anyhow::Result;

#[async_trait::async_trait]
pub trait Auth {
    async fn log_in(&mut self, username: String, password: String) -> Result<DbUser>;
    async fn log_out(&mut self) -> Result<()>;
    async fn refresh_auth(&mut self) -> Result<DbUser>;
    async fn get_user(&self, user_id: &str) -> Result<DbUser>;
    fn token(&self) -> Option<String>;
}
