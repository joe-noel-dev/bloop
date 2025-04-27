mod pocketbase;

pub use anyhow::Result;

#[async_trait::async_trait]
pub trait Backend {
    async fn log_in(&mut self, username: String, password: String) -> Result<()>;
    async fn log_out(&mut self) -> Result<()>;
}

pub fn create_pocketbase_backend(host: Option<String>) -> Box<impl Backend> {
    Box::new(pocketbase::PocketbaseBackend::new(host))
}
