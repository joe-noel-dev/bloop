use crate::model::{id, sample, section, song};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Entity {
    All,
    Channel,
    Project,
    Projects,
    Sample,
    Section,
    Song,
    Waveform,
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
    pub id: Option<id::ID>,
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
#[serde(tag = "entity", content = "value")]
pub enum UpdateRequest {
    Song(song::Song),
    Section(section::Section),
    Sample(sample::Sample),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoadRequest {
    pub id: id::ID,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RenameRequest {
    pub entity: Entity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<id::ID>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "method", content = "payload")]
pub enum Request {
    Get(GetRequest),
    Add(AddRequest),
    Select(SelectRequest),
    Remove(RemoveRequest),
    Update(UpdateRequest),
    Save,
    Load(LoadRequest),
    Rename(RenameRequest),
}
