use crate::api::{Request, Response};

use super::server;
use tokio::sync::{broadcast, mpsc};

pub async fn run(request_tx: mpsc::Sender<Request>, response_tx: broadcast::Sender<Response>) {
    tokio::spawn(async move {
        server::run(request_tx.clone(), response_tx).await;
    });
}
