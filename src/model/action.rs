#[derive(Copy, Clone)]
pub enum Action {
    PreviousSong,
    NextSong,
    PreviousSection,
    NextSection,
    QueueSelected,
    ToggleLoop,
    TogglePlay,
}
