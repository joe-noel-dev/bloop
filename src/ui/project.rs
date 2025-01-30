use iced::{
    widget::{button, column, row, text},
    Alignment::Center,
    Element,
    Length::Fill,
};

use crate::model::Song;

use super::{constants::display_units, icons::Icon, message::Message, sections::sections_view, state::State};

pub fn project_view(state: &State) -> Element<Message> {
    let song = match state.project.selected_song() {
        Some(song) => song,
        None => return row![].height(Fill).width(Fill).into(),
    };

    column![header(song), sections_view(song.id, state)]
        .spacing(display_units(2.0))
        .padding(display_units(2.0))
        .height(Fill)
        .width(Fill)
        .into()
}

fn header(song: &Song) -> Element<Message> {
    let icon_dimension = display_units(8.0);
    let left_icon = Icon::ArrowLeft.to_svg_with_size(icon_dimension);
    let right_icon = Icon::ArrowRight.to_svg_with_size(icon_dimension);

    row![
        button(left_icon)
            .height(icon_dimension)
            .width(icon_dimension)
            .on_press(Message::SelectPreviousSong),
        text(&song.name).width(Fill).size(64.0),
        button(right_icon)
            .height(icon_dimension)
            .width(icon_dimension)
            .on_press(Message::SelectNextSong),
    ]
    .spacing(display_units(2.0))
    .align_y(Center)
    .into()
}
