mod api;
mod generators;
mod model;

extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::server::accept;

fn send_error(
    websocket: &mut tungstenite::WebSocket<std::net::TcpStream>,
    error: &str,
) {
    let mut response = api::response::Response::new();
    response.error = Option::Some(error.to_string());
    websocket
        .write_message(tungstenite::Message::Text(
            serde_json::to_string(&response).unwrap(),
        ))
        .unwrap();
}

fn main() {
    let server =
        TcpListener::bind("127.0.0.1:8999").expect("Couldn't bind to port");

    println!(
        "Server listening on address {}",
        server.local_addr().unwrap()
    );

    for stream in server.incoming() {
        spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                let message = match websocket.read_message() {
                    Ok(message) => message,
                    Err(error) => {
                        println!("Closing connection: {}", error);
                        break;
                    }
                };

                if message.is_text() {
                    let request: api::request::Request =
                        match serde_json::from_str(message.to_text().unwrap()) {
                            Ok(request) => request,
                            Err(error) => {
                                let description =
                                    format!("Couldn't parse JSON: {}", error);
                                send_error(&mut websocket, &description);
                                println!("Error: {}", error);
                                continue;
                            }
                        };

                    dbg!(request);
                }
            }
        });
    }
    let project = generators::projects::generate_project(4, 3, 4);
    let serialised = serde_json::to_string_pretty(&project).unwrap();
    println!("{}", &serialised);
}
