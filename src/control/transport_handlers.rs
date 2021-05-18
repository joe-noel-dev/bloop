use crate::{
    api::request::{Request, TransportMethod},
    audio::manager::Audio,
};

pub fn handle_request(request: &Request, audio: &dyn Audio) {
    if let Request::Transport(transport_method) = request {
        handle_transport_request(transport_method, audio)
    }
}

fn handle_transport_request(transport_method: &TransportMethod, audio: &dyn Audio) {
    match transport_method {
        TransportMethod::Play => audio.play(),
        TransportMethod::Stop => audio.stop(),
        TransportMethod::Loop => audio.enter_loop(),
        TransportMethod::ExitLoop => audio.exit_loop(),
        TransportMethod::Queue(queue_request) => audio.queue(&queue_request.song_id, &queue_request.section_id),
    }
}
