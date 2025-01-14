use std::time::Duration;

use iced::widget::{button, text};
use iced::{time, Element, Subscription, Theme};

pub fn run_ui() -> iced::Result {
    iced::application("Bloop", update, view)
        .theme(theme)
        .subscription(subscription)
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
}

#[derive(Default)]
struct State {
    counter: u64,
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::Increment => state.counter += 1,
    }
}

fn view(state: &State) -> Element<Message> {
    button(text(state.counter)).on_press(Message::Increment).into()
}

fn theme(_state: &State) -> Theme {
    Theme::Dark
}

fn subscription(_state: &State) -> Subscription<Message> {
    time::every(Duration::from_millis(1_000)).map(|_| Message::Increment)
}
