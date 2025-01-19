use iced::widget::{button, column, row, text};
use iced::Alignment::Center;
use iced::Length::Fill;
use iced::{Element, Theme};

use crate::model::{PlaybackState, PlayingState, Project, Song};

use super::constants::display_units;
use super::icons::Icon;
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

    let icon_dimension = 20.0;
    let left_icon = Icon::ArrowLeft.to_svg_with_size(icon_dimension);
    let right_icon = Icon::ArrowRight.to_svg_with_size(icon_dimension);

    column![
        row![
            button(row![left_icon, "Back"].align_y(Center).spacing(display_units(1.0)))
                .on_press(Message::SelectPreviousSong),
            text(&song.name).center().width(Fill),
            button(row!["Forward", right_icon].align_y(Center).spacing(display_units(1.0)))
                .on_press(Message::SelectNextSong),
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
    let (icon, message) = match playback_state.playing {
        PlayingState::Playing => (Icon::Stop, Message::StopPlayback),
        PlayingState::Stopped => (Icon::Play, Message::StartPlayback),
    };

    let icon_dimension = 64.0;
    let play_button = button(icon.to_svg_with_size(icon_dimension)).on_press(message);

    column![row![play_button]]
        .width(Fill)
        .align_x(Center)
        .padding(display_units(2.0))
        .into()
}

pub fn theme(_state: &State) -> Theme {
    Theme::Dark
}
