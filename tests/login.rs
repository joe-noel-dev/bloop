mod common;

use bloop::backend::DbUser;
use chrono::DateTime;
use common::{user_json, BackendFixture};

fn verify_user(user: &DbUser, id: &str) {
    assert_eq!(user.id, id);
    assert_eq!(user.email, "user@abc.com");
    assert!(user.email_visibility);
    assert_eq!(user.name, "test");

    let expected_created_date = DateTime::parse_from_rfc3339("2022-01-01 10:00:00.123Z").unwrap();
    assert_eq!(user.created, expected_created_date);

    let expected_updated_date = DateTime::parse_from_rfc3339("2022-02-03 10:00:00.123Z").unwrap();
    assert_eq!(user.updated, expected_updated_date);
}

#[tokio::test]
async fn test_successful_log_in() {
    let mut fixture = BackendFixture::new();
    let (id, _) = user_json();
    let user = fixture.log_in().await;
    verify_user(&user, &id);
}

#[tokio::test]
async fn test_unsuccessful_log_in() {
    let mut fixture = BackendFixture::new();

    let email = "user@abc.com";
    let password = "wrong_password";

    let login_mock = fixture.mock_server.mock(|when, then| {
        when.method("POST").path("/api/collections/users/auth-with-password");
        then.status(401)
            .header("Content-Type", "application/json")
            .body(r#"{"message": "Invalid credentials"}"#);
    });

    let result = fixture.backend.log_in(email.to_string(), password.to_string()).await;

    assert!(result.is_err());
    login_mock.assert();
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

    let result = fixture.backend.get_user(&id).await;
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

    let result = fixture.backend.get_user(&id).await;
    assert!(result.is_err());

    mock.assert();
}
