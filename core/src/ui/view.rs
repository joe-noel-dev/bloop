use iced::widget::{button, column, responsive, row, Space};
use iced::Length::Fill;
use iced::{Element, Size, Theme};

use super::constants::UiMetrics;
use super::icons::Icon;
use super::message::Message;
use super::project::project_view;
use super::settings::render_settings_overlay;
use super::state::State;
use super::theme;
use super::transport::transport_view;

pub fn render(state: &State) -> Element<'_, Message> {
    responsive(move |viewport| render_at_size(state, viewport)).into()
}

fn render_at_size(state: &State, viewport: Size) -> Element<'_, Message> {
    let metrics = UiMetrics::from_viewport(viewport);
    let utility_target = 48.0;
    let utility_row = row![
        Space::new().width(Fill),
        button(Icon::Gear.to_svg_with_size(metrics.utility_icon()))
            .height(utility_target)
            .width(utility_target)
            .on_press(Message::OpenSettings),
    ]
    .padding([metrics.spacing(0.5), metrics.spacing(2.0)]);

    let base = column![
        utility_row,
        column![
            project_view(state, metrics),
            transport_view(&state.playback_state, &state.progress, metrics)
        ]
        .spacing(metrics.spacing(2.0))
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
