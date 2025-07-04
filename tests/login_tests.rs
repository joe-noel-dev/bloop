mod common;

use bloop::{backend::DbUser, bloop::Request};
use chrono::DateTime;
use common::{user_json, BackendFixture};

use crate::common::{user_email, user_id, user_name, user_password, IntegrationFixture};

fn verify_user(user: &DbUser, id: &str) {
    assert_eq!(user.id, id);
    assert_eq!(user.email, user_email());
    assert!(user.email_visibility);
    assert_eq!(user.name, user_name());

    let expected_created_date = DateTime::parse_from_rfc3339("2022-01-01 10:00:00.123Z").unwrap();
    assert_eq!(user.created, expected_created_date);

    let expected_updated_date = DateTime::parse_from_rfc3339("2022-02-03 10:00:00.123Z").unwrap();
    assert_eq!(user.updated, expected_updated_date);
}

#[tokio::test]
async fn test_successful_log_in() {
    let mut fixture = IntegrationFixture::new();

    let request = Request::log_in_request(user_email(), user_password());
    fixture.send_request(request).await;

    let response = fixture
        .wait_for_response(|response| {
            if let Some(user_state) = response.user_status.as_ref() {
                if user_state.user.is_some() {
                    return true;
                }
            }

            false
        })
        .await
        .expect("Didn't receive user state");

    let user = response.user_status.unwrap().user.unwrap();

    assert_eq!(user.name, user_name());
    assert_eq!(user.email, user_email());
    assert_eq!(user.id, user_id());
}

#[tokio::test]
async fn test_unsuccessful_login() {
    let mut fixture = IntegrationFixture::new();

    let request = Request::log_in_request("user@abc.com".to_string(), "wrong_password".to_string());
    fixture.send_request(request).await;

    fixture
        .wait_for_response(|response| !response.error.is_empty())
        .await
        .expect("Didn't receive error response");
}

#[tokio::test]
async fn test_get_user_successful() {
    let mut fixture = BackendFixture::new();

    fixture.log_in().await;

    let (id, user_json) = user_json();

    let mock = fixture.mock_server.mock(|when, then| {
        when.method("GET")
            .path(format!("/api/collections/users/records/{}", id));
        then.status(200)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .json_body(user_json);
    });

    let result = fixture.auth.lock().await.get_user(&id).await;
    assert!(result.is_ok());

    let user = result.unwrap();
    verify_user(&user, &id);

    mock.assert();
}

#[tokio::test]
async fn test_get_user_unsuccessful() {
    let mut fixture = BackendFixture::new();

    fixture.log_in().await;

    let (id, _) = user_json();

    let mock = fixture.mock_server.mock(|when, then| {
        when.method("GET")
            .path(format!("/api/collections/users/records/{}", id));
        then.status(404)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .body(r#"{"status": 404,"message": "The requested resource wasn't found.","data": {}}"#);
    });

    let result = fixture.auth.lock().await.get_user(&id).await;
    assert!(result.is_err());

    mock.assert();
}
