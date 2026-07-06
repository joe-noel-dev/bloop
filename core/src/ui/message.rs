use crate::{bloop::Response, model::ID};

use super::settings::{
    ActionOption, AudioDeviceOption, AudioNumberField, GestureOption, SampleRateOption, SettingsTab, SwitchNumberField,
    SwitchPickField,
};

#[derive(Debug, Clone)]
pub enum Message {
    ApiResponse(Box<Response>),
    StartPlayback,
    StopPlayback,
    TogglePlayback,
    EnterLoop,
    ExitLoop,
    SelectPreviousSong,
    SelectNextSong,
    SelectPreviousSection,
    SelectNextSection,
    #[allow(unused)]
    SelectSection(ID),
    OpenSettings,
    CloseSettings,
    RefreshSettings,
    SelectSettingsTab(SettingsTab),
    SaveSettings,
    RestartAudio,
    SetSettingsAudioDevice(AudioDeviceOption),
    SetSettingsSampleRate(SampleRateOption),
    SetSettingsAudioNumber(AudioNumberField, String),
    SetSettingsUseJack(bool),
    SetSettingsMidiPortEnabled(String, bool),
    AddSettingsSwitchMapping,
    RemoveSettingsSwitchMapping(usize),
    SetSettingsSwitchNumber(usize, SwitchNumberField, String),
    SetSettingsSwitchPick(usize, SwitchPickField, GestureOption, ActionOption),
}
