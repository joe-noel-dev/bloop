use super::matcher::Matcher;
use crate::midi::matcher::ExactMatcher;
use crate::model::Action;
use log::{error, info};
use midir::{MidiInput, MidiInputConnection};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader, path::Path};
use tokio::sync::mpsc;

#[derive(Default)]
#[allow(dead_code)]
pub struct MidiController {
    input_connection: Option<MidiInputConnection<Context>>,
}

struct Mapping {
    pub matcher: Box<dyn Matcher + Send>,
    pub action: Action,
}

struct Context {
    action_tx: mpsc::Sender<Action>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Preferences {
    input_device: Option<String>,
}

fn get_mappings() -> Vec<Mapping> {
    vec![
        Mapping {
            matcher: Box::new(ExactMatcher::new(&[176_u8, 40_u8, 127_u8])),
            action: Action::PreviousSong,
        },
        Mapping {
            matcher: Box::new(ExactMatcher::new(&[176_u8, 41_u8, 127_u8])),
            action: Action::NextSong,
        },
        Mapping {
            matcher: Box::new(ExactMatcher::new(&[176_u8, 42_u8, 127_u8])),
            action: Action::QueueSelected,
        },
        Mapping {
            matcher: Box::new(ExactMatcher::new(&[176_u8, 44_u8, 127_u8])),
            action: Action::PreviousSection,
        },
        Mapping {
            matcher: Box::new(ExactMatcher::new(&[176_u8, 45_u8, 127_u8])),
            action: Action::NextSection,
        },
        Mapping {
            matcher: Box::new(ExactMatcher::new(&[176_u8, 46_u8, 127_u8])),
            action: Action::ToggleLoop,
        },
        Mapping {
            matcher: Box::new(ExactMatcher::new(&[176_u8, 47_u8, 127_u8])),
            action: Action::TogglePlay,
        },
    ]
}

fn on_midi_input(_: u64, message: &[u8], mappings: &[Mapping], context: &mut Context) {
    mappings
        .iter()
        .filter(|mapping| mapping.matcher.matches(message))
        .for_each(|mapping| {
            let _ = context.action_tx.try_send(mapping.action);
        });
}

impl MidiController {
    fn print_midi_inputs(midi_input: &MidiInput) {
        info!("MIDI Input ports:");

        let ports = midi_input.ports();

        ports.iter().enumerate().for_each(|(index, port)| {
            let name = match midi_input.port_name(port) {
                Ok(name) => name,
                Err(_) => return,
            };

            info!("{index}: {name}");
        });

        info!("");
    }

    pub fn new(action_tx: mpsc::Sender<Action>, preferences_dir: &Path) -> Self {
        let midi_input = MidiInput::new("Bloop").expect("Unable to connect to MIDI backend");

        let preferences = read_preferences(preferences_dir).unwrap_or_default();

        let desired_input_device_name = match preferences.input_device {
            Some(input_device) => input_device,
            None => return Self::default(),
        };

        Self::print_midi_inputs(&midi_input);

        let ports = midi_input.ports();
        let port = ports.iter().find(|port| match midi_input.port_name(port) {
            Ok(name) => name.contains(&desired_input_device_name),
            Err(_) => false,
        });

        let mut input_connection: Option<MidiInputConnection<Context>> = None;
        let mappings = get_mappings();

        if let Some(port) = port {
            info!("Connecting to {}", midi_input.port_name(port).unwrap());

            input_connection = match midi_input.connect(
                port,
                "Bloop Input",
                move |timestamp, message, context| on_midi_input(timestamp, message, &mappings, context),
                Context { action_tx },
            ) {
                Ok(connection) => Some(connection),
                Err(error) => {
                    error!("Unable to connect to MIDI input: {error}");
                    None
                }
            }
        }

        Self { input_connection }
    }
}

fn read_preferences(preferences_dir: &Path) -> anyhow::Result<Preferences> {
    let mut preferences_path = preferences_dir.to_path_buf();
    preferences_path.push("midi.json");

    let file = File::open(preferences_path)?;
    let reader = BufReader::new(file);
    let preferences = serde_json::from_reader(reader)?;

    Ok(preferences)
}
