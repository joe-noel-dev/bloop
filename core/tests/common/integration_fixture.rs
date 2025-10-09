use anyhow::Result;
use log::{error, info};
use protobuf_json_mapping::print_to_string;
use std::{sync::Once, thread};
use tokio::time::{timeout, Duration};

use bloop::{
    bloop::{Request, Response},
    run_core, AppConfig,
};

use crate::common::Mocketbase;

pub struct IntegrationFixture {
    _home_directory: tempfile::TempDir,
    core_thread: Option<thread::JoinHandle<()>>,
    request_tx: tokio::sync::mpsc::Sender<Request>,
    response_rx: tokio::sync::broadcast::Receiver<bloop::bloop::Response>,
    _response_logger: tokio::task::JoinHandle<()>,
    mocketbase: Mocketbase,
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

impl IntegrationFixture {
    pub async fn new() -> Self {
        init_logger();

        // Set environment variable to use dummy audio for tests
        std::env::set_var("BLOOP_DUMMY_AUDIO", "1");

        let home_directory = tempfile::TempDir::new().expect("Unable to create temporary directory");
        println!(
            "Using temporary directory for home: {}",
            home_directory.path().display()
        );

        let mocketbase = Mocketbase::new().await;

        let app_config = AppConfig::default()
            .with_api_url(mocketbase.uri())
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
            mocketbase,
        }
    }

    pub fn mocketbase(&mut self) -> &mut Mocketbase {
        &mut self.mocketbase
    }

    pub async fn send_request(&self, request: Request) {
        self.request_tx.send(request).await.expect("Failed to send request");
    }

    pub async fn wait_for_response_with_timeout<F>(
        &mut self,
        timeout_duration: Duration,
        predicate: F,
    ) -> Result<Response>
    where
        F: Fn(&Response) -> bool,
    {
        Ok(timeout(timeout_duration, async {
            loop {
                match self.response_rx.recv().await {
                    Ok(response) => {
                        if predicate(&response) {
                            return Ok(response);
                        }
                    }
                    Err(e) => return Err(Box::new(e) as Box<dyn std::error::Error>),
                }
            }
        })
        .await
        .expect("Timeout waiting for response")
        .expect("Error receiving response"))
    }

    pub async fn wait_for_response<F>(&mut self, predicate: F) -> Result<Response>
    where
        F: Fn(&Response) -> bool,
    {
        self.wait_for_response_with_timeout(Duration::from_secs(3), predicate)
            .await
    }
}

impl Drop for IntegrationFixture {
    fn drop(&mut self) {
        self.core_thread = None;
    }
}
