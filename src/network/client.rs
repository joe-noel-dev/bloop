use super::error;
use crate::api::{request, response};
use futures::{Sink, SinkExt};
use futures_util::StreamExt;
use std::marker::Unpin;
use tokio::net::TcpStream;
use tokio::sync::{broadcast, mpsc};
use tungstenite::protocol::Message;

extern crate bson;
extern crate serde;
extern crate serde_derive;

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
            message = incoming.next() => {

                let message = match message {
                    Some(message) => message,
                    None => {
                        println!("Connection closed to client: {}", addr);
                        break;
                    }
                };


                let message = match message {
                    Ok(message) => message,
                    Err(_) => {
                        println!("Error receiving from client: {}", addr);
                        break;
                    }
                };

                let mut message = match message {
                    Message::Binary(message) => message,
                    _ => continue
                };

                let api_request = match handle_message(&mut message) {
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
            else => { break },

        }
    }

    println!("{} disconnected", addr);
}

async fn send_response(response: response::Response, mut outgoing: impl Sink<Message> + Unpin) {
    let document = match bson::to_document(&response) {
        Ok(doc) => doc,
        Err(error) => {
            println!("Error serialising response: {}", error);
            return;
        }
    };

    let mut data: Vec<u8> = vec![];
    document.to_writer(&mut data).unwrap();

    let _ = outgoing.send(Message::binary(data)).await;
}

fn handle_message(message: &mut [u8]) -> Result<request::Request, error::NetworkError> {
    let document = match bson::Document::from_reader(&mut &message[..]) {
        Ok(doc) => doc,
        Err(error) => {
            let message = format!("Failed to parse JSON: {}", error);
            return Err(error::NetworkError::new(&message));
        }
    };

    let request: request::Request = match bson::from_document(document) {
        Ok(request) => request,
        Err(error) => {
            let message = format!("Error parsing request: {}", error);
            return Err(error::NetworkError::new(&message));
        }
    };

    Ok(request)
}
