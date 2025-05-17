mod common;

use chrono::DateTime;
use common::BackendFixture;

#[tokio::test]
async fn list_projects_successful() {
    let mut fixture = BackendFixture::new();

    fixture.log_in().await;

    let get_projects_response = r#"  
    {
        "page": 1,
        "perPage": 30,
        "totalPages": 1,
        "totalItems": 2,
        "items": [
            {
            "collectionId": "pbc_484305853",
            "collectionName": "projects",
            "id": "test",
            "name": "test",
            "userId": "RELATION_RECORD_ID",
            "project": "filename.jpg",
            "samples": [
                "filename.jpg"
            ],
            "created": "2022-01-01 10:00:00.123Z",
            "updated": "2022-01-01 10:00:00.123Z"
            },
            {
            "collectionId": "pbc_484305853",
            "collectionName": "projects",
            "id": "[object Object]2",
            "name": "test",
            "userId": "RELATION_RECORD_ID",
            "project": "filename.jpg",
            "samples": [
                "filename.jpg"
            ],
            "created": "2022-01-01 10:00:00.123Z",
            "updated": "2022-01-01 10:00:00.123Z"
            }
        ]
    }"#;

    let mock = fixture.mock_server.mock(|when, then| {
        when.method("GET").path("/api/collections/projects/records");
        then.status(200)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .body(get_projects_response);
    });

    let projects = fixture.backend.get_projects().await.unwrap();

    assert_eq!(projects.len(), 2);

    assert_eq!(projects[0].id, "test");
    assert_eq!(projects[0].name, "test");
    assert_eq!(projects[0].user_id, "RELATION_RECORD_ID");
    assert_eq!(projects[0].project, "filename.jpg");
    assert_eq!(projects[0].samples, vec!["filename.jpg"]);
    assert_eq!(
        projects[0].created,
        DateTime::parse_from_rfc3339("2022-01-01 10:00:00.123Z").unwrap()
    );
    assert_eq!(
        projects[0].updated,
        DateTime::parse_from_rfc3339("2022-01-01 10:00:00.123Z").unwrap()
    );

    assert_eq!(projects[1].id, "[object Object]2");
    assert_eq!(projects[1].name, "test");
    assert_eq!(projects[1].user_id, "RELATION_RECORD_ID");
    assert_eq!(projects[1].project, "filename.jpg");
    assert_eq!(projects[1].samples, vec!["filename.jpg"]);
    assert_eq!(
        projects[1].created,
        DateTime::parse_from_rfc3339("2022-01-01 10:00:00.123Z").unwrap()
    );
    assert_eq!(
        projects[1].updated,
        DateTime::parse_from_rfc3339("2022-01-01 10:00:00.123Z").unwrap()
    );

    mock.assert();
}

#[tokio::test]
async fn get_project_successful() {
    let mut fixture = BackendFixture::new();
    fixture.log_in().await;

    let get_project_response = r#"
    {
        "collectionId": "pbc_484305853",
        "collectionName": "projects",
        "id": "test",
        "name": "Test Project",
        "userId": "user_id",
        "project": "project.bin",
        "samples": [
            "sample.wav"
        ],
        "created": "2022-01-02 10:00:00.123Z",
        "updated": "2022-03-04 10:00:00.123Z"
    }
    "#;

    let mock = fixture.mock_server.mock(|when, then| {
        when.method("GET").path("/api/collections/projects/records/test");
        then.status(200)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .body(get_project_response);
    });

    let project = fixture.backend.get_project("test").await.unwrap();

    assert_eq!(project.id, "test");
    assert_eq!(project.name, "Test Project");
    assert_eq!(project.user_id, "user_id");
    assert_eq!(project.project, "project.bin");
    assert_eq!(project.samples, vec!["sample.wav"]);
    assert_eq!(
        project.created,
        DateTime::parse_from_rfc3339("2022-01-02 10:00:00.123Z").unwrap()
    );
    assert_eq!(
        project.updated,
        DateTime::parse_from_rfc3339("2022-03-04 10:00:00.123Z").unwrap()
    );

    mock.assert();
}

#[tokio::test]
async fn create_project() {
    let mut fixture = BackendFixture::new();
    fixture.log_in().await;

    let create_project_response = r#"
    {
        "collectionId": "pbc_484305853",
        "collectionName": "projects",
        "id": "test",
        "name": "Project Name",
        "userId": "user_id",
        "project": "project.bin",
        "samples": [
            "sample.wav"
        ],
        "created": "2022-01-02 10:00:00.123Z",
        "updated": "2022-03-04 10:00:00.123Z"
    } 
    "#;

    let mock = fixture.mock_server.mock(|when, then| {
        when.method("POST").path("/api/collections/projects/records");
        then.status(200)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .body(create_project_response);
    });

    let created_project = fixture.backend.create_project("user_id").await.unwrap();

    assert_eq!(created_project.id, "test");
    assert_eq!(created_project.name, "Project Name");
    assert_eq!(created_project.user_id, "user_id");
    assert_eq!(created_project.project, "project.bin");
    assert_eq!(created_project.samples, vec!["sample.wav"]);
    assert_eq!(
        created_project.created,
        DateTime::parse_from_rfc3339("2022-01-02 10:00:00.123Z").unwrap()
    );
    assert_eq!(
        created_project.updated,
        DateTime::parse_from_rfc3339("2022-03-04 10:00:00.123Z").unwrap()
    );

    mock.assert();
}

#[tokio::test]
async fn update_project_name() {
    let mut fixture = BackendFixture::new();
    fixture.log_in().await;

    let response_body = r#"
    {
        "collectionId": "pbc_484305853",
        "collectionName": "projects",
        "id": "project_id",
        "name": "Updated Project Name",
        "userId": "user_id",
        "project": "project.bin",
        "samples": ["sample.wav"],
        "created": "2022-01-02 10:00:00.123Z",
        "updated": "2022-05-04 10:00:00.123Z"
    }
    "#;

    let mock = fixture.mock_server.mock(|when, then| {
        when.method("PATCH")
            .path("/api/collections/projects/records/project_id")
            .body_contains("name")
            .body_contains("Updated Project Name");
        then.status(200)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .body(response_body);
    });

    let updated_project = fixture
        .backend
        .update_project_name("project_id", "Updated Project Name")
        .await
        .unwrap();

    assert_eq!(updated_project.name, "Updated Project Name");

    mock.assert();
}

#[tokio::test]
async fn update_project_file() {
    let mut fixture = BackendFixture::new();
    fixture.log_in().await;

    let response_body = r#"
    {
        "collectionId": "pbc_484305853",
        "collectionName": "projects",
        "id": "project_id",
        "name": "Project Name",
        "userId": "user_id",
        "project": "updated_project.bin",
        "samples": ["sample.wav"],
        "created": "2022-01-02 10:00:00.123Z",
        "updated": "2022-05-04 10:00:00.123Z"
    }
    "#;

    let mock = fixture.mock_server.mock(|when, then| {
        when.method("PATCH")
            .path("/api/collections/projects/records/project_id")
            .body_contains("project")
            .body_contains("project.bin");
        then.status(200)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .body(response_body);
    });

    let updated_project = fixture
        .backend
        .update_project_file("project_id", b"updated project bytes")
        .await
        .unwrap();

    assert_eq!(updated_project.project, "updated_project.bin");
    mock.assert();
}

#[tokio::test]
async fn add_project_sample() {
    let mut fixture = BackendFixture::new();
    fixture.log_in().await;

    let response_body = r#"
    {
        "collectionId": "pbc_484305853",
        "collectionName": "projects",
        "id": "project_id",
        "name": "Project Name",
        "userId": "user_id",
        "project": "project.bin",
        "samples": ["sample.wav", "added_sample.wav"],
        "created": "2022-01-02 10:00:00.123Z",
        "updated": "2022-05-04 10:00:00.123Z"
    }
    "#;

    let mock = fixture.mock_server.mock(|when, then| {
        when.method("PATCH")
            .path("/api/collections/projects/records/project_id")
            .body_contains("sample")
            .body_contains("added_sample.wav");
        then.status(200)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .body(response_body);
    });

    let updated_project = fixture
        .backend
        .add_project_sample("project_id", b"sample-bytes", "added_sample.wav")
        .await
        .unwrap();

    assert!(updated_project.samples.contains(&"added_sample.wav".to_string()));
    mock.assert();
}

#[tokio::test]
async fn remove_project_sample() {
    let mut fixture = BackendFixture::new();
    fixture.log_in().await;

    let response_body = r#"
    {
        "collectionId": "pbc_484305853",
        "collectionName": "projects",
        "id": "project_id",
        "name": "Project Name",
        "userId": "user_id",
        "project": "project.bin",
        "samples": ["sample.wav"],
        "created": "2022-01-02 10:00:00.123Z",
        "updated": "2022-05-04 10:00:00.123Z"
    }
    "#;

    let mock = fixture.mock_server.mock(|when, then| {
        when.method("PATCH")
            .path("/api/collections/projects/records/project_id")
            .body_contains("samples-")
            .body_contains("removed_sample.wav");
        then.status(200)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .body(response_body);
    });

    let updated_project = fixture
        .backend
        .remove_project_sample("project_id", "removed_sample.wav")
        .await
        .unwrap();

    assert!(!updated_project.samples.contains(&"removed_sample.wav".to_string()));
    mock.assert();
}
