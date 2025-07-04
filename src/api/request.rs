use crate::{bloop::*, model::ID};

impl Request {
    pub fn get_request(entity: Entity, id: ID) -> Self {
        Self {
            get: Some(GetRequest {
                entity: entity.into(),
                id,
                ..Default::default()
            })
            .into(),
            ..Default::default()
        }
    }

    pub fn save_project_request() -> Self {
        Self {
            save: Some(SaveProjectRequest { ..Default::default() }).into(),
            ..Default::default()
        }
    }

    pub fn select_request(entity: Entity, id: ID) -> Self {
        Self {
            select: Some(SelectRequest {
                entity: entity.into(),
                id,
                ..Default::default()
            })
            .into(),
            ..Default::default()
        }
    }

    pub fn transport_request(method: TransportMethod) -> Request {
        Self {
            transport: Some(TransportRequest {
                method: method.into(),
                ..Default::default()
            })
            .into(),
            ..Default::default()
        }
    }
}
