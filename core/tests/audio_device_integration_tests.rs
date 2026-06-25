mod common;

use bloop::bloop::{AudioControlMethod, AudioEngineStatus, Entity, Request};
use common::IntegrationFixture;

#[tokio::test]
async fn get_audio_devices_returns_audio_devices_response() {
    let mut fixture = IntegrationFixture::new().await;

    fixture
        .send_request(Request::get_request(Entity::AUDIO_DEVICES, 0))
        .await;

    fixture
        .wait_for_response(|response| response.audio_devices.is_some())
        .await
        .expect("Did not receive an AudioDevices response");
}

#[tokio::test]
async fn get_audio_devices_includes_host_name() {
    let mut fixture = IntegrationFixture::new().await;

    fixture
        .send_request(Request::get_request(Entity::AUDIO_DEVICES, 0))
        .await;

    let response = fixture
        .wait_for_response(|response| response.audio_devices.is_some())
        .await
        .expect("Did not receive an AudioDevices response");

    let audio_devices = response.audio_devices.unwrap();
    assert!(!audio_devices.host_name.is_empty(), "Expected a non-empty host name");
}

#[tokio::test]
async fn get_audio_status_returns_audio_status_response() {
    let mut fixture = IntegrationFixture::new().await;

    fixture
        .send_request(Request::get_request(Entity::AUDIO_STATUS, 0))
        .await;

    fixture
        .wait_for_response(|response| response.audio_status.is_some())
        .await
        .expect("Did not receive an AudioStatus response");
}

#[tokio::test]
async fn get_all_includes_audio_status() {
    let mut fixture = IntegrationFixture::new().await;

    fixture.send_request(Request::get_request(Entity::ALL, 0)).await;

    fixture
        .wait_for_response(|response| response.audio_status.is_some())
        .await
        .expect("GET ALL response did not include AudioStatus");
}

#[tokio::test]
async fn audio_control_stop_broadcasts_stopped_status() {
    let mut fixture = IntegrationFixture::new().await;

    fixture
        .send_request(Request::audio_control_request(
            AudioControlMethod::AUDIO_CONTROL_METHOD_STOP,
        ))
        .await;

    let response = fixture
        .wait_for_response(|response| {
            response
                .audio_status
                .as_ref()
                .map(|s| s.engine_status.enum_value_or_default() == AudioEngineStatus::AUDIO_ENGINE_STATUS_STOPPED)
                .unwrap_or(false)
        })
        .await
        .expect("Did not receive STOPPED AudioStatus after AUDIO_CONTROL_METHOD_STOP");

    assert_eq!(
        response.audio_status.unwrap().engine_status.enum_value_or_default(),
        AudioEngineStatus::AUDIO_ENGINE_STATUS_STOPPED
    );
}

#[tokio::test]
async fn audio_control_start_after_stop_broadcasts_running_status() {
    let mut fixture = IntegrationFixture::new().await;

    fixture
        .send_request(Request::audio_control_request(
            AudioControlMethod::AUDIO_CONTROL_METHOD_STOP,
        ))
        .await;

    fixture
        .wait_for_response(|response| {
            response
                .audio_status
                .as_ref()
                .map(|s| s.engine_status.enum_value_or_default() == AudioEngineStatus::AUDIO_ENGINE_STATUS_STOPPED)
                .unwrap_or(false)
        })
        .await
        .expect("Did not receive STOPPED status");

    fixture
        .send_request(Request::audio_control_request(
            AudioControlMethod::AUDIO_CONTROL_METHOD_START,
        ))
        .await;

    let response = fixture
        .wait_for_response(|response| {
            response
                .audio_status
                .as_ref()
                .map(|s| s.engine_status.enum_value_or_default() == AudioEngineStatus::AUDIO_ENGINE_STATUS_RUNNING)
                .unwrap_or(false)
        })
        .await
        .expect("Did not receive RUNNING AudioStatus after AUDIO_CONTROL_METHOD_START");

    assert_eq!(
        response.audio_status.unwrap().engine_status.enum_value_or_default(),
        AudioEngineStatus::AUDIO_ENGINE_STATUS_RUNNING
    );
}

#[tokio::test]
async fn audio_control_restart_broadcasts_running_status() {
    let mut fixture = IntegrationFixture::new().await;

    fixture
        .send_request(Request::audio_control_request(
            AudioControlMethod::AUDIO_CONTROL_METHOD_RESTART,
        ))
        .await;

    let response = fixture
        .wait_for_response(|response| {
            response
                .audio_status
                .as_ref()
                .map(|s| s.engine_status.enum_value_or_default() == AudioEngineStatus::AUDIO_ENGINE_STATUS_RUNNING)
                .unwrap_or(false)
        })
        .await
        .expect("Did not receive RUNNING AudioStatus after AUDIO_CONTROL_METHOD_RESTART");

    assert_eq!(
        response.audio_status.unwrap().engine_status.enum_value_or_default(),
        AudioEngineStatus::AUDIO_ENGINE_STATUS_RUNNING
    );
}
