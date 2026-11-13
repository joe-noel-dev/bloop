use std::hash::{Hash, Hasher};

use futures::stream::unfold;
use iced::{
    keyboard::{self, key},
    Subscription,
};
use tokio::sync::{broadcast, mpsc};

use crate::api::client::{create_client_responses, ClientResponses};
use crate::bloop::{AudioControlMethod, Entity, Request, Response, TransportMethod};

use super::{message::Message, state::State};

pub fn update(state: &mut State, message: Message) {
    match message {
        Message::ApiResponse(response) => handle_api_response(state, *response),
        Message::OpenSettings => {
            state.settings.open(state.preferences.clone());
            request_settings_data(state);
        }
        Message::CloseSettings => state.settings.close(),
        Message::RefreshSettings => request_settings_data(state),
        Message::SelectSettingsTab(tab) => {
            state.settings.active_tab = tab;
        }
        Message::SaveSettings => {
            if let Some(preferences) = state.settings.draft_preferences_for_save() {
                state.settings.is_saving = true;
                let request = Request::update_preferences_request(preferences);
                send_request(state.request_tx.clone(), request);
            }
        }
        Message::RestartAudio => {
            let request = Request::audio_control_request(AudioControlMethod::AUDIO_CONTROL_METHOD_RESTART);
            send_request(state.request_tx.clone(), request);
        }
        Message::SetSettingsAudioDevice(option) => state.settings.set_audio_device(option),
        Message::SetSettingsSampleRate(option) => state.settings.set_sample_rate(option),
        Message::SetSettingsAudioNumber(field, value) => state.settings.set_audio_number(field, value),
        Message::SetSettingsUseJack(use_jack) => state.settings.set_use_jack(use_jack),
        Message::SetSettingsMidiPortEnabled(port_name, enabled) => {
            state.settings.set_midi_port_enabled(port_name, enabled);
        }
        Message::AddSettingsSwitchMapping => state.settings.add_switch_mapping(),
        Message::RemoveSettingsSwitchMapping(index) => state.settings.remove_switch_mapping(index),
        Message::SetSettingsSwitchNumber(index, field, value) => {
            state.settings.set_switch_number(index, field, value);
        }
        Message::SetSettingsSwitchPick(index, field, gesture, action) => match field {
            super::settings::SwitchPickField::Gesture => state.settings.set_switch_gesture(index, gesture),
            super::settings::SwitchPickField::Action => state.settings.set_switch_action(index, action),
        },
        Message::StartPlayback => {
            let request = Request::transport_request(TransportMethod::PLAY);
            send_request(state.request_tx.clone(), request);
        }
        Message::StopPlayback => {
            let request = Request::transport_request(TransportMethod::STOP);
            send_request(state.request_tx.clone(), request);
        }
        Message::TogglePlayback => {
            let method = if state.playback_state.is_playing() {
                TransportMethod::STOP
            } else {
                TransportMethod::PLAY
            };

            let request = Request::transport_request(method);
            send_request(state.request_tx.clone(), request);
        }
        Message::SelectPreviousSong => select_song_with_offset(state, -1),
        Message::SelectNextSong => select_song_with_offset(state, 1),
        Message::SelectPreviousSection => select_section_with_offset(state, -1),
        Message::SelectNextSection => select_section_with_offset(state, 1),
        Message::SelectSection(id) => {
            let request = Request::select_request(Entity::SECTION, id);
            send_request(state.request_tx.clone(), request);
        }
        Message::EnterLoop => {
            let request = Request::transport_request(TransportMethod::LOOP);
            send_request(state.request_tx.clone(), request);
        }
        Message::ExitLoop => {
            let request = Request::transport_request(TransportMethod::EXIT_LOOP);
            send_request(state.request_tx.clone(), request);
        }
    }
}

fn request_settings_data(state: &State) {
    for entity in [
        Entity::PREFERENCES,
        Entity::AUDIO_DEVICES,
        Entity::AUDIO_STATUS,
        Entity::MIDI_DEVICES,
    ] {
        send_request(state.request_tx.clone(), Request::get_request(entity, 0));
    }
}

fn select_song_with_offset(state: &State, offset: i64) {
    let current_song_index = match state.project.selected_song_index() {
        Some(index) => index,
        None => return,
    };

    let next_song_index = current_song_index as i64 + offset;
    if next_song_index < 0 || next_song_index >= state.project.songs.len() as i64 {
        return;
    }

    let song = match state.project.song_with_index(next_song_index as usize) {
        Some(song) => song,
        None => return,
    };

    let request = Request::select_request(Entity::SONG, song.id);
    send_request(state.request_tx.clone(), request);
}

fn select_section_with_offset(state: &State, offset: i64) {
    let song = match state.project.selected_song() {
        Some(song) => song,
        None => return,
    };

    let current_section_index = match song
        .sections
        .iter()
        .position(|section| section.id == state.project.selections.section)
    {
        Some(index) => index,
        None => return,
    };

    let next_section_index = current_section_index as i64 + offset;
    if next_section_index < 0 || next_section_index >= song.sections.len() as i64 {
        return;
    }

    let section = match song.sections.get(next_section_index as usize) {
        Some(section) => section,
        None => return,
    };

    let request = Request::select_request(Entity::SECTION, section.id);
    send_request(state.request_tx.clone(), request);
}

struct ResponseSubscription(broadcast::Sender<Response>);

impl Hash for ResponseSubscription {
    fn hash<H: Hasher>(&self, state: &mut H) {
        "api_response_subscription".hash(state);
    }
}

fn build_response_stream(data: &ResponseSubscription) -> impl futures::Stream<Item = Message> {
    let (_, responses) = create_client_responses(data.0.subscribe());
    unfold(responses, async move |mut responses: ClientResponses| loop {
        match responses.recv().await {
            Ok(response) => return Some((Message::ApiResponse(Box::new(response)), responses)),
            Err(broadcast::error::RecvError::Lagged(_)) => continue,
            Err(broadcast::error::RecvError::Closed) => return None,
        }
    })
}

pub fn subscription(state: &State) -> Subscription<Message> {
    Subscription::batch([
        Subscription::run_with(ResponseSubscription(state.response_tx.clone()), build_response_stream),
        keyboard::listen().filter_map(playback_shortcut),
    ])
}

fn playback_shortcut(event: keyboard::Event) -> Option<Message> {
    match event {
        keyboard::Event::KeyPressed {
            key: keyboard::Key::Named(key::Named::Space),
            repeat: false,
            ..
        } => Some(Message::TogglePlayback),
        keyboard::Event::KeyPressed {
            key: keyboard::Key::Named(key::Named::ArrowUp),
            ..
        } => Some(Message::SelectPreviousSection),
        keyboard::Event::KeyPressed {
            key: keyboard::Key::Named(key::Named::ArrowDown),
            ..
        } => Some(Message::SelectNextSection),
        keyboard::Event::KeyPressed {
            key: keyboard::Key::Named(key::Named::ArrowLeft),
            ..
        } => Some(Message::SelectPreviousSong),
        keyboard::Event::KeyPressed {
            key: keyboard::Key::Named(key::Named::ArrowRight),
            ..
        } => Some(Message::SelectNextSong),
        _ => None,
    }
}

fn handle_api_response(state: &mut State, response: Response) {
    if let Some(project) = response.project.as_ref() {
        state.project = project.clone();
    }

    if let Some(playback) = response.playback_state.as_ref() {
        state.playback_state = playback.clone();
    }

    if let Some(progress) = response.progress.as_ref() {
        state.progress = progress.clone();
    }

    if let Some(preferences) = response.preferences.as_ref() {
        state.preferences = Some(preferences.clone());
        if state.settings.is_open {
            if state.settings.is_saving {
                state.settings.close();
            } else {
                state.settings.set_draft(preferences.clone());
            }
        }
    }

    if let Some(audio_devices) = response.audio_devices.as_ref() {
        state.audio_devices = Some(audio_devices.clone());
    }

    if let Some(audio_status) = response.audio_status.as_ref() {
        state.audio_status = Some(audio_status.clone());
    }

    if let Some(midi_devices) = response.midi_devices.as_ref() {
        state.midi_devices = Some(midi_devices.clone());
    }
}

fn send_request(request_tx: mpsc::Sender<Request>, request: Request) {
    tokio::spawn(async move {
        let _ = request_tx.send(request).await;
    });
}
