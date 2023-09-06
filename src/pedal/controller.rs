use std::collections::HashMap;

use crate::model::{Action, Notification, PlayingState};
use tokio::{io::AsyncReadExt, io::AsyncWriteExt, sync::mpsc};
use tokio_serial::{SerialPortBuilderExt, SerialStream};

pub struct PedalController {
    last_beat: Option<i64>,
    notification_rx: mpsc::Receiver<Notification>,
    action_tx: mpsc::Sender<Action>,
    incoming_message: String,
    port: Option<SerialStream>,
    actions: HashMap<i32, Action>,
}

impl PedalController {
    fn open_serial() -> Option<SerialStream> {
        let serial_address = "/dev/cu.usbmodem21401";
        let builder = tokio_serial::new(serial_address, 9600);

        let mut port = match builder.open_native_async() {
            Ok(port) => port,
            Err(error) => {
                eprintln!("Error opening serial port: {error}");
                return None;
            }
        };

        if let Err(error) = port.set_exclusive(false) {
            eprintln!("Error setting port non-exclusive: {error}");
        }

        println!("Connected to serial at address: {serial_address}");

        Some(port)
    }

    pub fn new(action_tx: mpsc::Sender<Action>, notification_rx: mpsc::Receiver<Notification>) -> Self {
        Self {
            last_beat: None,
            notification_rx,
            action_tx,
            port: Self::open_serial(),
            incoming_message: String::default(),
            actions: HashMap::from([(0, Action::NextSong), (1, Action::ToggleLoop), (2, Action::TogglePlay)]),
        }
    }

    pub async fn run(&mut self) {
        let port = match &mut self.port {
            Some(port) => port,
            None => {
                let _ = self.notification_rx.recv().await;
                return;
            }
        };

        tokio::select! {
            Ok(byte) = port.read_u8() => self.on_byte_received(byte).await,

            Some(notification) = self.notification_rx.recv() => {
                if notification.playback_state.playing == PlayingState::Stopped {
                    return;
                }

                let beat = notification.progress.section_beat.floor() as i64;

                if self.last_beat == Some(beat) {
                    return;
                }

                let message = format!("beat:{beat};");

                if let Err(error) = port.write(message.as_bytes()).await {
                    eprintln!("Error writing to serial: {error}");
                }
            }
        }
    }

    async fn on_byte_received(&mut self, byte: u8) {
        if byte == b';' {
            self.on_message_received(self.incoming_message.as_str()).await;
            self.incoming_message.clear();
        } else {
            self.incoming_message.push(byte as char);
        }
    }

    async fn on_message_received(&self, message: &str) {
        let mut split = message.split(':');

        let value = match split.next() {
            Some("press") => split.next(),
            _ => return,
        };

        let value = match value {
            Some(value) => value,
            None => return,
        };

        let index = match value.parse::<i32>() {
            Ok(index) => index,
            _ => return,
        };

        let action = self.actions.get(&index);

        if let Some(action) = action {
            let _ = self.action_tx.send(*action).await;
        }
    }
}
