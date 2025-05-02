use std::sync::Once;

use bloop::backend::{create_pocketbase_backend, Backend};
use httpmock::MockServer;

static INIT: Once = Once::new();

fn init_logger() {
    INIT.call_once(|| {
        env_logger::builder()
            .is_test(true)
            .filter_level(log::LevelFilter::Debug)
            .try_init()
            .ok();
    });
}

pub struct BackendFixture {
    pub mock_server: MockServer,
    pub backend: Box<dyn Backend>,
}

impl BackendFixture {
    pub fn new() -> Self {
        init_logger();
        let mock_server = MockServer::start();
        let base_url = mock_server.base_url();
        let backend = create_pocketbase_backend(Some(base_url));

        Self { mock_server, backend }
    }
}
