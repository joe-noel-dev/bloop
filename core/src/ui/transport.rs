use iced::{
    widget::{button, column, row},
    Alignment::Center,
    Element,
    Length::Fill,
};

use crate::model::{PlaybackState, Progress};

use super::{constants::display_units, icons::Icon, message::Message, metronome::metronome, theme};

pub fn transport_view(playback_state: &PlaybackState, progress: &Progress) -> Element<'static, Message> {
    let is_playing = playback_state.is_playing();

    let (play_icon, play_message) = if is_playing {
        (Icon::Stop, Message::StopPlayback)
    } else {
        (Icon::Play, Message::StartPlayback)
    };

    let icon_dimension = 64.0;
    let is_looping = playback_state.looping;

    let loop_button = button(Icon::Loop.to_svg_with_size(icon_dimension))
        .on_press(if is_looping {
            Message::ExitLoop
        } else {
            Message::EnterLoop
        })
        .style(move |theme, status| loop_button_style(theme, status, is_looping));

    let play_button = button(play_icon.to_svg_with_size(icon_dimension))
        .on_press(play_message)
        .style(move |theme, status| {
            if is_playing {
                return button::primary(theme, status).with_background(theme::PRIMARY);
            }

            button::primary(theme, status)
        });

    column![row![
        metronome(playback_state, progress),
        column![].width(Fill),
        loop_button,
        play_button
    ]
    .align_y(Center)
    .spacing(display_units(2.0))]
    .width(Fill)
    .align_x(Center)
    .padding(display_units(2.0))
    .into()
}

fn loop_button_style(theme: &iced::Theme, status: button::Status, is_looping: bool) -> button::Style {
    if is_looping {
        return button::primary(theme, status).with_background(theme::PRIMARY);
    }

    let background = match status {
        button::Status::Hovered => theme::neutral::N1,
        button::Status::Disabled => theme::neutral::N4,
        button::Status::Active | button::Status::Pressed => theme::neutral::N2,
    };

    button::primary(theme, status).with_background(background)
}
