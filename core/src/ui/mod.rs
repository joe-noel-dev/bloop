mod constants;
mod control;
mod icons;
mod message;
mod metronome;
mod project;
mod sections;
mod state;
mod theme;
mod transport;
mod view;

use iced::{Size, Task};
use state::State;
use tokio::sync::{broadcast, mpsc};

use crate::bloop::{Request, Response};

pub fn run_ui(response_tx: broadcast::Sender<Response>, request_tx: mpsc::Sender<Request>) -> iced::Result {
    let state = State::new(response_tx, request_tx);

    let window_settings = iced::window::Settings {
        size: Size::new(1024.0, 600.0),
        fullscreen: cfg!(target_os = "linux"),
        maximized: cfg!(target_os = "linux"),
        resizable: !cfg!(target_os = "linux"),
        decorations: !cfg!(target_os = "linux"),
        ..iced::window::Settings::default()
    };

    iced::application("Bloop", control::update, view::render)
        .theme(view::theme)
        .window(window_settings)
        .resizable(cfg!(target_os = "linux") == false)
        .subscription(control::subscription)
        .run_with(move || (state, Task::none()))
}
