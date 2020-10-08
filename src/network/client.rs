use super::error;
use crate::api::{request, response};
use futures_util::{SinkExt, StreamExt};
use std::format;
use tokio::net::TcpStream;
use tokio::sync::{broadcast, mpsc};
use tokio_tungstenite::WebSocketStream;
use tungstenite::protocol::Message;

type MessageSink = futures_util::stream::SplitSink<WebSocketStream<TcpStream>, Message>;

pub async fn run(
    socket: TcpStream,
    mut request_tx: mpsc::Sender<request::Request>,
    mut response_rx: broadcast::Receiver<response::Response>,
) {
    let addr = socket.peer_addr().expect("Couldn't get peer address from connection");

    println!("New connection: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(socket)
        .await
        .expect("Error in handshake");

    println!("Web socket established: {}", addr);

    let (mut outgoing, mut incoming) = ws_stream.split();

    loop {
        tokio::select! {
            Ok(response) = response_rx.recv() => {
                send_response(response, &mut outgoing).await;
            },
            Some(message) = incoming.next() => {

                let message = match message {
                    Ok(message) => message,
                    Err(_) => {
                        println!("Error receiving from client: {}", addr);
                        break;
                    }
                };

                let api_request = match handle_message(message) {
                    Ok(request) => request,
                    Err(error) => {
                        send_response(response::Response::new().with_error(&error.to_string()), &mut outgoing).await;
                        continue;
                    }
                };
                match request_tx.send(api_request).await {
                    Ok(_) => continue,
                    Err(_) => {
                        println!("Client disconnected: {}", addr);
                        break;
                    }
                }
            },
            else => { break }
        }
    }

    println!("{} disconnected", addr);
}

async fn send_response(response: response::Response, outgoing: &mut MessageSink) {
    let message = serde_json::to_string(&response).unwrap();
    let message = Message::from(message);
    let _ = outgoing.send(message).await;
}

fn handle_message(message: Message) -> Result<request::Request, error::NetworkError> {
    let request: request::Request = match serde_json::from_str(message.to_text().unwrap()) {
        Ok(request) => request,
        Err(error) => {
            let message = format!("Failed to parse JSON: {}", error);
            return Err(error::NetworkError::new(&message));
        }
    };
    Ok(request)
}
