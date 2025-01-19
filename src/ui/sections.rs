use iced::{
    border,
    widget::{column, container, row, scrollable, text},
    Color, Element,
    Length::{Fill, FillPortion},
    Theme,
};
use uuid::Uuid;

use crate::model::{PlayingState, Section};

use super::{constants::display_units, message::Message, state::State};

pub fn sections_view(song_id: Uuid, state: &State) -> Element<Message> {
    let song = match state.project.song_with_id(&song_id) {
        Some(song) => song,
        None => return column![].into(),
    };

    scrollable(column(song.sections.iter().map(|section| section_view(section, state))).spacing(display_units(1.0)))
        .spacing(display_units(1.0))
        .into()
}

fn section_view<'a>(section: &'a Section, state: &'a State) -> Element<'a, Message> {
    let is_playing =
        state.playback_state.playing == PlayingState::Playing && state.playback_state.section_id == Some(section.id);

    let is_selected = state.project.selections.section == Some(section.id);

    let progress = match is_playing {
        true => state.progress.section_progress,
        false => 0.0,
    };

    container(row![
        status_bar(is_selected, is_playing),
        column![
            row![text(&section.name).size(18.0).width(Fill).height(Fill)].padding(display_units(0.5)),
            progress_bar(progress, is_playing)
        ],
    ])
    .clip(true)
    .height(display_units(6.0))
    .style(section_background_style)
    .into()
}

fn background_color() -> iced::Color {
    Color::WHITE.scale_alpha(0.01)
}

fn highlight_color(theme: &Theme, is_selected: bool, is_playing: bool) -> iced::Color {
    match (is_selected, is_playing) {
        (_, true) => theme.palette().primary,
        (true, false) => Color::WHITE.scale_alpha(0.5),
        (false, false) => Color::TRANSPARENT,
    }
}

fn section_border_radius() -> f32 {
    display_units(0.5)
}

fn section_background_style(_theme: &Theme) -> container::Style {
    container::background(background_color()).border(border::rounded(section_border_radius()))
}

fn status_bar(is_selected: bool, is_playing: bool) -> Element<'static, Message> {
    container(column![])
        .height(Fill)
        .width(display_units(1.0))
        .style(move |theme| {
            container::background(highlight_color(theme, is_selected, is_playing))
                .border(border::rounded(border::left(section_border_radius())))
        })
        .into()
}

fn progress_bar(progress: f64, is_playing: bool) -> Element<'static, Message> {
    let active_portion = (progress * u16::MAX as f64) as u16;
    let inactive_portion = u16::MAX - active_portion;

    let height = display_units(1.0);

    container(row![
        container(column![])
            .width(FillPortion(active_portion))
            .height(height)
            .style(move |theme| container::background(highlight_color(theme, false, is_playing))),
        container(column![])
            .width(FillPortion(inactive_portion))
            .height(height)
            .style(move |_| container::background(background_color())),
    ])
    .into()
}
