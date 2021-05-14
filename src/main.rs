mod api;
mod control;
mod database;
mod generators;
mod model;
mod network;

extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use std::io::Error;

use tokio::join;
use tokio::sync::{broadcast, mpsc};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (request_tx, mut request_rx) = mpsc::channel(100);
    let (response_tx, _) = broadcast::channel(100);

    let control_fut = control::main::run(&mut request_rx, response_tx.clone());
    let network_fut = network::manager::run(request_tx, response_tx);
    join!(control_fut, network_fut);
    Ok(())
}
