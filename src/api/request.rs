use crate::model::id;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Entity {
  All,
  Channel,
  Section,
  Song,
  Project,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetRequest {
  pub entity: Entity,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<id::ID>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddRequest {
  pub entity: Entity,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub parent_id: Option<id::ID>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SelectRequest {
  pub entity: Entity,
  pub id: id::ID,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RemoveRequest {
  pub entity: Entity,
  pub id: id::ID,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "method", content = "payload")]
pub enum Request {
  Get(GetRequest),
  Add(AddRequest),
  Select(SelectRequest),
  Remove(RemoveRequest),
}
