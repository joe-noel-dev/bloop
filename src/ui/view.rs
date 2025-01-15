use iced::widget::{button, column, row, text};
use iced::Alignment::Center;
use iced::Length::Fill;
use iced::{Element, Theme};

use crate::model::{PlaybackState, PlayingState, Project, Song};

use super::constants::display_units;
use super::message::Message;
use super::state::State;

pub fn render(state: &State) -> Element<Message> {
    column![project_view(&state.project), transport_view(&state.playback_state)]
        .width(Fill)
        .into()
}

fn project_view(project: &Project) -> Element<Message> {
    let empty_project = row![].height(Fill).width(Fill);

    let song = match project.selected_song() {
        Some(song) => song,
        None => return empty_project.into(),
    };

    column![
        row![
            button(text("Back")).on_press(Message::SelectPreviousSong),
            text(&song.name).center().width(Fill),
            button(text("Forward")).on_press(Message::SelectNextSong),
        ],
        sections_view(song)
    ]
    .padding(display_units(2.0))
    .height(Fill)
    .width(Fill)
    .into()
}

fn sections_view(song: &Song) -> Element<Message> {
    column(
        song.sections
            .iter()
            .map(|section| text(&section.name).width(Fill).into()),
    )
    .into()
}

fn transport_view(playback_state: &PlaybackState) -> Element<'static, Message> {
    let play_button = match playback_state.playing {
        PlayingState::Playing => button(text("Stop")).on_press(Message::StopPlayback),
        PlayingState::Stopped => button(text("Play")).on_press(Message::StartPlayback),
    };

    column![row![play_button]]
        .width(Fill)
        .align_x(Center)
        .padding(display_units(2.0))
        .into()
}

pub fn theme(_state: &State) -> Theme {
    Theme::Dark
}
