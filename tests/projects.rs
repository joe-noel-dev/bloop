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
async fn update_project() {
    let mut fixture = BackendFixture::new();
    fixture.log_in().await;

    let update_project_response = r#"
    {
        "collectionId": "pbc_484305853",
        "collectionName": "projects",
        "id": "project_id",
        "name": "Updated Project Name",
        "userId": "user_id",
        "project": "updated_project.bin",
        "samples": [
            "updated_sample1.wav",
            "updated_sample2.wav"
        ],
        "created": "2022-01-02 10:00:00.123Z",
        "updated": "2022-05-04 10:00:00.123Z"
    }
    "#;

    let mock = fixture.mock_server.mock(|when, then| {
        when.method("PATCH")
            .path("/api/collections/projects/records/project_id");
        then.status(200)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .body(update_project_response);
    });

    let name = Some("Updated Project Name");
    let project_bytes = Some(b"updated project bytes".as_ref());
    let samples_vec = vec![b"sample1".to_vec(), b"sample2".to_vec()];
    let samples: Option<&[Vec<u8>]> = Some(&samples_vec);

    let updated_project = fixture
        .backend
        .update_project("project_id", name, project_bytes, samples)
        .await
        .unwrap();

    assert_eq!(updated_project.id, "project_id");
    assert_eq!(updated_project.name, "Updated Project Name");
    assert_eq!(updated_project.user_id, "user_id");
    assert_eq!(updated_project.project, "updated_project.bin");
    assert_eq!(
        updated_project.samples,
        vec!["updated_sample1.wav", "updated_sample2.wav"]
    );
    assert_eq!(
        updated_project.created,
        DateTime::parse_from_rfc3339("2022-01-02 10:00:00.123Z").unwrap()
    );
    assert_eq!(
        updated_project.updated,
        DateTime::parse_from_rfc3339("2022-05-04 10:00:00.123Z").unwrap()
    );

    mock.assert();
}
