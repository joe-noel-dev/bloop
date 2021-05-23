mod api;
mod audio;
mod control;
mod generators;
mod model;
mod network;
mod samples;
mod types;
mod waveform;

use tokio::join;
use tokio::sync::{broadcast, mpsc};

use crate::control::main::MainController;

#[tokio::main]
async fn main() {
    let (request_tx, request_rx) = mpsc::channel(128);
    let (response_tx, _) = broadcast::channel(128);

    let mut main_controller = MainController::new(request_rx, response_tx.clone());

    let control_fut = main_controller.run();
    let network_fut = network::manager::run(request_tx, response_tx);
    join!(control_fut, network_fut);
}
