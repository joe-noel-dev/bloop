use tokio::sync::{broadcast, mpsc};

use crate::{
    api::{Request, Response},
    model::{PlaybackState, Project},
};

pub struct State {
    pub response_tx: broadcast::Sender<Response>,
    pub request_tx: mpsc::Sender<Request>,
    pub project: Project,
    pub playback_state: PlaybackState,
}

impl State {
    pub fn new(response_tx: broadcast::Sender<Response>, request_tx: mpsc::Sender<Request>) -> Self {
        Self {
            response_tx,
            request_tx,
            project: Default::default(),
            playback_state: Default::default(),
        }
    }
}
