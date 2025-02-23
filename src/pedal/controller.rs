use std::{
    collections::HashMap,
    time::{self, Duration, Instant},
};

use crate::{
    model::{Action, Notification, PlayingState},
    preferences::PedalPreferences,
};

use log::{debug, error, info};
use tokio::{io::AsyncReadExt, io::AsyncWriteExt, sync::mpsc};
use tokio_serial::{SerialPortBuilderExt, SerialStream};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Gesture {
    Press,
    Release,
    Hold,
}

struct Mapping {
    index: i32,
    gesture: Gesture,
    action: Action,
}

const HOLD_DURATION: Duration = Duration::from_millis(300);

pub struct PedalController {
    last_beat: Option<i64>,
    notification_rx: mpsc::Receiver<Notification>,
    action_tx: mpsc::Sender<Action>,
    incoming_message: String,
    port: Option<SerialStream>,
    mappings: Vec<Mapping>,
    press_times: HashMap<i32, time::Instant>,
}

impl PedalController {
    pub fn new(
        action_tx: mpsc::Sender<Action>,
        notification_rx: mpsc::Receiver<Notification>,
        preferences: PedalPreferences,
    ) -> Self {
        let path = preferences.serial_path.as_deref().unwrap_or("/dev/cu.usbmodem21401");
        let port = open_serial(path);

        Self {
            last_beat: None,
            notification_rx,
            action_tx,
            port,
            incoming_message: String::default(),
            mappings: vec![
                Mapping {
                    index: 0,
                    gesture: Gesture::Press,
                    action: Action::ToggleLoop,
                },
                Mapping {
                    index: 1,
                    gesture: Gesture::Release,
                    action: Action::NextSong,
                },
                Mapping {
                    index: 1,
                    gesture: Gesture::Hold,
                    action: Action::PreviousSong,
                },
                Mapping {
                    index: 2,
                    gesture: Gesture::Press,
                    action: Action::TogglePlay,
                },
            ],
            press_times: HashMap::new(),
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

        let mut tick_interval = tokio::time::interval(Duration::from_millis(50));

        tokio::select! {
            Ok(byte) = port.read_u8() => self.on_byte_received(byte).await,

            _ = tick_interval.tick() => self.on_tick().await,

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
                    error!("Error writing to serial: {error}");
                }
            }
        }
    }

    async fn on_byte_received(&mut self, byte: u8) {
        if byte == b';' {
            let message = self.incoming_message.clone();
            self.on_message_received(&message).await;
            self.incoming_message.clear();
        } else {
            self.incoming_message.push(byte as char);
        }
    }

    async fn on_press(&mut self, value: &str) {
        let index = match value.parse::<i32>() {
            Ok(index) => index,
            _ => return,
        };

        self.press_times.insert(index, Instant::now());

        let mapping = match self
            .mappings
            .iter()
            .find(|mapping| mapping.index == index && mapping.gesture == Gesture::Press)
        {
            Some(mapping) => mapping,
            None => return,
        };

        let _ = self.action_tx.send(mapping.action).await;
    }

    async fn on_release(&mut self, value: &str) {
        let index = match value.parse::<i32>() {
            Ok(index) => index,
            _ => return,
        };

        let duration = match self.press_times.remove(&index) {
            Some(instant) => instant.elapsed(),
            _ => return,
        };

        if duration <= HOLD_DURATION {
            let mapping = match self
                .mappings
                .iter()
                .find(|mapping| mapping.index == index && mapping.gesture == Gesture::Release)
            {
                Some(mapping) => mapping,
                None => return,
            };

            let _ = self.action_tx.send(mapping.action).await;
        }
    }

    async fn on_tick(&mut self) {
        for (index, duration) in self.press_times.iter() {
            if duration.elapsed() > HOLD_DURATION {
                let mapping = match self
                    .mappings
                    .iter()
                    .find(|&mapping| mapping.index == *index && mapping.gesture == Gesture::Hold)
                {
                    Some(mapping) => mapping,
                    None => continue,
                };

                let _ = self.action_tx.send(mapping.action).await;
            }
        }

        self.press_times
            .retain(|_, duration| duration.elapsed() <= HOLD_DURATION);
    }

    async fn on_message_received(&mut self, message: &str) {
        debug!("Received from pedal: {message}");

        let mut split = message.split(':');

        let key = split.next();
        let value = split.next();

        match (key, value) {
            (Some("press"), Some(value)) => self.on_press(value).await,
            (Some("release"), Some(value)) => self.on_release(value).await,
            _ => (),
        }
    }
}

fn open_serial(serial_path: &str) -> Option<SerialStream> {
    let builder = tokio_serial::new(serial_path, 9600);

    let mut port = match builder.open_native_async() {
        Ok(port) => port,
        Err(error) => {
            error!("Error opening serial port ({serial_path}): {error}");
            return None;
        }
    };

    if let Err(error) = port.set_exclusive(false) {
        error!("Error setting port non-exclusive: {error}");
    }

    info!("Connected to serial at: {serial_path}");

    Some(port)
}
