use super::server;
use crate::api::{request, response};
use tokio::sync::{broadcast, mpsc};

pub async fn run(request_tx: mpsc::Sender<request::Request>, response_tx: broadcast::Sender<response::Response>) {
    tokio::spawn(async move {
        server::run(request_tx.clone(), response_tx).await;
    });
}
