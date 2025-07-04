mod common;

use bloop::bloop::Request;

use crate::common::{IntegrationFixture, MockUser};

#[tokio::test]
async fn test_successful_log_in() {
    let mut fixture = IntegrationFixture::new().await;

    let user = MockUser::new("user-id", "hello@123.com", "password123", "Test User");

    fixture.mocketbase().add_user(user.clone()).await;

    let request = Request::log_in_request(user.email, user.password);
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

    assert_eq!(user.name, user.name);
    assert_eq!(user.email, user.email);
    assert_eq!(user.id, user.id);
}

#[tokio::test]
async fn test_unsuccessful_login() {
    let mut fixture = IntegrationFixture::new().await;

    let request = Request::log_in_request("user@abc.com".to_string(), "wrong_password".to_string());
    fixture.send_request(request).await;

    fixture
        .wait_for_response(|response| !response.error.is_empty())
        .await
        .expect("Didn't receive error response");
}
