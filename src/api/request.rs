use crate::{
    model::{Sample, Section, Song, ID},
    types::AudioFileFormat,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Entity {
    All,
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
    pub id: Option<ID>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddRequest {
    pub entity: Entity,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SelectRequest {
    pub entity: Entity,
    pub id: ID,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RemoveRequest {
    pub entity: Entity,
    pub id: ID,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "entity", content = "value")]
pub enum UpdateRequest {
    Song(Song),
    Section(Section),
    Sample(Sample),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoadRequest {
    pub id: ID,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RenameRequest {
    pub entity: Entity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RemoveSampleRequest {
    pub song_id: ID,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QueueRequest {
    pub song_id: ID,
    pub section_id: ID,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "method", content = "options")]
pub enum TransportMethod {
    Play,
    Stop,
    Loop,
    ExitLoop,
    Queue(QueueRequest),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BeginUploadRequest {
    pub upload_id: ID,
    pub filename: String,
    pub format: AudioFileFormat,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UploadRequest {
    pub upload_id: ID,
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CompleteUploadRequest {
    pub upload_id: ID,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddSampleRequest {
    pub song_id: ID,
    pub upload_id: ID,
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
    BeginUpload(BeginUploadRequest),
    Upload(UploadRequest),
    CompleteUpload(CompleteUploadRequest),
    AddSample(AddSampleRequest),
    RemoveSample(RemoveSampleRequest),
    Transport(TransportMethod),
}
