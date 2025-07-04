include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));

mod api;
mod audio;
pub mod backend;
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
use std::path::PathBuf;
use tokio::sync::{broadcast, mpsc};

#[cfg(feature = "ui")]
use ui::run_ui;

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

fn get_root_directory() -> PathBuf {
    if let Ok(bloop_home) = std::env::var("BLOOP_HOME") {
        PathBuf::from(bloop_home)
    } else {
        let mut home = home::home_dir().unwrap();

        if cfg!(target_os = "ios") {
            home.push("Documents");
        }

        home.push("bloop");

        home
    }
}
