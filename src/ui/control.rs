use futures::stream::unfold;
use iced::Subscription;
use tokio::sync::mpsc;

use crate::api::{Entity, Request, Response, SelectRequest, TransportMethod};

use super::{message::Message, state::State};

pub fn update(state: &mut State, message: Message) {
    match message {
        Message::ApiResponse(response) => handle_api_response(state, *response),
        Message::StartPlayback => {
            let request = Request::Transport(TransportMethod::Play);
            send_request(state.request_tx.clone(), request);
        }
        Message::StopPlayback => {
            let request = Request::Transport(TransportMethod::Stop);
            send_request(state.request_tx.clone(), request);
        }
        Message::SelectPreviousSong => select_song_with_offset(state, -1),
        Message::SelectNextSong => select_song_with_offset(state, 1),
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

    let request = Request::Select(SelectRequest {
        entity: Entity::Song,
        id: song.id,
    });

    send_request(state.request_tx.clone(), request);
}

pub fn subscription(state: &State) -> Subscription<Message> {
    Subscription::run_with_id(
        "api_response_subscription",
        unfold(
            state.response_tx.subscribe(),
            async move |mut response_rx| match response_rx.recv().await {
                Ok(response) => Some((Message::ApiResponse(Box::new(response)), response_rx)),
                Err(_) => None,
            },
        ),
    )
}

fn handle_api_response(state: &mut State, response: Response) {
    if let Some(project) = response.project {
        state.project = project;
    }

    if let Some(playback) = response.playback_state {
        state.playback_state = playback;
    }
}

fn send_request(request_tx: mpsc::Sender<Request>, request: Request) {
    tokio::spawn(async move {
        let _ = request_tx.send(request).await;
    });
}
