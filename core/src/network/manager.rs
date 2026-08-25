use crate::bloop::{Request, Response};

use super::server;
use tokio::sync::{broadcast, mpsc};
use tokio_util::sync::CancellationToken;

pub async fn run(
    request_tx: mpsc::Sender<Request>,
    response_tx: broadcast::Sender<Response>,
    shutdown: CancellationToken,
) {
    server::run(request_tx, response_tx, shutdown).await;
}
