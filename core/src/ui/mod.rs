mod constants;
mod control;
mod icons;
mod message;
mod metronome;
mod power;
mod project;
mod sections;
mod settings;
mod state;
mod theme;
mod transport;
mod view;

use iced::{Size, Task};
use state::State;
use std::sync::Mutex;
use tokio::sync::{broadcast, mpsc};

use crate::bloop::{Request, Response};

pub fn run_ui(response_tx: broadcast::Sender<Response>, request_tx: mpsc::Sender<Request>) -> iced::Result {
    let state = Mutex::new(Some(State::new(response_tx, request_tx)));
    let _sleep_inhibitor = power::SleepInhibitor::new();

    let window_settings = iced::window::Settings {
        size: initial_window_size(),
        fullscreen: cfg!(target_os = "linux"),
        maximized: cfg!(target_os = "linux"),
        resizable: !cfg!(target_os = "linux"),
        decorations: !cfg!(target_os = "linux"),
        ..iced::window::Settings::default()
    };

    iced::application(
        move || {
            let state = state.lock().unwrap().take().expect("boot called twice");
            (state, Task::none())
        },
        control::update,
        view::render,
    )
    .title("Bloop")
    .theme(view::theme)
    .window(window_settings)
    .resizable(cfg!(target_os = "linux") == false)
    .subscription(control::subscription)
    .run()
}

fn initial_window_size() -> Size {
    std::env::var("BLOOP_WINDOW_SIZE")
        .ok()
        .and_then(|value| parse_window_size(&value))
        .unwrap_or_else(|| Size::new(800.0, 480.0))
}

fn parse_window_size(value: &str) -> Option<Size> {
    let (width, height) = value.split_once('x')?;
    let width = width.parse::<f32>().ok()?;
    let height = height.parse::<f32>().ok()?;

    if width.is_finite() && height.is_finite() && width > 0.0 && height > 0.0 {
        Some(Size::new(width, height))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::parse_window_size;
    use iced::Size;

    #[test]
    fn parses_desktop_preview_window_size() {
        assert_eq!(parse_window_size("800x480"), Some(Size::new(800.0, 480.0)));
    }

    #[test]
    fn rejects_invalid_window_sizes() {
        assert_eq!(parse_window_size("800"), None);
        assert_eq!(parse_window_size("widex480"), None);
        assert_eq!(parse_window_size("800x0"), None);
    }
}
