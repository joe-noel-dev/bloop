use std::{collections::HashMap, fs::File, io::BufReader, path::Path};

use crate::model::{Action, Notification, PlayingState};
use serde_derive::{Deserialize, Serialize};
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

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Preferences {
    serial_path: Option<String>,
}

impl PedalController {
    pub fn new(
        action_tx: mpsc::Sender<Action>,
        notification_rx: mpsc::Receiver<Notification>,
        preferences_dir: &Path,
    ) -> Self {
        let preferences = read_preferences(preferences_dir).unwrap_or_default();

        Self {
            last_beat: None,
            notification_rx,
            action_tx,
            port: open_serial(&preferences.serial_path.unwrap_or("/dev/cu.usbmodem21401".to_string())),
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

fn open_serial(serial_path: &str) -> Option<SerialStream> {
    let builder = tokio_serial::new(serial_path, 9600);

    let mut port = match builder.open_native_async() {
        Ok(port) => port,
        Err(error) => {
            eprintln!("Error opening serial port ({serial_path}): {error}");
            return None;
        }
    };

    if let Err(error) = port.set_exclusive(false) {
        eprintln!("Error setting port non-exclusive: {error}");
    }

    println!("Connected to serial at: {serial_path}");

    Some(port)
}

fn read_preferences(preferences_dir: &Path) -> anyhow::Result<Preferences> {
    let mut preferences_path = preferences_dir.to_path_buf();
    preferences_path.push("pedal.json");

    let file = File::open(preferences_path)?;
    let reader = BufReader::new(file);
    let preferences = serde_json::from_reader(reader)?;

    Ok(preferences)
}
