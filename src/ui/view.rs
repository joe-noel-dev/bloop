use iced::widget::column;
use iced::Length::Fill;
use iced::{Element, Theme};

use super::constants::display_units;
use super::message::Message;
use super::project::project_view;
use super::state::State;
use super::transport::transport_view;

pub fn render(state: &State) -> Element<Message> {
    column![
        project_view(state),
        transport_view(&state.playback_state, &state.progress)
    ]
    .spacing(display_units(2.0))
    .width(Fill)
    .into()
}

pub fn theme(_state: &State) -> Theme {
    Theme::Moonfly
}
