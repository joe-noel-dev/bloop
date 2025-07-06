mod common;

use bloop::bloop::Request;

use crate::common::{IntegrationFixture, MockProject, MockUser};

#[tokio::test]
async fn list_projects_successful() {
    let mut fixture = IntegrationFixture::new().await;

    let mut project1 = MockProject::new("project-1", "Project 1", "user-id");
    project1.created = "2022-01-01 10:00:00.123Z".to_string();
    project1.updated = "2022-01-02 11:00:00.123Z".to_string();
    let mut project2 = MockProject::new("project-2", "Project 2", "user-id");
    project2.created = "2022-01-03 12:00:00.123Z".to_string();
    project2.updated = "2022-01-04 13:00:00.123Z".to_string();

    fixture.mocketbase().set_projects(vec![project1, project2]).await;

    fixture
        .mocketbase()
        .add_user(MockUser::new("user-id", "user@123.com", "password", "name"))
        .await;

    fixture
        .send_request(Request::log_in_request(
            "user@123.com".to_string(),
            "password".to_string(),
        ))
        .await;

    fixture
        .wait_for_response(|response| {
            response.user_status.is_some() && response.user_status.as_ref().unwrap().user.is_some()
        })
        .await
        .expect("Didn't receive user state ");

    fixture
        .send_request(Request::get_request(bloop::bloop::Entity::PROJECTS, 0))
        .await;

    let response = fixture
        .wait_for_response(|response| !response.cloud_projects.is_empty())
        .await
        .expect("Didn't receive projects");

    let projects = response.cloud_projects;
    assert_eq!(projects.len(), 2);

    assert_eq!(projects[0].id, "project-1");
    assert_eq!(projects[0].name, "Project 1");
    assert_eq!(projects[0].last_saved, "2022-01-02T11:00:00.123+00:00");

    assert_eq!(projects[1].id, "project-2");
    assert_eq!(projects[1].name, "Project 2");
    assert_eq!(projects[1].last_saved, "2022-01-04T13:00:00.123+00:00");
}

// #[tokio::test]
// async fn get_project_successful() {
//     let mut fixture = BackendFixture::new();
//     fixture.log_in().await;

//     let get_project_response = r#"
//     {
//         "collectionId": "pbc_484305853",
//         "collectionName": "projects",
//         "id": "test",
//         "name": "Test Project",
//         "userId": "user_id",
//         "project": "project.bin",
//         "samples": [
//             "sample.wav"
//         ],
//         "created": "2022-01-02 10:00:00.123Z",
//         "updated": "2022-03-04 10:00:00.123Z"
//     }
//     "#;

//     let mock = fixture.mock_server.mock(|when, then| {
//         when.method("GET").path("/api/collections/projects/records/test");
//         then.status(200)
//             .header("Content-Type", "application/json")
//             .header("Accept", "application/json")
//             .body(get_project_response);
//     });

//     let project = fixture.backend.read_project("test").await.unwrap();

//     assert_eq!(project.id, "test");
//     assert_eq!(project.name, "Test Project");
//     assert_eq!(project.user_id, "user_id");
//     assert_eq!(
//         project.created,
//         DateTime::parse_from_rfc3339("2022-01-02 10:00:00.123Z").unwrap()
//     );
//     assert_eq!(
//         project.updated,
//         DateTime::parse_from_rfc3339("2022-03-04 10:00:00.123Z").unwrap()
//     );

//     mock.assert();
// }

// #[tokio::test]
// async fn create_project() {
//     let mut fixture = BackendFixture::new();
//     fixture.log_in().await;

//     let create_project_response = r#"
//     {
//         "collectionId": "pbc_484305853",
//         "collectionName": "projects",
//         "id": "test",
//         "name": "Project Name",
//         "userId": "user_id",
//         "project": "project.bin",
//         "samples": [
//             "sample.wav"
//         ],
//         "created": "2022-01-02 10:00:00.123Z",
//         "updated": "2022-03-04 10:00:00.123Z"
//     }
//     "#;

//     let mock = fixture.mock_server.mock(|when, then| {
//         when.method("POST").path("/api/collections/projects/records");
//         then.status(200)
//             .header("Content-Type", "application/json")
//             .header("Accept", "application/json")
//             .body(create_project_response);
//     });

//     let created_project = fixture.backend.create_project("user_id").await.unwrap();

//     assert_eq!(created_project.id, "test");
//     assert_eq!(created_project.name, "Project Name");
//     assert_eq!(created_project.user_id, "user_id");
//     assert_eq!(
//         created_project.created,
//         DateTime::parse_from_rfc3339("2022-01-02 10:00:00.123Z").unwrap()
//     );
//     assert_eq!(
//         created_project.updated,
//         DateTime::parse_from_rfc3339("2022-03-04 10:00:00.123Z").unwrap()
//     );

//     mock.assert();
// }

// #[tokio::test]
// async fn update_project_name() {
//     let mut fixture = BackendFixture::new();
//     fixture.log_in().await;

//     let response_body = r#"
//     {
//         "collectionId": "pbc_484305853",
//         "collectionName": "projects",
//         "id": "project_id",
//         "name": "Updated Project Name",
//         "userId": "user_id",
//         "project": "project.bin",
//         "samples": ["sample.wav"],
//         "created": "2022-01-02 10:00:00.123Z",
//         "updated": "2022-05-04 10:00:00.123Z"
//     }
//     "#;

//     let mock = fixture.mock_server.mock(|when, then| {
//         when.method("PATCH")
//             .path("/api/collections/projects/records/project_id")
//             .body_contains("name")
//             .body_contains("Updated Project Name");
//         then.status(200)
//             .header("Content-Type", "application/json")
//             .header("Accept", "application/json")
//             .body(response_body);
//     });

//     let updated_project = fixture
//         .backend
//         .update_project_name("project_id", "Updated Project Name")
//         .await
//         .unwrap();

//     assert_eq!(updated_project.name, "Updated Project Name");

//     mock.assert();
// }

// #[tokio::test]
// async fn update_project_file() {
//     let mut fixture = BackendFixture::new();
//     fixture.log_in().await;

//     let response_body = r#"
//     {
//         "collectionId": "pbc_484305853",
//         "collectionName": "projects",
//         "id": "project_id",
//         "name": "Project Name",
//         "userId": "user_id",
//         "project": "updated_project.bin",
//         "samples": ["sample.wav"],
//         "created": "2022-01-02 10:00:00.123Z",
//         "updated": "2022-05-04 10:00:00.123Z"
//     }
//     "#;

//     let mock = fixture.mock_server.mock(|when, then| {
//         when.method("PATCH")
//             .path("/api/collections/projects/records/project_id")
//             .body_contains("project")
//             .body_contains("project.bin");
//         then.status(200)
//             .header("Content-Type", "application/json")
//             .header("Accept", "application/json")
//             .body(response_body);
//     });

//     fixture
//         .backend
//         .update_project_file("project_id", b"updated project bytes")
//         .await
//         .unwrap();

//     mock.assert();
// }

// #[tokio::test]
// async fn add_project_sample() {
//     let mut fixture = BackendFixture::new();
//     fixture.log_in().await;

//     let response_body = r#"
//     {
//         "collectionId": "pbc_484305853",
//         "collectionName": "projects",
//         "id": "project_id",
//         "name": "Project Name",
//         "userId": "user_id",
//         "project": "project.bin",
//         "samples": ["sample.wav", "added_sample.wav"],
//         "created": "2022-01-02 10:00:00.123Z",
//         "updated": "2022-05-04 10:00:00.123Z"
//     }
//     "#;

//     let mock = fixture.mock_server.mock(|when, then| {
//         when.method("PATCH")
//             .path("/api/collections/projects/records/project_id")
//             .body_contains("sample")
//             .body_contains("added_sample.wav");
//         then.status(200)
//             .header("Content-Type", "application/json")
//             .header("Accept", "application/json")
//             .body(response_body);
//     });

//     fixture
//         .backend
//         .add_project_sample("project_id", b"sample-bytes", "added_sample.wav")
//         .await
//         .unwrap();

//     mock.assert();
// }

// #[tokio::test]
// async fn remove_project_sample() {
//     let mut fixture = BackendFixture::new();
//     fixture.log_in().await;

//     let response_body = r#"
//     {
//         "collectionId": "pbc_484305853",
//         "collectionName": "projects",
//         "id": "project_id",
//         "name": "Project Name",
//         "userId": "user_id",
//         "project": "project.bin",
//         "samples": ["sample.wav"],
//         "created": "2022-01-02 10:00:00.123Z",
//         "updated": "2022-05-04 10:00:00.123Z"
//     }
//     "#;

//     let mock = fixture.mock_server.mock(|when, then| {
//         when.method("PATCH")
//             .path("/api/collections/projects/records/project_id")
//             .body_contains("samples-")
//             .body_contains("removed_sample.wav");
//         then.status(200)
//             .header("Content-Type", "application/json")
//             .header("Accept", "application/json")
//             .body(response_body);
//     });

//     fixture
//         .backend
//         .remove_project_sample("project_id", "removed_sample.wav")
//         .await
//         .unwrap();

//     mock.assert();
// }

// #[tokio::test]
// async fn remove_project() {
//     let mut fixture = BackendFixture::new();
//     fixture.log_in().await;

//     let mock = fixture.mock_server.mock(|when, then| {
//         when.method("DELETE")
//             .path("/api/collections/projects/records/project_id");
//         then.status(204);
//     });

//     let result = fixture.backend.remove_project("project_id").await;
//     assert!(result.is_ok());

//     mock.assert();
// }

// #[tokio::test]
// async fn get_project_file() {
//     let mut fixture = BackendFixture::new();
//     fixture.log_in().await;

//     let get_project_response = r#"
//     {
//         "collectionId": "pbc_484305853",
//         "collectionName": "projects",
//         "id": "test",
//         "name": "Test Project",
//         "userId": "user_id",
//         "project": "project_file.bin",
//         "samples": [],
//         "created": "2022-01-02 10:00:00.123Z",
//         "updated": "2022-03-04 10:00:00.123Z"
//     }
//     "#;

//     let file_bytes = b"project file bytes";

//     let get_project_mock = fixture.mock_server.mock(|when, then| {
//         when.method("GET").path("/api/collections/projects/records/project_id");
//         then.status(200)
//             .header("Content-Type", "application/json")
//             .header("Accept", "application/json")
//             .body(get_project_response);
//     });

//     let get_project_bin_mock = fixture.mock_server.mock(|when, then| {
//         when.method("GET")
//             .path("/api/files/projects/project_id/project_file.bin");
//         then.status(200)
//             .header("Content-Type", "application/octet-stream")
//             .body(file_bytes);
//     });

//     let bytes = fixture.backend.read_project_file("project_id").await.unwrap();
//     assert_eq!(bytes, file_bytes);

//     get_project_mock.assert();
//     get_project_bin_mock.assert();
// }
