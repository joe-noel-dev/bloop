use crate::api::Response;

#[derive(Debug, Clone)]
pub enum Message {
    ApiResponse(Box<Response>),
    StartPlayback,
    StopPlayback,
    SelectPreviousSong,
    SelectNextSong,
}
