use iced::{
    widget::{
        button, checkbox, column, container, opaque, pick_list, row, rule, scrollable, stack, text, text_input, toggler,
    },
    Background, Border, Color, Element, Length, Padding, Shadow, Theme,
};

use crate::{
    bloop::{
        Action, AudioDevice, AudioDevices, AudioEngineStatus, AudioPreferences, AudioStatus, Gesture, MidiDevices,
        MidiPreferences, Preferences, SwitchMapping, SwitchPreferences,
    },
    preferences::{default_audio_preferences, default_midi_preferences},
};

use super::{constants::display_units, message::Message, theme};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsTab {
    Audio,
    Midi,
    Switches,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioNumberField {
    SampleRate,
    BufferSize,
    MainChannelOffset,
    ClickChannelOffset,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwitchNumberField {
    Pin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwitchPickField {
    Gesture,
    Action,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AudioDeviceOption {
    Default,
    Device { id: String, name: String },
}

impl std::fmt::Display for AudioDeviceOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AudioDeviceOption::Default => f.write_str("System Default"),
            AudioDeviceOption::Device { name, .. } => f.write_str(name),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SampleRateOption(pub u32);

impl std::fmt::Display for SampleRateOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} Hz", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GestureOption(pub Gesture);

impl std::fmt::Display for GestureOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(gesture_label(self.0))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActionOption(pub Action);

impl std::fmt::Display for ActionOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(action_label(self.0))
    }
}

#[derive(Debug, Clone)]
pub struct SettingsUiState {
    pub is_open: bool,
    pub active_tab: SettingsTab,
    pub draft: Preferences,
    pub sample_rate: String,
    pub buffer_size: String,
    pub main_channel_offset: String,
    pub click_channel_offset: String,
    pub switch_pin_values: Vec<String>,
    pub is_saving: bool,
    pub validation_error: Option<String>,
}

impl Default for SettingsUiState {
    fn default() -> Self {
        let draft = editable_preferences(Preferences::default());
        let mut state = Self {
            is_open: false,
            active_tab: SettingsTab::Audio,
            draft,
            sample_rate: String::new(),
            buffer_size: String::new(),
            main_channel_offset: String::new(),
            click_channel_offset: String::new(),
            switch_pin_values: Vec::new(),
            is_saving: false,
            validation_error: None,
        };
        state.sync_fields_from_draft();
        state
    }
}

impl SettingsUiState {
    pub fn open(&mut self, preferences: Option<Preferences>) {
        self.is_open = true;
        self.is_saving = false;
        self.validation_error = None;
        self.active_tab = SettingsTab::Audio;
        self.set_draft(editable_preferences(preferences.unwrap_or_default()));
    }

    pub fn close(&mut self) {
        self.is_open = false;
        self.is_saving = false;
        self.validation_error = None;
    }

    pub fn set_draft(&mut self, preferences: Preferences) {
        self.draft = editable_preferences(preferences);
        self.sync_fields_from_draft();
    }

    pub fn draft_preferences_for_save(&mut self) -> Option<Preferences> {
        self.validation_error = None;

        let sample_rate = parse_u32_in_range(&self.sample_rate, 1, 192_000, "Sample rate")?;
        let buffer_size = parse_u32_in_range(&self.buffer_size, 1, 8192, "Buffer size")?;
        let main_channel_offset = parse_u32(&self.main_channel_offset, "Main channel offset")?;
        let click_channel_offset = parse_u32(&self.click_channel_offset, "Click channel offset")?;

        let mut preferences = self.draft.clone();
        let mut audio = preferences.audio.unwrap_or_else(default_audio_preferences);
        audio.sample_rate = sample_rate;
        audio.buffer_size = buffer_size;
        audio.main_channel_offset = main_channel_offset;
        audio.click_channel_offset = click_channel_offset;
        preferences.audio = Some(audio).into();

        let mut switch = preferences.switch.unwrap_or_default();
        if switch.mappings.len() != self.switch_pin_values.len() {
            self.sync_switch_pin_values(&switch);
        }
        for (index, pin_text) in self.switch_pin_values.iter().enumerate() {
            let Some(mapping) = switch.mappings.get_mut(index) else {
                continue;
            };
            match parse_u32(pin_text, "Switch pin") {
                Some(pin) => mapping.pin = pin,
                None => {
                    self.validation_error = Some("Switch pin must be a whole number".to_string());
                    return None;
                }
            }
        }
        preferences.switch = Some(switch).into();

        Some(preferences)
    }

    pub fn set_audio_number(&mut self, field: AudioNumberField, value: String) {
        match field {
            AudioNumberField::SampleRate => self.sample_rate = value,
            AudioNumberField::BufferSize => self.buffer_size = value,
            AudioNumberField::MainChannelOffset => self.main_channel_offset = value,
            AudioNumberField::ClickChannelOffset => self.click_channel_offset = value,
        }
        self.validation_error = None;
    }

    pub fn set_audio_device(&mut self, option: AudioDeviceOption) {
        let mut audio = self.draft.audio.clone().unwrap_or_else(default_audio_preferences);
        audio.output_device = match option {
            AudioDeviceOption::Default => String::new(),
            AudioDeviceOption::Device { id, .. } => id,
        };
        self.draft.audio = Some(audio).into();
    }

    pub fn set_sample_rate(&mut self, option: SampleRateOption) {
        self.sample_rate = option.0.to_string();
    }

    pub fn set_use_jack(&mut self, use_jack: bool) {
        let mut audio = self.draft.audio.clone().unwrap_or_else(default_audio_preferences);
        audio.use_jack = use_jack;
        self.draft.audio = Some(audio).into();
    }

    pub fn set_midi_port_enabled(&mut self, port_name: String, enabled: bool) {
        let mut midi = self.draft.midi.clone().unwrap_or_else(default_midi_preferences);
        set_midi_enabled_device(&mut midi, port_name, enabled);
        self.draft.midi = Some(midi).into();
    }

    pub fn add_switch_mapping(&mut self) {
        let mut switch = self.draft.switch.clone().unwrap_or_default();
        switch.mappings.push(default_switch_mapping());
        self.draft.switch = Some(switch).into();
        self.sync_fields_from_draft();
    }

    pub fn remove_switch_mapping(&mut self, index: usize) {
        let mut switch = self.draft.switch.clone().unwrap_or_default();
        if index < switch.mappings.len() {
            switch.mappings.remove(index);
        }
        self.draft.switch = Some(switch).into();
        self.sync_fields_from_draft();
    }

    pub fn set_switch_number(&mut self, index: usize, field: SwitchNumberField, value: String) {
        match field {
            SwitchNumberField::Pin => {
                if index < self.switch_pin_values.len() {
                    self.switch_pin_values[index] = value;
                }
            }
        }
        self.validation_error = None;
    }

    pub fn set_switch_gesture(&mut self, index: usize, gesture: GestureOption) {
        if let Some(mapping) = self.switch_mapping_mut(index) {
            mapping.gesture = gesture.0.into();
        }
    }

    pub fn set_switch_action(&mut self, index: usize, action: ActionOption) {
        if let Some(mapping) = self.switch_mapping_mut(index) {
            mapping.action = action.0.into();
        }
    }

    pub fn has_validation_error(&self) -> bool {
        validate_audio_number(&self.sample_rate, AudioNumberField::SampleRate).is_some()
            || validate_audio_number(&self.buffer_size, AudioNumberField::BufferSize).is_some()
            || validate_audio_number(&self.main_channel_offset, AudioNumberField::MainChannelOffset).is_some()
            || validate_audio_number(&self.click_channel_offset, AudioNumberField::ClickChannelOffset).is_some()
            || self
                .switch_pin_values
                .iter()
                .any(|pin| parse_u32(pin, "Switch pin").is_none())
    }

    fn switch_mapping_mut(&mut self, index: usize) -> Option<&mut SwitchMapping> {
        let switch = self.draft.switch.clone().unwrap_or_default();
        if index >= switch.mappings.len() {
            return None;
        }
        self.draft.switch = Some(switch).into();
        self.draft.switch.as_mut()?.mappings.get_mut(index)
    }

    fn sync_fields_from_draft(&mut self) {
        let audio = self
            .draft
            .audio
            .as_ref()
            .cloned()
            .unwrap_or_else(default_audio_preferences);
        self.sample_rate = audio.sample_rate.to_string();
        self.buffer_size = audio.buffer_size.to_string();
        self.main_channel_offset = audio.main_channel_offset.to_string();
        self.click_channel_offset = audio.click_channel_offset.to_string();

        let switch = self.draft.switch.clone().unwrap_or_default();
        self.sync_switch_pin_values(&switch);
    }

    fn sync_switch_pin_values(&mut self, switch: &SwitchPreferences) {
        self.switch_pin_values = switch.mappings.iter().map(|mapping| mapping.pin.to_string()).collect();
    }
}

pub fn render_settings_overlay<'a>(
    base: Element<'a, Message>,
    settings: &'a SettingsUiState,
    preferences: Option<&'a Preferences>,
    audio_devices: Option<&'a AudioDevices>,
    audio_status: Option<&'a AudioStatus>,
    midi_devices: Option<&'a MidiDevices>,
) -> Element<'a, Message> {
    if !settings.is_open {
        return base;
    }

    stack![
        base,
        opaque(
            container(
                container(settings_panel(
                    settings,
                    preferences,
                    audio_devices,
                    audio_status,
                    midi_devices
                ))
                .max_width(760.0)
                .max_height(520.0)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(panel_style),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(display_units(4.0))
            .center(Length::Fill)
            .style(backdrop_style),
        ),
    ]
    .into()
}

fn settings_panel<'a>(
    settings: &'a SettingsUiState,
    preferences: Option<&'a Preferences>,
    audio_devices: Option<&'a AudioDevices>,
    audio_status: Option<&'a AudioStatus>,
    midi_devices: Option<&'a MidiDevices>,
) -> Element<'a, Message> {
    let tabs = row![
        tab_button("Audio", SettingsTab::Audio, settings.active_tab),
        tab_button("MIDI", SettingsTab::Midi, settings.active_tab),
        if preferences
            .map(|p| p.switch_available)
            .unwrap_or(settings.draft.switch_available)
        {
            tab_button("Switches", SettingsTab::Switches, settings.active_tab)
        } else {
            container(row![]).into()
        },
    ]
    .spacing(display_units(1.0));

    let content = match settings.active_tab {
        SettingsTab::Audio => audio_tab(settings, audio_devices, audio_status),
        SettingsTab::Midi => midi_tab(settings, midi_devices),
        SettingsTab::Switches => switch_tab(settings),
    };

    let error = settings
        .validation_error
        .as_ref()
        .map(|message| {
            text(message).style(|_| text::Style {
                color: Some(theme::palette::COLOR_4),
            })
        })
        .map(Element::from)
        .unwrap_or_else(|| container(row![]).height(Length::Fixed(0.0)).into());

    let save_button = if settings.is_saving || settings.has_validation_error() {
        button(text("Save")).padding(button_padding())
    } else {
        button(text("Save"))
            .padding(button_padding())
            .on_press(Message::SaveSettings)
    };

    column![
        row![
            text("Settings").size(28.0).width(Length::Fill),
            button(text("Refresh"))
                .padding(button_padding())
                .on_press(Message::RefreshSettings),
        ]
        .align_y(iced::Alignment::Center),
        tabs,
        rule::horizontal(1),
        scrollable(content).height(Length::Fill),
        error,
        row![
            button(text("Cancel"))
                .padding(button_padding())
                .on_press(Message::CloseSettings),
            save_button,
        ]
        .spacing(display_units(1.0))
        .align_y(iced::Alignment::Center)
        .width(Length::Fill),
    ]
    .spacing(display_units(2.0))
    .padding(display_units(3.0))
    .height(Length::Fill)
    .into()
}

fn tab_button<'a>(label: &'a str, tab: SettingsTab, active_tab: SettingsTab) -> Element<'a, Message> {
    let mut tab_button = button(text(label)).padding(Padding::from([8.0, 14.0]));
    if tab != active_tab {
        tab_button = tab_button.on_press(Message::SelectSettingsTab(tab));
    }
    tab_button.into()
}

fn audio_tab<'a>(
    settings: &'a SettingsUiState,
    audio_devices: Option<&'a AudioDevices>,
    audio_status: Option<&'a AudioStatus>,
) -> Element<'a, Message> {
    let audio = settings
        .draft
        .audio
        .as_ref()
        .cloned()
        .unwrap_or_else(default_audio_preferences);
    let device_options = audio_device_options(audio_devices);
    let selected_device = selected_device_option(&audio, &device_options);
    let sample_rate_options = sample_rate_options(&audio, audio_devices, audio_status);

    let sample_rate_control: Element<'a, Message> = if sample_rate_options.is_empty() {
        number_input(
            "Sample Rate",
            &settings.sample_rate,
            AudioNumberField::SampleRate,
            validate_audio_number(&settings.sample_rate, AudioNumberField::SampleRate),
        )
    } else {
        setting_row(
            "Sample Rate",
            pick_list(
                sample_rate_options.clone(),
                settings.sample_rate.parse::<u32>().ok().map(SampleRateOption),
                Message::SetSettingsSampleRate,
            )
            .width(Length::Fill)
            .into(),
        )
    };

    column![
        audio_status_view(audio_status),
        setting_row(
            "Output Device",
            pick_list(
                device_options.clone(),
                Some(selected_device),
                Message::SetSettingsAudioDevice
            )
            .width(Length::Fill)
            .into(),
        ),
        sample_rate_control,
        number_input(
            "Buffer Size",
            &settings.buffer_size,
            AudioNumberField::BufferSize,
            validate_audio_number(&settings.buffer_size, AudioNumberField::BufferSize),
        ),
        setting_row(
            "Use JACK",
            toggler(audio.use_jack)
                .on_toggle(Message::SetSettingsUseJack)
                .width(Length::Shrink)
                .into(),
        ),
        number_input(
            "Main Channel Offset",
            &settings.main_channel_offset,
            AudioNumberField::MainChannelOffset,
            validate_audio_number(&settings.main_channel_offset, AudioNumberField::MainChannelOffset),
        ),
        number_input(
            "Click Channel Offset",
            &settings.click_channel_offset,
            AudioNumberField::ClickChannelOffset,
            validate_audio_number(&settings.click_channel_offset, AudioNumberField::ClickChannelOffset),
        ),
    ]
    .spacing(display_units(1.5))
    .into()
}

fn audio_status_view(audio_status: Option<&AudioStatus>) -> Element<'_, Message> {
    let Some(status) = audio_status else {
        return container(text("Audio status unavailable"))
            .padding(display_units(1.5))
            .style(subtle_panel_style)
            .width(Length::Fill)
            .into();
    };

    let status_label = match status.engine_status.enum_value_or_default() {
        AudioEngineStatus::AUDIO_ENGINE_STATUS_RUNNING => "Audio engine running",
        AudioEngineStatus::AUDIO_ENGINE_STATUS_STOPPED => "Audio engine stopped",
        AudioEngineStatus::AUDIO_ENGINE_STATUS_FAILED => "Audio engine failed",
    };

    let device_name = if status.current_device_name.is_empty() {
        "System Default".to_string()
    } else {
        status.current_device_name.clone()
    };

    let error_line: Element<'_, Message> = if status.error.is_empty() {
        text("").into()
    } else {
        text(&status.error)
            .style(|_| text::Style {
                color: Some(theme::palette::COLOR_4),
            })
            .into()
    };

    container(
        column![
            row![
                text(status_label).size(18.0).width(Length::Fill),
                button(text("Restart Audio"))
                    .padding(button_padding())
                    .on_press(Message::RestartAudio),
            ]
            .align_y(iced::Alignment::Center),
            text(format!(
                "{} · {} Hz · {} channels · buffer {}",
                device_name, status.current_sample_rate, status.current_channel_count, status.current_buffer_size
            ))
            .size(14.0),
            error_line,
        ]
        .spacing(display_units(0.75)),
    )
    .padding(display_units(1.5))
    .style(subtle_panel_style)
    .width(Length::Fill)
    .into()
}

fn midi_tab<'a>(settings: &'a SettingsUiState, midi_devices: Option<&'a MidiDevices>) -> Element<'a, Message> {
    let midi = settings
        .draft
        .midi
        .as_ref()
        .cloned()
        .unwrap_or_else(default_midi_preferences);
    let port_names = midi_devices.map(|devices| devices.port_names.as_slice()).unwrap_or(&[]);

    if port_names.is_empty() {
        return column![text("No MIDI devices found").size(16.0)].into();
    }

    let rows = port_names
        .iter()
        .fold(column![].spacing(display_units(1.0)), |column, port_name| {
            let enabled = midi.enabled_devices.contains(port_name);
            let port = port_name.clone();
            column.push(
                row![
                    text(port_name).width(Length::Fill),
                    checkbox(enabled)
                        .on_toggle(move |checked| Message::SetSettingsMidiPortEnabled(port.clone(), checked)),
                ]
                .align_y(iced::Alignment::Center),
            )
        });

    rows.into()
}

fn switch_tab<'a>(settings: &'a SettingsUiState) -> Element<'a, Message> {
    let switch = settings.draft.switch.as_ref();
    let mut content = column![].spacing(display_units(1.5));

    if let Some(switch) = switch {
        for (index, mapping) in switch.mappings.iter().enumerate() {
            let pin = settings.switch_pin_values.get(index).map(String::as_str).unwrap_or("");
            content = content.push(switch_mapping_row(index, mapping, pin));
        }
    }

    if switch.map(|switch| switch.mappings.is_empty()).unwrap_or(true) {
        content = content.push(text("No switch mappings configured"));
    }

    content = content.push(
        button(text("Add Mapping"))
            .padding(button_padding())
            .on_press(Message::AddSettingsSwitchMapping),
    );

    content.into()
}

fn switch_mapping_row<'a>(index: usize, mapping: &'a SwitchMapping, pin: &'a str) -> Element<'a, Message> {
    let gesture_options = gesture_options();
    let action_options = action_options();
    let selected_gesture = GestureOption(mapping.gesture.enum_value_or_default());
    let selected_action = ActionOption(mapping.action.enum_value_or_default());

    container(
        column![
            row![
                text(format!("Mapping {}", index + 1)).width(Length::Fill),
                button(text("Remove"))
                    .padding(button_padding())
                    .on_press(Message::RemoveSettingsSwitchMapping(index)),
            ]
            .align_y(iced::Alignment::Center),
            setting_row(
                "Pin",
                text_input("Pin", pin)
                    .on_input(move |value| Message::SetSettingsSwitchNumber(index, SwitchNumberField::Pin, value))
                    .width(Length::Fill)
                    .into(),
            ),
            setting_row(
                "Gesture",
                pick_list(gesture_options, Some(selected_gesture), move |gesture| {
                    Message::SetSettingsSwitchPick(index, SwitchPickField::Gesture, gesture, selected_action)
                })
                .width(Length::Fill)
                .into(),
            ),
            setting_row(
                "Action",
                pick_list(action_options, Some(selected_action), move |action| {
                    Message::SetSettingsSwitchPick(index, SwitchPickField::Action, selected_gesture, action)
                })
                .width(Length::Fill)
                .into(),
            ),
        ]
        .spacing(display_units(1.0)),
    )
    .padding(display_units(1.5))
    .style(subtle_panel_style)
    .width(Length::Fill)
    .into()
}

fn setting_row<'a>(label: &'a str, control: Element<'a, Message>) -> Element<'a, Message> {
    row![text(label).width(Length::Fixed(190.0)), control]
        .spacing(display_units(1.5))
        .align_y(iced::Alignment::Center)
        .into()
}

fn number_input<'a>(
    label: &'a str,
    value: &'a str,
    field: AudioNumberField,
    error: Option<String>,
) -> Element<'a, Message> {
    let input = text_input(label, value)
        .on_input(move |value| Message::SetSettingsAudioNumber(field, value))
        .width(Length::Fill)
        .padding(display_units(1.0));

    let control: Element<'a, Message> = if let Some(error) = error {
        column![
            input,
            text(error).size(13.0).style(|_| text::Style {
                color: Some(theme::palette::COLOR_4)
            }),
        ]
        .spacing(display_units(0.5))
        .into()
    } else {
        input.into()
    };

    setting_row(label, control)
}

fn button_padding() -> Padding {
    Padding::from([6.0, 12.0])
}

fn backdrop_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color { a: 0.55, ..theme::neutral::N8 })),
        ..Default::default()
    }
}

fn panel_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(theme::neutral::N7)),
        text_color: Some(theme::neutral::N1),
        border: Border {
            radius: 8.0.into(),
            width: 1.0,
            color: theme::neutral::N5,
        },
        shadow: Shadow {
            color: Color { a: 0.35, ..theme::neutral::N8 },
            offset: iced::Vector::new(0.0, 12.0),
            blur_radius: 24.0,
        },
        ..Default::default()
    }
}

fn subtle_panel_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(theme::neutral::N6)),
        text_color: Some(theme::neutral::N1),
        border: Border {
            radius: 6.0.into(),
            width: 1.0,
            color: theme::neutral::N5,
        },
        ..Default::default()
    }
}

fn editable_preferences(mut preferences: Preferences) -> Preferences {
    let mut audio = preferences.audio.unwrap_or_else(default_audio_preferences);
    if audio.sample_rate == 0 {
        audio.sample_rate = 48_000;
    }
    if audio.buffer_size == 0 {
        audio.buffer_size = 512;
    }
    preferences.audio = Some(audio).into();

    if preferences.midi.is_none() {
        preferences.midi = Some(default_midi_preferences()).into();
    }
    if preferences.switch.is_none() {
        preferences.switch = Some(SwitchPreferences::default()).into();
    }

    preferences
}

fn set_midi_enabled_device(midi: &mut MidiPreferences, port_name: String, enabled: bool) {
    if enabled {
        if !midi.enabled_devices.contains(&port_name) {
            midi.enabled_devices.push(port_name);
        }
    } else {
        midi.enabled_devices.retain(|device| device != &port_name);
    }
}

fn default_switch_mapping() -> SwitchMapping {
    SwitchMapping {
        pin: 0,
        gesture: Gesture::GESTURE_PRESS.into(),
        action: Action::ACTION_TOGGLE_PLAY.into(),
        ..Default::default()
    }
}

fn parse_u32_in_range(value: &str, min: u32, max: u32, label: &str) -> Option<u32> {
    let parsed = parse_u32(value, label)?;
    if parsed < min || parsed > max {
        return None;
    }
    Some(parsed)
}

fn parse_u32(value: &str, _label: &str) -> Option<u32> {
    value.trim().parse::<u32>().ok()
}

fn validate_audio_number(value: &str, field: AudioNumberField) -> Option<String> {
    let parsed = value.trim().parse::<u32>().ok();

    match field {
        AudioNumberField::SampleRate => match parsed {
            Some(number) if (1..=192_000).contains(&number) => None,
            _ => Some("Sample rate must be between 1 and 192000".to_string()),
        },
        AudioNumberField::BufferSize => match parsed {
            Some(number) if (1..=8192).contains(&number) => None,
            _ => Some("Buffer size must be between 1 and 8192".to_string()),
        },
        AudioNumberField::MainChannelOffset => match parsed {
            Some(_) => None,
            None => Some("Main channel offset must be a whole number".to_string()),
        },
        AudioNumberField::ClickChannelOffset => match parsed {
            Some(_) => None,
            None => Some("Click channel offset must be a whole number".to_string()),
        },
    }
}

fn audio_device_options(audio_devices: Option<&AudioDevices>) -> Vec<AudioDeviceOption> {
    let mut options = vec![AudioDeviceOption::Default];
    if let Some(audio_devices) = audio_devices {
        options.extend(audio_devices.devices.iter().map(|device| AudioDeviceOption::Device {
            id: device.id.clone(),
            name: device.name.clone(),
        }));
    }
    options
}

fn selected_device_option(audio: &AudioPreferences, options: &[AudioDeviceOption]) -> AudioDeviceOption {
    if audio.output_device.is_empty() {
        return AudioDeviceOption::Default;
    }

    options
        .iter()
        .find(|option| matches!(option, AudioDeviceOption::Device { id, .. } if id == &audio.output_device))
        .cloned()
        .unwrap_or_else(|| AudioDeviceOption::Device {
            id: audio.output_device.clone(),
            name: audio.output_device.clone(),
        })
}

fn sample_rate_options(
    audio: &AudioPreferences,
    audio_devices: Option<&AudioDevices>,
    audio_status: Option<&AudioStatus>,
) -> Vec<SampleRateOption> {
    let selected = selected_audio_device(audio, audio_devices);
    let mut rates = selected
        .map(|device| device.supported_sample_rates.clone())
        .unwrap_or_default();

    if audio.sample_rate > 0 {
        rates.push(audio.sample_rate);
    }
    if let Some(status) = audio_status {
        if status.current_sample_rate > 0 {
            rates.push(status.current_sample_rate);
        }
    }

    rates.sort_unstable();
    rates.dedup();
    rates.into_iter().map(SampleRateOption).collect()
}

fn selected_audio_device<'a>(
    audio: &AudioPreferences,
    audio_devices: Option<&'a AudioDevices>,
) -> Option<&'a AudioDevice> {
    let devices = audio_devices?.devices.as_slice();
    if audio.output_device.is_empty() {
        return devices
            .iter()
            .find(|device| device.is_default)
            .or_else(|| devices.first());
    }
    devices.iter().find(|device| device.id == audio.output_device)
}

fn gesture_options() -> Vec<GestureOption> {
    vec![
        GestureOption(Gesture::GESTURE_PRESS),
        GestureOption(Gesture::GESTURE_RELEASE),
        GestureOption(Gesture::GESTURE_HOLD),
    ]
}

fn action_options() -> Vec<ActionOption> {
    vec![
        ActionOption(Action::ACTION_PREVIOUS_SONG),
        ActionOption(Action::ACTION_NEXT_SONG),
        ActionOption(Action::ACTION_PREVIOUS_SECTION),
        ActionOption(Action::ACTION_NEXT_SECTION),
        ActionOption(Action::ACTION_QUEUE_SELECTED),
        ActionOption(Action::ACTION_TOGGLE_LOOP),
        ActionOption(Action::ACTION_TOGGLE_PLAY),
    ]
}

fn gesture_label(gesture: Gesture) -> &'static str {
    match gesture {
        Gesture::GESTURE_PRESS => "Press",
        Gesture::GESTURE_RELEASE => "Release",
        Gesture::GESTURE_HOLD => "Hold",
        Gesture::GESTURE_UNKNOWN => "Unknown",
    }
}

fn action_label(action: Action) -> &'static str {
    match action {
        Action::ACTION_PREVIOUS_SONG => "Previous Song",
        Action::ACTION_NEXT_SONG => "Next Song",
        Action::ACTION_PREVIOUS_SECTION => "Previous Section",
        Action::ACTION_NEXT_SECTION => "Next Section",
        Action::ACTION_QUEUE_SELECTED => "Queue Selected",
        Action::ACTION_TOGGLE_LOOP => "Toggle Loop",
        Action::ACTION_TOGGLE_PLAY => "Toggle Play",
        Action::ACTION_UNKNOWN => "Unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn editable_preferences_fills_audio_defaults() {
        let preferences = editable_preferences(Preferences {
            audio: Some(AudioPreferences::default()).into(),
            ..Default::default()
        });

        let audio = preferences.audio.unwrap();
        assert_eq!(audio.sample_rate, 48_000);
        assert_eq!(audio.buffer_size, 512);
    }

    #[test]
    fn audio_number_validation_accepts_valid_values_and_rejects_invalid_values() {
        assert!(validate_audio_number("48000", AudioNumberField::SampleRate).is_none());
        assert!(validate_audio_number("0", AudioNumberField::SampleRate).is_some());
        assert!(validate_audio_number("192001", AudioNumberField::SampleRate).is_some());
        assert!(validate_audio_number("512", AudioNumberField::BufferSize).is_none());
        assert!(validate_audio_number("9000", AudioNumberField::BufferSize).is_some());
        assert!(validate_audio_number("4", AudioNumberField::MainChannelOffset).is_none());
    }

    #[test]
    fn midi_toggle_preserves_unmatched_patterns() {
        let mut midi = MidiPreferences {
            enabled_devices: vec!["Manual Pattern".to_string(), "Port A".to_string()],
            ..Default::default()
        };

        set_midi_enabled_device(&mut midi, "Port A".to_string(), false);
        set_midi_enabled_device(&mut midi, "Port B".to_string(), true);

        assert_eq!(
            midi.enabled_devices,
            vec!["Manual Pattern".to_string(), "Port B".to_string()]
        );
    }

    #[test]
    fn switch_mapping_add_update_remove_produces_expected_preferences() {
        let mut state = SettingsUiState::default();

        state.add_switch_mapping();
        state.set_switch_number(0, SwitchNumberField::Pin, "17".to_string());
        state.set_switch_gesture(0, GestureOption(Gesture::GESTURE_HOLD));
        state.set_switch_action(0, ActionOption(Action::ACTION_NEXT_SONG));

        let saved = state.draft_preferences_for_save().unwrap();
        let mapping = &saved.switch.unwrap().mappings[0];
        assert_eq!(mapping.pin, 17);
        assert_eq!(mapping.gesture.enum_value_or_default(), Gesture::GESTURE_HOLD);
        assert_eq!(mapping.action.enum_value_or_default(), Action::ACTION_NEXT_SONG);

        state.remove_switch_mapping(0);
        let saved = state.draft_preferences_for_save().unwrap();
        assert!(saved.switch.unwrap().mappings.is_empty());
    }
}
