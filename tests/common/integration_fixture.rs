use anyhow::Result;
use protobuf_json_mapping::print_to_string;
use std::thread;
use tokio::time::{timeout, Duration};

use bloop::{
    bloop::{Request, Response},
    run_core,
};

use crate::common::BackendFixture;

pub struct IntegrationFixture {
    _home_directory: tempfile::TempDir,
    core_thread: Option<thread::JoinHandle<()>>,
    request_tx: tokio::sync::mpsc::Sender<Request>,
    response_rx: tokio::sync::broadcast::Receiver<bloop::bloop::Response>,
    _response_logger: tokio::task::JoinHandle<()>,
    backend_fixture: BackendFixture,
}

impl IntegrationFixture {
    pub fn new() -> Self {
        let home_directory = tempfile::TempDir::new().expect("Unable to create temporary directory");
        println!(
            "Using temporary directory for home: {}",
            home_directory.path().display()
        );

        let backend_fixture = BackendFixture::new();

        let (request_tx, request_rx) = tokio::sync::mpsc::channel(100);
        let (response_tx, response_rx) = tokio::sync::broadcast::channel(100);
        let core_thread = run_core(
            home_directory.path().to_path_buf(),
            backend_fixture.mock_server.base_url(),
            request_rx,
            request_tx.clone(),
            response_tx.clone(),
        );

        let mut log_rx = response_rx.resubscribe();
        let _response_logger = tokio::spawn(async move {
            while let Ok(response) = log_rx.recv().await {
                match print_to_string(&response) {
                    Ok(message) => println!("{}", message),
                    Err(error) => eprintln!("Error logging response: {}", error),
                }
            }
        });

        Self {
            _home_directory: home_directory,
            core_thread: Some(core_thread),
            request_tx,
            response_rx,
            _response_logger,
            backend_fixture,
        }
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
