use protobuf_json_mapping::print_to_string;
use std::thread;
use tokio::time::{timeout, Duration};

use bloop::{
    bloop::{Request, Response},
    run_core,
};

pub struct IntegrationFixture {
    _home_directory: tempfile::TempDir,
    core_thread: Option<thread::JoinHandle<()>>,
    request_tx: tokio::sync::mpsc::Sender<Request>,
    response_rx: tokio::sync::broadcast::Receiver<bloop::bloop::Response>,
    _response_logger: tokio::task::JoinHandle<()>,
}

impl IntegrationFixture {
    pub fn new() -> Self {
        let home_directory = tempfile::TempDir::new().expect("Unable to create temporary directory");
        std::env::set_var("BLOOP_HOME", home_directory.path());

        let (request_tx, request_rx) = tokio::sync::mpsc::channel(100);
        let (response_tx, response_rx) = tokio::sync::broadcast::channel(100);
        let core_thread = run_core(request_rx, request_tx.clone(), response_tx.clone());

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
        }
    }

    pub async fn send_request(&self, request: Request) {
        self.request_tx.send(request).await.expect("Failed to send request");
    }

    pub async fn wait_for_response_with_timeout<F>(
        &mut self,
        timeout_duration: Duration,
        predicate: F,
    ) -> Result<Response, Box<dyn std::error::Error>>
    where
        F: Fn(&Response) -> bool,
    {
        timeout(timeout_duration, async {
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
        .map_err(|_| Box::<dyn std::error::Error>::from("Timeout waiting for response"))?
    }

    pub async fn wait_for_response<F>(&mut self, predicate: F) -> Result<Response, Box<dyn std::error::Error>>
    where
        F: Fn(&Response) -> bool,
    {
        self.wait_for_response_with_timeout(Duration::from_secs(1), predicate)
            .await
    }
}

impl Drop for IntegrationFixture {
    fn drop(&mut self) {
        self.core_thread = None;
    }
}
