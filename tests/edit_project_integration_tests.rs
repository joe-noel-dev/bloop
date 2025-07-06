mod common;

use bloop::bloop::{Entity, Request};
use common::IntegrationFixture;

#[tokio::test]
async fn add_song_request() {
    let mut fixture = IntegrationFixture::new().await;

    let request = Request::get_request(Entity::ALL, 0);
    fixture.send_request(request).await;

    let response = fixture
        .wait_for_response(|response| response.error.is_empty() && response.project.is_some())
        .await
        .expect("Didn't receive get response");

    let project = response.project.as_ref().expect("Project should be present");

    let song_count_before = project.songs.len();

    let add_song_request = Request::add_song_request();
    fixture.send_request(add_song_request).await;

    let response = fixture
        .wait_for_response(|response| response.error.is_empty() && response.project.is_some())
        .await
        .expect("Didn't receive add song response");

    let updated_project = response.project.as_ref().expect("Updated project should be present");
    let song_count_after = updated_project.songs.len();

    assert_eq!(
        song_count_after,
        song_count_before + 1,
        "Song count should increase by 1"
    );
}
