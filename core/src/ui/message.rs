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
    /// Periodic tick to force UI redraw on platforms that need it (e.g., Raspberry Pi)
    Tick,
}
