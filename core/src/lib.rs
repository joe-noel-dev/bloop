include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));

use std::fs;

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

pub use crate::config::AppConfig;

const GIT_SHA: &str = git_version!();

pub fn run_main() {
    let app_config = AppConfig::default();
    let log_directory = app_config.root_directory.join("logs");
    if let Err(error) = fs::create_dir_all(&log_directory) {
        eprintln!("Unable to create log directory {}: {error}", log_directory.display());
    }

    let options = LogOptions::default()
        .log_to_console(true)
        .log_to_file(log_directory.join("bloop.log"))
        .log_dependencies_to_file(log_directory.join("bloop.deps.log"));

    set_up_logger(options);

    let version = env!("CARGO_PKG_VERSION");

    info!("Running bloop v{version} ({GIT_SHA})");

    let (request_tx, request_rx) = mpsc::channel(128);
    let (response_tx, _) = broadcast::channel(128);

    let core_thread = run_core(request_rx, request_tx.clone(), response_tx.clone(), app_config);

    #[cfg(feature = "ui")]
    if !std::env::args().any(|arg| arg == "--headless") {
        run_ui(response_tx, request_tx).expect("Error running UI");
    }

    core_thread.join().expect("Failed to join core thread");
}
