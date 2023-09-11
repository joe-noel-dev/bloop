#![feature(is_sorted)]

mod api;
mod audio;
mod control;
mod generators;
mod midi;
mod model;
mod network;
mod pedal;
mod samples;
mod types;
mod waveform;

use crate::control::MainController;
use crate::network::run_server;
use tokio::join;
use tokio::sync::{broadcast, mpsc};

#[tokio::main]
async fn main() {
    let (request_tx, request_rx) = mpsc::channel(128);
    let (response_tx, _) = broadcast::channel(128);

    let mut main_controller = MainController::new(request_rx, response_tx.clone());
    main_controller.load_last_project().await;

    let control_future = main_controller.run();
    let network_future = run_server(request_tx, response_tx);
    join!(control_future, network_future);
}
