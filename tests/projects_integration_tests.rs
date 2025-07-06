mod common;

use bloop::bloop::{Entity, Request};
use common::IntegrationFixture;

#[tokio::test]
async fn get_projects_request() {
    let mut fixture = IntegrationFixture::new().await;

    let request = Request::get_request(Entity::ALL, 0);
    fixture.send_request(request).await;
    fixture
        .wait_for_response(|response| response.error.is_empty())
        .await
        .expect("Didn't receive get response");
}

#[tokio::test]
async fn get_projects_request_with_saved_project() {
    let mut fixture = IntegrationFixture::new().await;

    // First, save a project to ensure we have something to retrieve
    let save_request = Request::save_project_request();
    fixture.send_request(save_request).await;
    fixture
        .wait_for_response(|response| response.error.is_empty())
        .await
        .expect("Didn't receive save project response");

    let get_request = Request::get_request(Entity::PROJECTS, 0);
    fixture.send_request(get_request).await;
    fixture
        .wait_for_response(|response| response.error.is_empty() && !response.projects.is_empty())
        .await
        .expect("Didn't receive get projects response");
}
