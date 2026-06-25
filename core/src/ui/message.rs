use crate::{bloop::Response, model::ID};

#[derive(Debug, Clone)]
pub enum Message {
    ApiResponse(Box<Response>),
    StartPlayback,
    StopPlayback,
    TogglePlayback,
    EnterLoop,
    ExitLoop,
    SelectPreviousSong,
    SelectNextSong,
    SelectPreviousSection,
    SelectNextSection,
    #[allow(unused)]
    SelectSection(ID),
}
