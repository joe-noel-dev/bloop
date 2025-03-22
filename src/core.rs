use std::thread;

use tokio::{
    join,
    sync::{broadcast, mpsc},
};

use crate::{
    bloop::{Request, Response},
    control::run_main_controller,
    network::run_server,
};

pub fn run_core(
    request_rx: mpsc::Receiver<Request>,
    request_tx: mpsc::Sender<Request>,
    response_tx: broadcast::Sender<Response>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let runtime = tokio::runtime::Runtime::new().expect("Failed to create runtime");
        runtime.block_on(async {
            let control = run_main_controller(request_rx, response_tx.clone());
            let network = run_server(request_tx, response_tx.clone());
            join!(control, network);
        });
    })
}
