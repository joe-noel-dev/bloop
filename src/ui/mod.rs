mod control;
mod message;
mod state;
mod view;

use iced::Task;
use state::State;
use tokio::sync::{broadcast, mpsc};

use crate::api::{Request, Response};

pub fn run_ui(response_tx: broadcast::Sender<Response>, request_tx: mpsc::Sender<Request>) -> iced::Result {
    let state = State::new(response_tx, request_tx);

    iced::application("Bloop", control::update, view::render)
        .theme(view::theme)
        .subscription(control::subscription)
        .run_with(move || (state, Task::none()))
}
