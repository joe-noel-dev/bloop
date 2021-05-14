use super::error;
use crate::api::{request, response};
use futures::Sink;
use futures_util::{SinkExt, StreamExt};
use std::format;
use std::marker::Unpin;
use tokio::net::TcpStream;
use tokio::sync::{broadcast, mpsc};
use tungstenite::protocol::Message;

pub async fn run(
    socket: TcpStream,
    request_tx: mpsc::Sender<request::Request>,
    mut response_rx: broadcast::Receiver<response::Response>,
) {
    let addr = match socket.peer_addr() {
        Ok(addr) => addr,
        Err(_) => {
            println!("Error getting peer address");
            return;
        }
    };

    println!("New connection: {}", addr);

    let ws_stream = match tokio_tungstenite::accept_async(socket).await {
        Ok(stream) => stream,
        Err(_) => {
            println!("Error during WebSocket handshake");
            return;
        }
    };

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

                let message = match message {
                    Message::Text(message) => message,
                    _ => continue
                };

                let api_request = match handle_message(&message) {
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

async fn send_response(response: response::Response, mut outgoing: impl Sink<Message> + Unpin) {
    let message = match serde_json::to_string(&response) {
        Ok(message) => message,
        Err(_) => {
            println!("Failed to convert response to string");
            return;
        }
    };
    let message = Message::from(message);
    let _ = outgoing.send(message).await;
}

fn handle_message(message: &str) -> Result<request::Request, error::NetworkError> {
    let request: request::Request = match serde_json::from_str(message) {
        Ok(request) => request,
        Err(error) => {
            let message = format!("Failed to parse JSON: {}", error);
            return Err(error::NetworkError::new(&message));
        }
    };
    Ok(request)
}
