mod api;
mod audio;
mod control;
mod core;
mod generators;
mod logger;
mod midi;
mod model;
mod network;
mod pedal;
mod preferences;
mod samples;
mod types;
mod ui;
mod waveform;

use core::run_core;
use git_version::git_version;
use log::info;
use logger::set_up_logger;
use tokio::sync::{broadcast, mpsc};
use ui::run_ui;

const GIT_SHA: &str = git_version!();

fn main() {
    set_up_logger();

    let version = env!("CARGO_PKG_VERSION");

    info!("Running bloop v{version} ({GIT_SHA})");

    let (request_tx, request_rx) = mpsc::channel(128);
    let (response_tx, _) = broadcast::channel(128);

    let core_thread = run_core(request_rx, request_tx.clone(), response_tx.clone());

    if !std::env::args().any(|arg| arg == "--headless") {
        run_ui(response_tx, request_tx).expect("Error running UI");
    }

    core_thread.join().expect("Failed to join core thread");
}
