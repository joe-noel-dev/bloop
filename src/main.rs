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

use std::time::SystemTime;

use crate::control::MainController;
use crate::network::run_server;
use tokio::join;
use tokio::sync::{broadcast, mpsc};

#[tokio::main]
async fn main() {
    setup_logger().expect("Failed to setup logger");

    let (request_tx, request_rx) = mpsc::channel(128);
    let (response_tx, _) = broadcast::channel(128);

    let mut main_controller = MainController::new(request_rx, response_tx.clone());
    main_controller.load_last_project().await;

    let control_future = main_controller.run();
    let network_future = run_server(request_tx, response_tx);
    join!(control_future, network_future);
}

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .filter(|metadata| {
            if metadata.target().contains("libmdns") {
                return metadata.level() <= log::LevelFilter::Info;
            }

            true
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("bloop.log")?)
        .apply()?;
    Ok(())
}
