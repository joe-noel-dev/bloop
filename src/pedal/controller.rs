use futures_util::select;
use serialport::TTYPort;
use std::{
    io::Write,
    sync::mpsc,
    thread::{self, JoinHandle},
};

use crate::model::{Action, PlaybackState, PlayingState, Progress};

pub struct PedalController {
    last_beat: Option<i64>,
    #[allow(dead_code)]
    thread: JoinHandle<()>,
    beat_channel_tx: mpsc::Sender<i64>,
}

impl PedalController {
    pub fn new() -> Self {
        let (beat_channel_tx, beat_channel_rx) = mpsc::channel();

        let thread = thread::spawn(move || {
            let builder = serialport::new("/dev/cu.usbmodem11401", 9600);
            let mut port = match TTYPort::open(&builder) {
                Ok(port) => port,
                Err(error) => {
                    eprintln!("Error opening serial port: {error}");
                    return;
                }
            };

            while let Ok(beat) = beat_channel_rx.recv() {
                if let Err(error) = write!(port, "beat:{beat};") {
                    eprintln!("Error writing to serial port: {error}");
                }
            }
        });

        Self {
            last_beat: None,
            thread,
            beat_channel_tx,
        }
    }

    pub fn set_state(&mut self, playback_state: &PlaybackState, progress: &Progress) {
        if playback_state.playing == PlayingState::Stopped {
            return;
        }

        let beat = progress.section_beat.floor() as i64;

        if self.last_beat != Some(beat) {
            self.last_beat = Some(beat);
            let _ = self.beat_channel_tx.send(beat);
        }
    }
}
