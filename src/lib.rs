include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));

mod api;
mod audio;
pub mod backend;
mod config;
mod control;
mod core;
mod ffi;
mod generators;
mod logger;
mod midi;
mod model;
mod network;
mod preferences;
mod samples;
mod switch;
mod types;
#[cfg(feature = "ui")]
mod ui;
mod waveform;

pub use core::run_core;
use git_version::git_version;
use log::info;
use logger::{set_up_logger, LogOptions};
use tokio::sync::{broadcast, mpsc};

#[cfg(feature = "ui")]
use ui::run_ui;

use crate::config::{get_api_url, get_root_directory};

const GIT_SHA: &str = git_version!();

pub fn run_main() {
    let options = LogOptions::default()
        .log_to_console(true)
        .log_to_file("bloop.log".into())
        .log_dependencies_to_file("bloop.deps.log".into());

    set_up_logger(options);

    let version = env!("CARGO_PKG_VERSION");

    info!("Running bloop v{version} ({GIT_SHA})");

    let (request_tx, request_rx) = mpsc::channel(128);
    let (response_tx, _) = broadcast::channel(128);

    let core_thread = run_core(
        get_root_directory(),
        get_api_url(),
        request_rx,
        request_tx.clone(),
        response_tx.clone(),
    );

    #[cfg(feature = "ui")]
    if !std::env::args().any(|arg| arg == "--headless") {
        run_ui(response_tx, request_tx).expect("Error running UI");
    }

    core_thread.join().expect("Failed to join core thread");
}
