mod api;
mod audio;
mod control;
mod generators;
mod midi;
mod model;
mod network;
mod pedal;
mod preferences;
mod samples;
mod types;
mod ui;
mod waveform;

use crate::network::run_server;
use control::run_main_controller;
use git_version::git_version;
use log::info;
use std::thread;
use std::time::SystemTime;
use tokio::join;
use tokio::sync::{broadcast, mpsc};
use ui::run_ui;

const GIT_SHA: &str = git_version!();

fn main() {
    setup_logger().expect("Failed to setup logger");

    let version = env!("CARGO_PKG_VERSION");

    info!("Running bloop v{version} ({GIT_SHA})");

    let (request_tx, request_rx) = mpsc::channel(128);
    let (response_tx, _) = broadcast::channel(128);

    let async_task = thread::spawn(move || {
        let runtime = tokio::runtime::Runtime::new().expect("Failed to create runtime");
        runtime.block_on(async {
            let control = run_main_controller(request_rx, response_tx.clone());
            let network = run_server(request_tx, response_tx);
            join!(control, network);
        });
    });

    run_ui().expect("Error running UI");

    async_task.join().expect("Failed to join async task");
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
