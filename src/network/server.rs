use crate::api::{Request, Response};

use super::client;
use libmdns::Responder;
use log::info;
use tokio::net::TcpListener;
use tokio::sync::{broadcast, mpsc};

const PORT: u16 = 0;
const BIND_ADDRESS: &str = "0.0.0.0";

pub async fn run(request_tx: mpsc::Sender<Request>, response_tx: broadcast::Sender<Response>) {
    let address = format!("{BIND_ADDRESS}:{PORT}");
    let listener = TcpListener::bind(address).await.expect("Failed to bind");

    let local_address = listener.local_addr().expect("Unable to get address from port");

    let local_port = local_address.port();
    let my_id = uuid::Uuid::new_v4().hyphenated().to_string();

    let responder = Responder::new().expect("Couldn't create an mDNS responder");
    let _service = responder.register("_bloop._tcp".into(), format!("bloop-{my_id}"), local_port, &[]);

    info!("Server listening on port {local_port}");

    while let Ok((stream, _)) = listener.accept().await {
        let tx = request_tx.clone();
        let rx = response_tx.subscribe();
        tokio::spawn(async move {
            client::run(stream, tx, rx).await;
        });
    }
}
