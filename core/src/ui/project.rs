use iced::{
    widget::{button, column, row, text},
    Alignment::Center,
    Element,
    Length::Fill,
};

use crate::model::Song;

use super::{constants::UiMetrics, icons::Icon, message::Message, sections::sections_view, state::State};

pub fn project_view(state: &State, metrics: UiMetrics) -> Element<'_, Message> {
    let song = match state.project.selected_song() {
        Some(song) => song,
        None => return row![].height(Fill).width(Fill).into(),
    };

    column![header(song, metrics), sections_view(song.id, state, metrics)]
        .spacing(metrics.spacing(2.0))
        .padding(metrics.spacing(2.0))
        .height(Fill)
        .width(Fill)
        .into()
}

fn header(song: &Song, metrics: UiMetrics) -> Element<'_, Message> {
    let target_dimension = metrics.touch_target();
    let icon_dimension = metrics.control_icon();
    let left_icon = Icon::ArrowLeft.to_svg_with_size(icon_dimension);
    let right_icon = Icon::ArrowRight.to_svg_with_size(icon_dimension);

    row![
        button(left_icon)
            .height(target_dimension)
            .width(target_dimension)
            .on_press(Message::SelectPreviousSong),
        text(&song.name).width(Fill).size(metrics.song_text()),
        button(right_icon)
            .height(target_dimension)
            .width(target_dimension)
            .on_press(Message::SelectNextSong),
    ]
    .spacing(metrics.spacing(2.0))
    .align_y(Center)
    .into()
}
