use std::sync::{Arc, Once};

use bloop::backend::{create_pocketbase_auth, create_pocketbase_backend, Auth, Backend, DbUser};
use httpmock::MockServer;
use serde_json::json;
use tokio::sync::Mutex;

static INIT: Once = Once::new();

fn init_logger() {
    INIT.call_once(|| {
        env_logger::builder()
            .is_test(true)
            .filter_level(log::LevelFilter::Debug)
            .try_init()
            .ok();
    });
}

pub struct BackendFixture {
    pub temporary_directory: tempfile::TempDir,
    pub mock_server: MockServer,
    pub backend: Arc<dyn Backend>,
    pub auth: Arc<Mutex<dyn Auth + Send + Sync>>,
}

impl BackendFixture {
    pub fn new() -> Self {
        init_logger();
        let temporary_directory = tempfile::TempDir::new().expect("Unable to create temporary directory");
        let mock_server = MockServer::start();
        let base_url = mock_server.base_url();
        let auth = create_pocketbase_auth(base_url.clone(), temporary_directory.path());
        let backend = create_pocketbase_backend(base_url, auth.clone());

        add_valid_login_mock(&mock_server);
        add_invalid_login_mock(&mock_server);

        Self {
            temporary_directory,
            mock_server,
            backend,
            auth,
        }
    }

    pub async fn log_in(&mut self) -> DbUser {
        let email = user_email();
        let password = user_password();

        self.auth
            .lock()
            .await
            .log_in(email.to_string(), password.to_string())
            .await
            .expect("Failed to log in")
    }
}

fn add_valid_login_mock(mock_server: &MockServer) {
    let (_, user_json) = user_json();

    let response = json!(
        {
            "record": user_json,
            "token": "test_token"
        }
    );

    mock_server.mock(|when, then| {
        when.method("POST")
            .path("/api/collections/users/auth-with-password")
            .json_body(json!({
                "identity": user_email(),
                "password": user_password()
            }));

        then.status(200)
            .header("Content-Type", "application/json")
            .body(response.to_string());
    });
}

fn add_invalid_login_mock(mock_server: &MockServer) {
    let error_response = json!({
        "code": 400,
        "message": "Failed to authenticate.",
        "data": {
            "identity": {
                "code": "validation_invalid_login",
                "message": "Invalid login credentials."
            }
        }
    });

    mock_server.mock(|when, then| {
        when.method("POST").path("/api/collections/users/auth-with-password");

        then.status(400)
            .header("Content-Type", "application/json")
            .body(error_response.to_string());
    });
}

pub fn user_email() -> String {
    "user@abc.com".to_string()
}

pub fn user_password() -> String {
    "password".to_string()
}

pub fn user_name() -> String {
    "Test User".to_string()
}

pub fn user_id() -> String {
    "abc123".to_string()
}

pub fn user_json() -> (String, serde_json::Value) {
    let id = "abc123".to_string();

    let user = serde_json::json!({
        "collectionId": "_pb_users_auth_",
        "collectionName": "users",
        "id": user_id(),
        "email": user_email(),
        "emailVisibility": true,
        "verified": true,
        "name": user_name(),
        "avatar": "filename.jpg",
        "created": "2022-01-01 10:00:00.123Z",
        "updated": "2022-02-03 10:00:00.123Z"
    });

    (id, user)
}
