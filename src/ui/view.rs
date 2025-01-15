use iced::widget::{button, column, row, text};
use iced::Length::Fill;
use iced::{Element, Theme};

use crate::model::{PlaybackState, PlayingState};

use super::message::Message;
use super::state::State;

pub fn render(state: &State) -> Element<Message> {
    column![
        text(&state.project.info.name),
        row![].height(Fill).width(Fill),
        playback_state(&state.playback_state),
        button(text("Play")).on_press(Message::StartPlayback),
        button(text("Stop")).on_press(Message::StopPlayback),
    ]
    .into()
}

fn playback_state(playback_state: &PlaybackState) -> Element<Message> {
    match playback_state.playing {
        PlayingState::Playing => text("Playing").into(),
        PlayingState::Stopped => text("Stopped").into(),
    }
}

pub fn theme(_state: &State) -> Theme {
    Theme::Dark
}
