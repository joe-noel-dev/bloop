use super::error;
use crate::api::{Request, Response};
use futures::SinkExt;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::StreamExt;
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio::select;
use tokio::sync::{broadcast, mpsc};
use tokio_tungstenite::{accept_async, WebSocketStream};
use tungstenite::protocol::Message;

struct Client {
    request_tx: mpsc::Sender<Request>,
    response_rx: broadcast::Receiver<Response>,
    address: SocketAddr,
    outgoing: SplitSink<WebSocketStream<TcpStream>, Message>,
    incoming: SplitStream<WebSocketStream<TcpStream>>,
}

impl Client {
    async fn new(
        socket: TcpStream,
        request_tx: mpsc::Sender<Request>,
        response_rx: broadcast::Receiver<Response>,
    ) -> Result<Self, String> {
        let address = socket.peer_addr().expect("Error getting peer address");

        println!("New connection: {address}");

        let ws_stream = match accept_async(socket).await {
            Ok(stream) => stream,
            Err(_) => {
                return Err("Error during WebSocket handshake".to_string());
            }
        };

        let (outgoing, incoming) = ws_stream.split();

        Ok(Self {
            request_tx,
            response_rx,
            address,
            outgoing,
            incoming,
        })
    }

    async fn on_incoming_message(
        &mut self,
        message: Option<Result<Message, tungstenite::Error>>,
    ) -> anyhow::Result<()> {
        let message = match message {
            Some(message) => message,
            None => anyhow::bail!("Connection closed to client: {}", self.address),
        };

        let message = match message {
            Ok(message) => message,
            Err(_) => anyhow::bail!("Error receiving from client: {}", self.address),
        };

        let mut message = match message {
            Message::Binary(message) => message,
            _ => return Ok(()),
        };

        let api_request = match convert_bytes_to_request(&mut message) {
            Ok(request) => request,
            Err(error) => {
                let response = Response::default().with_error(&error.to_string());
                self.send_response(&response).await;
                return Ok(());
            }
        };

        match self.request_tx.send(api_request).await {
            Ok(_) => Ok(()),
            Err(_) => anyhow::bail!("Client disconnected: {}", self.address),
        }
    }

    async fn run(&mut self) {
        loop {
            select! {
                Ok(response) = self.response_rx.recv() => {
                    self.send_response(&response).await;
                },
                message = self.incoming.next() => {

                    if let Err(err) = self.on_incoming_message(message).await {
                        eprintln!("Error from client: {err}");
                        break;
                    }

                },
                else => { break },

            }
        }

        println!("Client disconnected: {}", self.address);
    }

    async fn send_response(&mut self, response: &Response) {
        if let Some(message) = convert_response(response) {
            let _ = self.outgoing.send(message).await;
        }
    }
}

pub async fn run(socket: TcpStream, request_tx: mpsc::Sender<Request>, response_rx: broadcast::Receiver<Response>) {
    match Client::new(socket, request_tx, response_rx).await {
        Ok(mut client) => client.run().await,
        Err(error) => eprintln!("Error from client: {error}"),
    }
}

fn convert_response(response: &Response) -> Option<Message> {
    let document = match bson::to_document(response) {
        Ok(doc) => doc,
        Err(error) => {
            println!("Error serialising response: {error}");
            return None;
        }
    };

    let mut data: Vec<u8> = vec![];
    document.to_writer(&mut data).unwrap();
    Some(Message::binary(data))
}

fn convert_bytes_to_request(message: &mut [u8]) -> Result<Request, error::NetworkError> {
    let document = match bson::Document::from_reader(&mut &message[..]) {
        Ok(doc) => doc,
        Err(error) => {
            let message = format!("Failed to parse JSON: {error}");
            return Err(error::NetworkError::new(&message));
        }
    };

    let request: Request = match bson::from_document(document) {
        Ok(request) => request,
        Err(error) => {
            let message = format!("Error parsing request: {error}");
            return Err(error::NetworkError::new(&message));
        }
    };

    Ok(request)
}
