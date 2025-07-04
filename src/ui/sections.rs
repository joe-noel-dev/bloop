use iced::{
    border,
    widget::{center, column, container, row, text},
    Color, Element,
    Length::{Fill, FillPortion},
    Theme,
};

use crate::model::{Section, ID, INVALID_ID};

use super::{constants::display_units, message::Message, state::State};

pub fn sections_view(song_id: ID, state: &State) -> Element<'_, Message> {
    let song = match state.project.song_with_id(song_id) {
        Some(song) => song,
        None => return column![].into(),
    };

    let section_id = if state.playback_state.is_playing() && state.playback_state.song_id == song_id {
        state.playback_state.section_id
    } else {
        state.project.selections.section
    };

    let section_id = match section_id {
        INVALID_ID => return column![].into(),
        section_id => section_id,
    };

    let index = match song.sections.iter().position(|s| s.id == section_id) {
        Some(index) => index,
        None => return column![].into(),
    };

    let previous_index = if index == 0 { None } else { Some(index - 1) };
    let next_index = if index == song.sections.len() - 1 {
        None
    } else {
        Some(index + 1)
    };

    let previous_section = previous_index.and_then(|i| song.sections.get(i));

    let section = match song.find_section(section_id) {
        Some(section) => section,
        None => return column![].into(),
    };

    let next_section = next_index.and_then(|i| song.sections.get(i));

    let mut elements = Vec::new();

    if let Some(section) = previous_section {
        elements.push(section_view(section, state));
    } else {
        elements.push(container(column![]).height(Fill).into());
    }

    elements.push(section_view(section, state));

    if let Some(section) = next_section {
        elements.push(section_view(section, state));
    } else {
        elements.push(container(column![]).height(Fill).into());
    }

    column(elements).spacing(display_units(2.0)).into()
}

fn section_view<'a>(section: &'a Section, state: &'a State) -> Element<'a, Message> {
    let is_playing = state.playback_state.is_playing() && state.playback_state.section_id == section.id;
    let is_selected = state.project.selections.section == section.id;

    let progress = match is_playing {
        true => state.progress.section_progress,
        false => 0.0,
    };

    container(row![
        status_bar(is_selected, is_playing),
        column![
            row![center(text(&section.name).size(64.0))]
                .padding(display_units(0.5))
                .height(Fill),
            progress_bar(progress, is_playing)
        ],
    ])
    .clip(true)
    .height(Fill)
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
