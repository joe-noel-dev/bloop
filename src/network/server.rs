use super::client;
use crate::api::{request, response};
use tokio::net::TcpListener;
use tokio::sync::{broadcast, mpsc};

pub async fn run(request_tx: mpsc::Sender<request::Request>, response_tx: broadcast::Sender<response::Response>) {
    let listener = TcpListener::bind("127.0.0.1:8999").await.expect("Failed to bind");

    println!("Server listening");

    while let Ok((stream, _)) = listener.accept().await {
        let tx = request_tx.clone();
        let rx = response_tx.subscribe();
        tokio::spawn(async move {
            client::run(stream, tx, rx).await;
        });
    }
}
