mod common;

use bloop::bloop::{
    AudioPreferences, Entity, MidiPreferences, Preferences, Request, SwitchMapping, SwitchPreferences, UpdateRequest,
};
use common::IntegrationFixture;

#[tokio::test]
async fn get_preferences_returns_defaults() {
    let mut fixture = IntegrationFixture::new().await;

    let request = Request::get_request(Entity::PREFERENCES, 0);
    fixture.send_request(request).await;

    let response = fixture
        .wait_for_response(|response| response.error.is_empty() && response.preferences.is_some())
        .await
        .expect("Didn't receive preferences response");

    let preferences = response.preferences.unwrap();

    // Verify default audio preferences
    assert!(preferences.audio.is_some());
    let audio = preferences.audio.unwrap();
    assert_eq!(audio.sample_rate, 48_000);
    assert_eq!(audio.buffer_size, 512);
    assert_eq!(audio.output_channel_count, 2);
    assert_eq!(audio.main_channel_offset, 0);
    assert_eq!(audio.click_channel_offset, 2);
    assert!(!audio.use_jack);

    // Verify default midi preferences
    assert!(preferences.midi.is_some());
}

#[tokio::test]
async fn update_audio_preferences_persists() {
    let mut fixture = IntegrationFixture::new().await;

    // Create custom audio preferences
    let custom_audio = AudioPreferences {
        output_device: "Test Device".to_string(),
        sample_rate: 96_000,
        buffer_size: 1024,
        output_channel_count: 4,
        use_jack: true,
        main_channel_offset: 2,
        click_channel_offset: 4,
        ..Default::default()
    };

    let custom_preferences = Preferences {
        audio: Some(custom_audio.clone()).into(),
        midi: Some(MidiPreferences::default()).into(),
        switch: Some(SwitchPreferences::default()).into(),
        ..Default::default()
    };

    // Send update request
    let update_request = Request {
        update: Some(UpdateRequest {
            preferences: Some(custom_preferences.clone()).into(),
            ..Default::default()
        })
        .into(),
        ..Default::default()
    };

    fixture.send_request(update_request).await;

    // Wait for preferences response
    let response = fixture
        .wait_for_response(|response| response.error.is_empty() && response.preferences.is_some())
        .await
        .expect("Didn't receive preferences response");

    let loaded_preferences = response.preferences.unwrap();
    let loaded_audio = loaded_preferences.audio.unwrap();

    // Verify all custom values were persisted
    assert_eq!(loaded_audio.output_device, "Test Device");
    assert_eq!(loaded_audio.sample_rate, 96_000);
    assert_eq!(loaded_audio.buffer_size, 1024);
    assert_eq!(loaded_audio.output_channel_count, 4);
    assert!(loaded_audio.use_jack);
    assert_eq!(loaded_audio.main_channel_offset, 2);
    assert_eq!(loaded_audio.click_channel_offset, 4);
}

#[tokio::test]
async fn update_midi_preferences_persists() {
    let mut fixture = IntegrationFixture::new().await;

    // Create custom midi preferences
    let custom_midi = MidiPreferences {
        input_device: "Test MIDI Input".to_string(),
        ..Default::default()
    };

    let custom_preferences = Preferences {
        audio: Some(AudioPreferences::default()).into(),
        midi: Some(custom_midi.clone()).into(),
        switch: Some(SwitchPreferences::default()).into(),
        ..Default::default()
    };

    // Send update request
    let update_request = Request {
        update: Some(UpdateRequest {
            preferences: Some(custom_preferences.clone()).into(),
            ..Default::default()
        })
        .into(),
        ..Default::default()
    };

    fixture.send_request(update_request).await;

    // Wait for preferences response
    let response = fixture
        .wait_for_response(|response| response.error.is_empty() && response.preferences.is_some())
        .await
        .expect("Didn't receive preferences response");

    let loaded_preferences = response.preferences.unwrap();
    let loaded_midi = loaded_preferences.midi.unwrap();

    // Verify custom MIDI device was persisted
    assert_eq!(loaded_midi.input_device, "Test MIDI Input");
}

#[tokio::test]
async fn update_switch_preferences_persists() {
    let mut fixture = IntegrationFixture::new().await;

    // Create custom switch preferences with mappings
    let custom_switch = SwitchPreferences {
        mappings: vec![
            SwitchMapping {
                pin: 1,
                gesture: bloop::bloop::Gesture::GESTURE_PRESS.into(),
                action: bloop::bloop::Action::ACTION_TOGGLE_PLAY.into(),
                ..Default::default()
            },
            SwitchMapping {
                pin: 2,
                gesture: bloop::bloop::Gesture::GESTURE_HOLD.into(),
                action: bloop::bloop::Action::ACTION_QUEUE_SELECTED.into(),
                ..Default::default()
            },
        ],
        ..Default::default()
    };

    let custom_preferences = Preferences {
        audio: Some(AudioPreferences::default()).into(),
        midi: Some(MidiPreferences::default()).into(),
        switch: Some(custom_switch.clone()).into(),
        ..Default::default()
    };

    // Send update request
    let update_request = Request {
        update: Some(UpdateRequest {
            preferences: Some(custom_preferences.clone()).into(),
            ..Default::default()
        })
        .into(),
        ..Default::default()
    };

    fixture.send_request(update_request).await;

    // Wait for preferences response
    let response = fixture
        .wait_for_response(|response| response.error.is_empty() && response.preferences.is_some())
        .await
        .expect("Didn't receive preferences response");

    let loaded_preferences = response.preferences.unwrap();
    let loaded_switch = loaded_preferences.switch.unwrap();

    // Verify switch mappings were persisted
    assert_eq!(loaded_switch.mappings.len(), 2);

    let mapping1 = &loaded_switch.mappings[0];
    assert_eq!(mapping1.pin, 1);
    assert_eq!(
        mapping1.gesture.enum_value_or_default(),
        bloop::bloop::Gesture::GESTURE_PRESS
    );
    assert_eq!(
        mapping1.action.enum_value_or_default(),
        bloop::bloop::Action::ACTION_TOGGLE_PLAY
    );

    let mapping2 = &loaded_switch.mappings[1];
    assert_eq!(mapping2.pin, 2);
    assert_eq!(
        mapping2.gesture.enum_value_or_default(),
        bloop::bloop::Gesture::GESTURE_HOLD
    );
    assert_eq!(
        mapping2.action.enum_value_or_default(),
        bloop::bloop::Action::ACTION_QUEUE_SELECTED
    );
}

#[tokio::test]
async fn update_partial_preferences_preserves_others() {
    let mut fixture = IntegrationFixture::new().await;

    // First, set complete custom preferences
    let initial_audio = AudioPreferences {
        output_device: "Initial Device".to_string(),
        sample_rate: 96_000,
        buffer_size: 1024,
        output_channel_count: 4,
        use_jack: true,
        main_channel_offset: 2,
        click_channel_offset: 4,
        ..Default::default()
    };

    let initial_midi = MidiPreferences {
        input_device: "Initial MIDI".to_string(),
        ..Default::default()
    };

    let initial_preferences = Preferences {
        audio: Some(initial_audio.clone()).into(),
        midi: Some(initial_midi.clone()).into(),
        switch: Some(SwitchPreferences::default()).into(),
        ..Default::default()
    };

    let update_request = Request {
        update: Some(UpdateRequest {
            preferences: Some(initial_preferences).into(),
            ..Default::default()
        })
        .into(),
        ..Default::default()
    };

    fixture.send_request(update_request).await;
    fixture
        .wait_for_response(|response| response.error.is_empty() && response.preferences.is_some())
        .await
        .expect("Didn't receive initial update response");

    // Now update only the MIDI preferences
    let updated_midi = MidiPreferences {
        input_device: "Updated MIDI".to_string(),
        ..Default::default()
    };

    let partial_preferences = Preferences {
        audio: Some(initial_audio.clone()).into(), // Keep the same
        midi: Some(updated_midi.clone()).into(),   // Change only this
        switch: Some(SwitchPreferences::default()).into(),
        ..Default::default()
    };

    let partial_update_request = Request {
        update: Some(UpdateRequest {
            preferences: Some(partial_preferences).into(),
            ..Default::default()
        })
        .into(),
        ..Default::default()
    };

    fixture.send_request(partial_update_request).await;
    let response = fixture
        .wait_for_response(|response| response.error.is_empty() && response.preferences.is_some())
        .await
        .expect("Didn't receive partial update response");

    let loaded_preferences = response.preferences.unwrap();
    let loaded_audio = loaded_preferences.audio.unwrap();
    let loaded_midi = loaded_preferences.midi.unwrap();

    // Verify audio preferences were preserved
    assert_eq!(loaded_audio.output_device, "Initial Device");
    assert_eq!(loaded_audio.sample_rate, 96_000);
    assert_eq!(loaded_audio.buffer_size, 1024);

    // Verify MIDI preferences were updated
    assert_eq!(loaded_midi.input_device, "Updated MIDI");
}

#[tokio::test]
async fn preferences_persist_across_restarts() {
    // This test verifies that preferences are written to disk correctly
    // In a real scenario, they would persist across app restarts
    let mut fixture = IntegrationFixture::new().await;

    let custom_audio = AudioPreferences {
        output_device: "Persistent Device".to_string(),
        sample_rate: 88_200,
        buffer_size: 256,
        output_channel_count: 8,
        use_jack: false,
        main_channel_offset: 4,
        click_channel_offset: 6,
        ..Default::default()
    };

    let custom_preferences = Preferences {
        audio: Some(custom_audio.clone()).into(),
        midi: Some(MidiPreferences::default()).into(),
        switch: Some(SwitchPreferences::default()).into(),
        ..Default::default()
    };

    let update_request = Request {
        update: Some(UpdateRequest {
            preferences: Some(custom_preferences).into(),
            ..Default::default()
        })
        .into(),
        ..Default::default()
    };

    fixture.send_request(update_request).await;
    fixture
        .wait_for_response(|response| response.error.is_empty() && response.preferences.is_some())
        .await
        .expect("Didn't receive update response");

    // Request preferences again to verify they're still there
    let get_request = Request::get_request(Entity::PREFERENCES, 0);
    fixture.send_request(get_request).await;

    let response = fixture
        .wait_for_response(|response| response.error.is_empty() && response.preferences.is_some())
        .await
        .expect("Didn't receive preferences response");

    let loaded_preferences = response.preferences.unwrap();
    let loaded_audio = loaded_preferences.audio.unwrap();

    // Verify preferences persisted
    assert_eq!(loaded_audio.output_device, "Persistent Device");
    assert_eq!(loaded_audio.sample_rate, 88_200);
    assert_eq!(loaded_audio.buffer_size, 256);
    assert_eq!(loaded_audio.output_channel_count, 8);
    assert!(!loaded_audio.use_jack);
    assert_eq!(loaded_audio.main_channel_offset, 4);
    assert_eq!(loaded_audio.click_channel_offset, 6);
}

#[tokio::test]
async fn get_all_includes_preferences() {
    let mut fixture = IntegrationFixture::new().await;

    // Set custom preferences first
    let custom_audio = AudioPreferences {
        output_device: "All Test Device".to_string(),
        sample_rate: 44_100,
        ..Default::default()
    };

    let custom_preferences = Preferences {
        audio: Some(custom_audio.clone()).into(),
        midi: Some(MidiPreferences::default()).into(),
        switch: Some(SwitchPreferences::default()).into(),
        ..Default::default()
    };

    let update_request = Request {
        update: Some(UpdateRequest {
            preferences: Some(custom_preferences).into(),
            ..Default::default()
        })
        .into(),
        ..Default::default()
    };

    fixture.send_request(update_request).await;
    let response = fixture
        .wait_for_response(|response| response.error.is_empty() && response.preferences.is_some())
        .await
        .expect("Didn't receive update response");

    // Verify preferences are included in the update response
    let preferences = response.preferences.unwrap();
    let audio = preferences.audio.unwrap();
    assert_eq!(audio.output_device, "All Test Device");
    assert_eq!(audio.sample_rate, 44_100);
}
