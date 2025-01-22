use uuid::Uuid;

use crate::api::Response;

#[derive(Debug, Clone)]
pub enum Message {
    ApiResponse(Box<Response>),
    StartPlayback,
    StopPlayback,
    EnterLoop,
    ExitLoop,
    SelectPreviousSong,
    SelectNextSong,
    #[allow(unused)]
    SelectSection(Uuid),
}
