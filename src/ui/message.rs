use crate::{bloop::Response, model::ID};

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
    SelectSection(ID),
}
