use futures::stream::unfold;
use iced::Subscription;
use std::hash::{Hash, Hasher};
use std::time::Duration;
use tokio::sync::{broadcast, mpsc};

use crate::bloop::{Entity, Request, Response, TransportMethod};

use super::{message::Message, state::State};

pub fn update(state: &mut State, message: Message) {
    match message {
        Message::ApiResponse(response) => handle_api_response(state, *response),
        Message::Tick => {
            // No-op: just triggers a redraw
        }
        Message::StartPlayback => {
            let request = Request::transport_request(TransportMethod::PLAY);
            send_request(state.request_tx.clone(), request);
        }
        Message::StopPlayback => {
            let request = Request::transport_request(TransportMethod::STOP);
            send_request(state.request_tx.clone(), request);
        }
        Message::SelectPreviousSong => select_song_with_offset(state, -1),
        Message::SelectNextSong => select_song_with_offset(state, 1),
        Message::SelectSection(id) => {
            let request = Request::select_request(Entity::SECTION, id);
            send_request(state.request_tx.clone(), request);
        }
        Message::EnterLoop => {
            let request = Request::transport_request(TransportMethod::LOOP);
            send_request(state.request_tx.clone(), request);
        }
        Message::ExitLoop => {
            let request = Request::transport_request(TransportMethod::EXIT_LOOP);
            send_request(state.request_tx.clone(), request);
        }
    }
}

fn select_song_with_offset(state: &State, offset: i64) {
    let current_song_index = match state.project.selected_song_index() {
        Some(index) => index,
        None => return,
    };

    let next_song_index = current_song_index as i64 + offset;
    if next_song_index < 0 || next_song_index >= state.project.songs.len() as i64 {
        return;
    }

    let song = match state.project.song_with_index(next_song_index as usize) {
        Some(song) => song,
        None => return,
    };

    let request = Request::select_request(Entity::SONG, song.id);
    send_request(state.request_tx.clone(), request);
}

struct ResponseSender(broadcast::Sender<Response>);

impl Hash for ResponseSender {
    fn hash<H: Hasher>(&self, state: &mut H) {
        "api_response_subscription".hash(state);
    }
}

pub fn subscription(state: &State) -> Subscription<Message> {
    let api_subscription = Subscription::run_with(ResponseSender(state.response_tx.clone()), |sender| {
        unfold(sender.0.subscribe(), async move |mut response_rx| {
            match response_rx.recv().await {
                Ok(response) => Some((Message::ApiResponse(Box::new(response)), response_rx)),
                Err(_) => None,
            }
        })
    });

    // On Linux (Raspberry Pi), add a periodic heartbeat to force redraws
    // This works around a rendering issue where the UI doesn't always update
    #[cfg(target_os = "linux")]
    {
        let heartbeat = iced::time::every(Duration::from_millis(500)).map(|_| Message::Tick);
        Subscription::batch([api_subscription, heartbeat])
    }

    #[cfg(not(target_os = "linux"))]
    {
        api_subscription
    }
}

fn handle_api_response(state: &mut State, response: Response) {
    if let Some(project) = response.project.as_ref() {
        state.project = project.clone();
    }

    if let Some(playback) = response.playback_state.as_ref() {
        state.playback_state = playback.clone();
    }

    if let Some(progress) = response.progress.as_ref() {
        state.progress = progress.clone();
    }
}

fn send_request(request_tx: mpsc::Sender<Request>, request: Request) {
    tokio::spawn(async move {
        let _ = request_tx.send(request).await;
    });
}
