mod common;

use bloop::bloop::{Entity, MidiDevices, Request};
use common::IntegrationFixture;
use protobuf::Message;

#[tokio::test]
async fn get_midi_devices_returns_response() {
    let mut fixture = IntegrationFixture::new().await;

    let request = Request::get_request(Entity::MIDI_DEVICES, 0);
    fixture.send_request(request).await;

    let response = fixture
        .wait_for_response(|response| response.error.is_empty() && response.midi_devices.is_some())
        .await
        .expect("Didn't receive midi_devices response");

    // The list may be empty on CI; we just confirm the field is present.
    assert!(response.midi_devices.is_some());
}

#[test]
fn midi_devices_round_trip_serialization() {
    let original = MidiDevices {
        port_names: vec!["iCON G_Boar V1.03".to_string(), "USB MIDI Interface".to_string()],
        ..Default::default()
    };

    let bytes = original.write_to_bytes().expect("Failed to serialize MidiDevices");
    let decoded = MidiDevices::parse_from_bytes(&bytes).expect("Failed to deserialize MidiDevices");

    assert_eq!(decoded.port_names, original.port_names);
}
