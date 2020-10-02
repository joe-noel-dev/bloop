use tokio::net::{TcpListener, TcpStream};

use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};
use tungstenite::protocol::Message;

use crate::api;

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

pub async fn run() {
    let mut listener = TcpListener::bind("127.0.0.1:8999")
        .await
        .expect("Failed to bind");

    println!("Server listening");

    let connections = PeerMap::new(Mutex::new(HashMap::new()));

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(connections.clone(), stream));
    }
}

fn handle_message(message: Message) -> Message {
    if message.is_text() {
        let request: api::request::Request =
            match serde_json::from_str(message.to_text().unwrap()) {
                Ok(request) => request,
                Err(error) => {
                    let description = format!("Couldn't parse JSON: {}", error);
                    return error_message(&description);
                }
            };

        dbg!(request);
    }

    message.clone()
}

async fn accept_connection(connections: PeerMap, stream: TcpStream) {
    let addr = stream
        .peer_addr()
        .expect("Couldn't get peer address from connection");

    println!("New connection: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error in handshake");

    println!("Web socket established: {}", addr);

    let (tx, rx) = unbounded();

    connections.lock().unwrap().insert(addr, tx);

    let (outgoing, incoming) = ws_stream.split();
    let broadcast_incoming = incoming.try_for_each(|message| {
        println!(
            "Received a message from {}: {}",
            addr,
            message.to_text().unwrap()
        );

        let response = handle_message(message);

        let recipients = connections.lock().unwrap();
        let destinations = recipients.iter().map(|(_, sink)| sink);

        for recipient in destinations {
            recipient.unbounded_send(response.clone()).unwrap();
        }

        future::ok(())
    });

    let receive_from_others = rx.map(Ok).forward(outgoing);

    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;

    println!("{} disconnected", addr);

    connections.lock().unwrap().remove(&addr);
}

fn error_message(error: &str) -> Message {
    let mut response = api::response::Response::new();
    response.error = Option::Some(error.to_string());
    Message::from(serde_json::to_string(&response).unwrap())
}
