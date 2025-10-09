use wiremock::{matchers, Mock, MockServer, ResponseTemplate};

pub struct Mocketbase {
    server: MockServer,
}

const ACCEPTED_TOKEN: &str = "test-token";

impl Mocketbase {
    pub async fn new() -> Self {
        let server = MockServer::start().await;
        Self { server }
    }

    pub fn uri(&self) -> String {
        self.server.uri()
    }

    pub async fn add_user(&mut self, user: MockUser) {
        Mock::given(matchers::method("POST"))
            .and(matchers::path("/api/collections/users/auth-with-password"))
            .and(matchers::body_json(serde_json::json!({
                    "identity": user.email,
                    "password": user.password
                }
            )))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "token": ACCEPTED_TOKEN.to_string(),
                    "record": user.to_json()
                }
            )))
            .mount(&self.server)
            .await;
    }

    pub async fn set_projects(&mut self, projects: Vec<MockProject>) {
        Mock::given(matchers::method("GET"))
            .and(matchers::path("/api/collections/projects/records"))
            .and(matchers::bearer_token(ACCEPTED_TOKEN))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "page": 1,
                "perPage": 30,
                "totalPages": 1,
                "totalItems": projects.len(),
                "items": projects.iter().map(|p| p.to_json()).collect::<Vec<_>>()
            })))
            .mount(&self.server)
            .await;
    }
}

#[derive(Clone, Default)]
pub struct MockUser {
    pub id: String,
    pub email: String,
    pub password: String,
    pub name: String,
    pub avatar: String,
    pub created: String,
    pub updated: String,
}

impl MockUser {
    pub fn new(id: &str, email: &str, password: &str, name: &str) -> Self {
        Self {
            id: id.to_string(),
            email: email.to_string(),
            password: password.to_string(),
            name: name.to_string(),
            avatar: String::new(),
            created: "2023-01-01T00:00:00.000Z".to_string(),
            updated: "2023-01-01T00:00:00.000Z".to_string(),
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "collectionId": "_pb_users_auth_",
            "collectionName": "users",
            "id": self.id,
            "email": self.email,
            "emailVisibility": true,
            "verified": true,
            "name": self.name,
            "avatar": self.avatar,
            "created": self.created,
            "updated": self.updated
        })
    }
}

pub struct MockProject {
    pub id: String,
    pub name: String,
    pub user_id: String,
    pub project_file: Vec<u8>,
    pub samples: Vec<String>,
    pub created: String,
    pub updated: String,
}

impl MockProject {
    pub fn new(id: &str, name: &str, user_id: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            user_id: user_id.to_string(),
            project_file: Vec::new(),
            samples: Vec::new(),
            created: "2022-01-01T10:00:00.123Z".to_string(),
            updated: "2022-01-01T10:00:00.123Z".to_string(),
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "collectionId": "pbc_484305853",
            "collectionName": "projects",
            "id": self.id,
            "name": self.name,
            "userId": self.user_id,
            "project": self.project_file,
            "samples": self.samples,
            "created": self.created,
            "updated": self.updated
        })
    }
}
