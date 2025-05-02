mod common;

use bloop::backend::User;
use chrono::DateTime;
use common::BackendFixture;
use serde_json::json;

fn get_user_id() -> String {
    "abc123".to_string()
}

fn get_user_json() -> serde_json::Value {
    serde_json::json!({
        "collectionId": "_pb_users_auth_",
        "collectionName": "users",
        "id": get_user_id(),
        "email": "test@example.com",
        "emailVisibility": true,
        "verified": true,
        "name": "test",
        "avatar": "filename.jpg",
        "created": "2022-01-01 10:00:00.123Z",
        "updated": "2022-02-03 10:00:00.123Z"
    })
}

fn verify_user(user: &User) {
    assert_eq!(user.id, get_user_id());
    assert_eq!(user.email, "test@example.com");
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

    let email = "user@abc.com";
    let password = "password";

    let response = json!(
        {
            "record": get_user_json(),
            "token": "test_token"
        }
    );

    let login_mock = fixture.mock_server.mock(|when, then| {
        when.method("POST").path("/api/collections/users/auth-with-password");
        then.status(200)
            .header("Content-Type", "application/json")
            .body(response.to_string());
    });

    let result = fixture.backend.log_in(email.to_string(), password.to_string()).await;

    let user = result.unwrap();
    verify_user(&user);

    login_mock.assert();
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
    let fixture = BackendFixture::new();

    let mock = fixture.mock_server.mock(|when, then| {
        when.method("GET")
            .path(format!("/api/collections/users/records/{}", get_user_id()));
        then.status(200)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .json_body(get_user_json());
    });

    let result = fixture.backend.get_user(&get_user_id()).await;
    assert!(result.is_ok());

    let user = result.unwrap();
    verify_user(&user);

    mock.assert();
}

#[tokio::test]
async fn test_get_user_unsuccessful() {
    let fixture = BackendFixture::new();

    let mock = fixture.mock_server.mock(|when, then| {
        when.method("GET")
            .path(format!("/api/collections/users/records/{}", get_user_id()));
        then.status(404)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .body(r#"{"status": 404,"message": "The requested resource wasn't found.","data": {}}"#);
    });

    let result = fixture.backend.get_user(&get_user_id()).await;
    assert!(result.is_err());

    mock.assert();
}
