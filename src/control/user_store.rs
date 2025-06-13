use std::sync::Arc;
use tokio::sync::Mutex;

use anyhow::Context;
use log::info;

use crate::{backend::Auth, bloop::User};

pub struct UserStore {
    auth: Arc<Mutex<dyn Auth + Send + Sync>>,
}

impl UserStore {
    pub fn new(auth: Arc<Mutex<dyn Auth + Send + Sync>>) -> Self {
        UserStore { auth }
    }

    pub async fn refresh_auth(&mut self) -> anyhow::Result<User> {
        let mut auth = self.auth.lock().await;

        let db_user = auth.refresh_auth().await.context("Error refreshing authentication")?;

        info!("Authentication refreshed successfully");

        Ok(User {
            id: db_user.id,
            name: db_user.name,
            email: db_user.email,
            ..Default::default()
        })
    }

    pub async fn log_in(&mut self, username: String, password: String) -> anyhow::Result<User> {
        let mut auth = self.auth.lock().await;

        let db_user = auth.log_in(username, password).await.context("Error logging in")?;

        info!("Logged in successfully");

        Ok(User {
            id: db_user.id,
            name: db_user.name,
            email: db_user.email,
            ..Default::default()
        })
    }

    pub async fn log_out(&mut self) -> anyhow::Result<()> {
        let mut auth = self.auth.lock().await;

        auth.log_out().await.context("Error logging out")?;

        info!("Logged out successfully");
        Ok(())
    }
}
