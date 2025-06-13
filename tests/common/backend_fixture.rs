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
    pub backend: Box<dyn Backend>,
    pub auth: Arc<Mutex<dyn Auth + Send + Sync>>,
}

impl BackendFixture {
    pub fn new() -> Self {
        init_logger();
        let temporary_directory = tempfile::TempDir::new().expect("Unable to create temporary directory");
        let mock_server = MockServer::start();
        let base_url = mock_server.base_url();
        let auth = create_pocketbase_auth(Some(base_url.clone()), temporary_directory.path());
        let backend = create_pocketbase_backend(Some(base_url), auth.clone());
        add_login_mock(&mock_server);
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

fn add_login_mock(mock_server: &MockServer) {
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

pub fn user_email() -> String {
    "user@abc.com".to_string()
}

pub fn user_password() -> String {
    "password".to_string()
}

pub fn user_json() -> (String, serde_json::Value) {
    let id = "abc123".to_string();

    let user = serde_json::json!({
        "collectionId": "_pb_users_auth_",
        "collectionName": "users",
        "id": id,
        "email": user_email(),
        "emailVisibility": true,
        "verified": true,
        "name": "test",
        "avatar": "filename.jpg",
        "created": "2022-01-01 10:00:00.123Z",
        "updated": "2022-02-03 10:00:00.123Z"
    });

    (id, user)
}
