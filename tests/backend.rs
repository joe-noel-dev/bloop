use bloop::backend::{create_pocketbase_backend, Backend};
use httpmock::MockServer;
use iced::Length::Fill;
use std::sync::Once;

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

struct Fixture {
    mock_server: MockServer,
    backend: Box<dyn Backend>,
}

impl Fixture {
    fn new() -> Self {
        init_logger();
        let mock_server = MockServer::start();
        let base_url = mock_server.base_url();
        let mut backend = create_pocketbase_backend(Some(base_url));

        Self { mock_server, backend }
    }
}

#[tokio::test]
async fn test_successful_log_in() {
    let mut fixture = Fixture::new();

    let email = "user@abc.com";
    let password = "password";

    let login_mock = fixture.mock_server.mock(|when, then| {
        when.method("POST").path("/api/collections/users/auth-with-password");
        then.status(200)
            .header("Content-Type", "application/json")
            .body(r#"{"record": { "name": "Name of User" }, "token":"test_token"}"#);
    });

    let result = fixture.backend.log_in(email.to_string(), password.to_string()).await;

    assert!(result.is_ok());
    login_mock.assert();
}

#[tokio::test]
async fn test_unsuccessful_log_in() {
    let mut fixture = Fixture::new();

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
