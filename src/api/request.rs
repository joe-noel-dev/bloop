use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Method {
  Get,
  Add,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Entity {
  #[serde(rename = "*")]
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
  pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddRequest {
  pub entity: Entity,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub parent_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "method", content = "payload")]
pub enum Request {
  Get(GetRequest),
  Add(AddRequest),
}
