use super::server;
use crate::api::response;
use tokio::sync::{broadcast, mpsc};

pub async fn run() {
    let (request_tx, mut request_rx) = mpsc::channel(100);
    let (response_tx, _) = broadcast::channel(100);

    let server_response_tx = response_tx.clone();
    tokio::spawn(async move {
        server::run(request_tx.clone(), server_response_tx).await;
    });

    while let Some(message) = request_rx.recv().await {
        println!("Handling message {:?}", message);
        let mut response = response::Response::new();
        response.error = Some("Unhandled message".to_string());
        response_tx.send(response).unwrap();
    }
}
