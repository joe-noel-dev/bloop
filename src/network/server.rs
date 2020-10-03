use tokio::net::TcpListener;

use super::client;

use tokio::sync::mpsc;

pub async fn run() {
    let mut listener = TcpListener::bind("127.0.0.1:8999")
        .await
        .expect("Failed to bind");

    println!("Server listening");

    let (tx, mut rx) = mpsc::channel(100);

    tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            println!("Handling message {:?}", message);
        }
    });

    while let Ok((stream, _)) = listener.accept().await {
        let tx = tx.clone();
        tokio::spawn(async move {
            client::run(stream, tx).await;
        });
    }
}
