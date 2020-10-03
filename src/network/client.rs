use super::error;
use crate::api;
use futures_util::StreamExt;
use std::error::Error;
use std::format;
use tokio::net::TcpStream;
use tungstenite::protocol::Message;

type RequestDestination = tokio::sync::mpsc::Sender<api::request::Request>;

pub async fn run(
    socket: TcpStream,
    mut request_destination: RequestDestination,
) {
    let addr = socket
        .peer_addr()
        .expect("Couldn't get peer address from connection");

    println!("New connection: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(socket)
        .await
        .expect("Error in handshake");

    println!("Web socket established: {}", addr);

    let (_, mut incoming) = ws_stream.split();
    let tx_task = tokio::spawn(async move {});

    let rx_task = tokio::spawn(async move {
        while let Some(message) = incoming.next().await {
            let api_request = match handle_message(message.unwrap()) {
                Ok(request) => request,
                Err(error) => {
                    // TODO: Respond with error
                    println!("{:?}", error);
                    continue;
                }
            };

            request_destination.send(api_request).await.unwrap();
        }
    });

    tx_task.await.unwrap();
    rx_task.await.unwrap();

    println!("{} disconnected", addr);
}

// fn error_message(error: &str) -> Message {
//     let mut response = api::response::Response::new();
//     response.error = Option::Some(error.to_string());
//     Message::from(serde_json::to_string(&response).unwrap())
// }

fn handle_message(
    message: Message,
) -> Result<api::request::Request, error::NetworkError> {
    let request: api::request::Request =
        match serde_json::from_str(message.to_text().unwrap()) {
            Ok(request) => request,
            Err(error) => {
                let message = format!("Failed to parse JSON: {}", error);
                return Err(error::NetworkError::new(&message));
            }
        };
    Ok(request)
}
