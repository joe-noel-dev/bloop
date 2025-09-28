use anyhow::Result;
use log::{error, info};
use protobuf_json_mapping::print_to_string;
use std::{sync::Once, thread};
use tokio::time::{timeout, Duration};

use bloop::{
    bloop::{Request, Response},
    run_core, AppConfig,
};

use crate::common::{Mocketbase, LocalPocketbase, TestUser};

pub struct EnhancedIntegrationFixture {
    _home_directory: tempfile::TempDir,
    core_thread: Option<thread::JoinHandle<()>>,
    request_tx: tokio::sync::mpsc::Sender<Request>,
    response_rx: tokio::sync::broadcast::Receiver<bloop::bloop::Response>,
    _response_logger: tokio::task::JoinHandle<()>,
    backend: BackendType,
}

enum BackendType {
    Mock(Mocketbase),
    Local(LocalPocketbase),
}

impl BackendType {
    fn uri(&self) -> String {
        match self {
            BackendType::Mock(mock) => mock.uri(),
            BackendType::Local(local) => local.url(),
        }
    }
}

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

impl EnhancedIntegrationFixture {
    pub async fn new() -> Self {
        Self::new_with_backend(false).await
    }

    pub async fn new_with_real_pocketbase() -> Self {
        Self::new_with_backend(true).await
    }

    async fn new_with_backend(use_real_pocketbase: bool) -> Self {
        init_logger();

        // Set environment variable to use dummy audio for tests
        std::env::set_var("BLOOP_DUMMY_AUDIO", "1");

        let home_directory = tempfile::TempDir::new().expect("Unable to create temporary directory");
        println!(
            "Using temporary directory for home: {}",
            home_directory.path().display()
        );

        let backend = if use_real_pocketbase {
            let mut local_pb = LocalPocketbase::new().await.expect("Failed to create LocalPocketbase");
            local_pb.start().await.expect("Failed to start LocalPocketbase");
            BackendType::Local(local_pb)
        } else {
            BackendType::Mock(Mocketbase::new().await)
        };

        let app_config = AppConfig::default()
            .with_api_url(backend.uri())
            .with_root_directory(home_directory.path().to_path_buf())
            .with_use_dummy_audio(true)
            .with_use_midi(false);

        let (request_tx, request_rx) = tokio::sync::mpsc::channel(100);
        let (response_tx, response_rx) = tokio::sync::broadcast::channel(100);
        let core_thread = run_core(request_rx, request_tx.clone(), response_tx.clone(), app_config);

        let mut log_rx = response_rx.resubscribe();
        let _response_logger = tokio::spawn(async move {
            while let Ok(response) = log_rx.recv().await {
                match print_to_string(&response) {
                    Ok(message) => info!("API response: {message}"),
                    Err(error) => error!("Error logging response: {error}"),
                }
            }
        });

        Self {
            _home_directory: home_directory,
            core_thread: Some(core_thread),
            request_tx,
            response_rx,
            _response_logger,
            backend,
        }
    }

    pub async fn send_request(&self, request: Request) {
        self.request_tx
            .send(request)
            .await
            .expect("Failed to send request");
    }

    pub async fn wait_for_response<F>(&mut self, predicate: F) -> Result<Response>
    where
        F: Fn(&Response) -> bool,
    {
        let result = timeout(Duration::from_secs(5), async move {
            loop {
                let response = self.response_rx.recv().await?;
                if predicate(&response) {
                    return Ok(response);
                }
            }
        })
        .await;

        match result {
            Ok(inner_result) => inner_result,
            Err(_) => Err(anyhow::anyhow!("Timeout waiting for response")),
        }
    }

    pub fn mocketbase(&mut self) -> &mut Mocketbase {
        match &mut self.backend {
            BackendType::Mock(mock) => mock,
            BackendType::Local(_) => panic!("This fixture is using real PocketBase, not Mocketbase"),
        }
    }

    pub fn local_pocketbase(&mut self) -> &mut LocalPocketbase {
        match &mut self.backend {
            BackendType::Local(local) => local,
            BackendType::Mock(_) => panic!("This fixture is using Mocketbase, not real PocketBase"),
        }
    }

    pub async fn add_test_user(&mut self, email: &str, password: &str, name: &str) -> Result<()> {
        match &mut self.backend {
            BackendType::Mock(mock) => {
                let user = crate::common::MockUser::new("test-user-id", email, password, name);
                mock.add_user(user).await;
                Ok(())
            }
            BackendType::Local(local) => {
                let user = TestUser::new("test-user-id", email, password, name);
                local.add_user(user).await
            }
        }
    }
}

impl Drop for EnhancedIntegrationFixture {
    fn drop(&mut self) {
        // Clean up LocalPocketbase if we're using it
        if let BackendType::Local(ref mut local_pb) = self.backend {
            let _ = futures::executor::block_on(local_pb.stop());
        }
        self.core_thread = None;
    }
}