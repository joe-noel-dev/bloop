use iced::widget::{button, column, row, Space};
use iced::Length::Fill;
use iced::{Element, Theme};

use super::constants::display_units;
use super::icons::Icon;
use super::message::Message;
use super::project::project_view;
use super::settings::render_settings_overlay;
use super::state::State;
use super::theme;
use super::transport::transport_view;

pub fn render(state: &State) -> Element<'_, Message> {
    let icon_dimension = display_units(3.0);
    let utility_row = row![
        Space::new().width(Fill),
        button(Icon::Gear.to_svg_with_size(icon_dimension))
            .height(display_units(4.0))
            .width(display_units(4.0))
            .on_press(Message::OpenSettings),
    ]
    .padding([display_units(0.5), display_units(2.0)]);

    let base = column![
        utility_row,
        column![
            project_view(state),
            transport_view(&state.playback_state, &state.progress)
        ]
        .spacing(display_units(2.0))
    ]
    .spacing(0)
    .width(Fill)
    .into();

    render_settings_overlay(
        base,
        &state.settings,
        state.preferences.as_ref(),
        state.audio_devices.as_ref(),
        state.audio_status.as_ref(),
        state.midi_devices.as_ref(),
    )
}

/// Returns the unified Bloop theme matching Editor and iOS color schemes
pub fn theme(_state: &State) -> Theme {
    theme::create_bloop_theme()
}
