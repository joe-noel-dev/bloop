use crate::api::{Request, Response};

use super::client;
use tokio::net::TcpListener;
use tokio::sync::{broadcast, mpsc};

const PORT: i32 = 8999;
const BIND_ADDRESS: &str = "0.0.0.0";

pub async fn run(request_tx: mpsc::Sender<Request>, response_tx: broadcast::Sender<Response>) {
    let address = format!("{BIND_ADDRESS}:{PORT}");
    let listener = TcpListener::bind(address).await.expect("Failed to bind");

    println!("Server listening on port {PORT}");

    while let Ok((stream, _)) = listener.accept().await {
        let tx = request_tx.clone();
        let rx = response_tx.subscribe();
        tokio::spawn(async move {
            client::run(stream, tx, rx).await;
        });
    }
}
