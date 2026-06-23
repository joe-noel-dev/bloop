use super::mappings::{load_mappings, Mapping};
use crate::bloop::{Action, MidiPreferences};
use log::{error, info};
use midir::{MidiInput, MidiInputConnection};
use std::path::Path;
use tokio::sync::mpsc;

#[derive(Default)]
#[allow(dead_code)]
pub struct MidiController {
    input_connection: Option<MidiInputConnection<Context>>,
}

struct Context {
    action_tx: mpsc::Sender<Action>,
}

const DEFAULT_DEVICE_NAME: &str = "iCON G_Boar V1.03";

fn on_midi_input(_: u64, message: &[u8], mappings: &[Mapping], context: &mut Context) {
    mappings
        .iter()
        .filter(|mapping| mapping.matches(message))
        .for_each(|mapping| {
            let _ = context.action_tx.try_send(mapping.action);
        });
}

fn print_midi_inputs(midi_input: &MidiInput) {
    info!("MIDI Input ports:");

    let ports = midi_input.ports();

    if ports.is_empty() {
        info!("No MIDI input devices found");
        return;
    }

    ports.iter().enumerate().for_each(|(index, port)| {
        let name = match midi_input.port_name(port) {
            Ok(name) => name,
            Err(_) => return,
        };

        info!("{index}: {name}");
    });
}

impl MidiController {
    pub fn new(
        action_tx: mpsc::Sender<Action>,
        preferences: MidiPreferences,
        midi_mappings_dir: &Path,
    ) -> Self {
        let midi_input = match MidiInput::new("Bloop") {
            Ok(input) => input,
            Err(error) => {
                error!("Unable to connect to MIDI backend: {error}");
                return Self::default();
            }
        };

        let desired_input_device_name = if preferences.input_device.is_empty() {
            DEFAULT_DEVICE_NAME.to_string()
        } else {
            preferences.input_device.clone()
        };

        print_midi_inputs(&midi_input);

        let ports = midi_input.ports();
        let port = ports.iter().find(|port| match midi_input.port_name(port) {
            Ok(name) => name.contains(&desired_input_device_name),
            Err(_) => false,
        });

        let mut input_connection: Option<MidiInputConnection<Context>> = None;

        if let Some(port) = port {
            let port_name = midi_input.port_name(port).unwrap();
            info!("Connecting to {port_name}");

            let all_device_mappings = load_mappings(midi_mappings_dir);
            let mappings: Vec<Mapping> = all_device_mappings
                .into_iter()
                .filter(|dm| dm.device_regex.is_match(&port_name))
                .flat_map(|dm| dm.mappings)
                .collect();

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
