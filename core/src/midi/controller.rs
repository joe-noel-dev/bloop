use super::mappings::{load_mappings, Mapping};
use crate::bloop::{Action, MidiPreferences};
use log::{error, info};
use midir::{MidiInput, MidiInputConnection};
use std::path::Path;
use tokio::sync::mpsc;

#[derive(Default)]
#[allow(dead_code)]
pub struct MidiController {
    input_connections: Vec<MidiInputConnection<Context>>,
}

struct Context {
    action_tx: mpsc::Sender<Action>,
}

const DEFAULT_ENABLED_DEVICE: &str = "iCON G_Boar";

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
    pub fn new(action_tx: mpsc::Sender<Action>, preferences: MidiPreferences, midi_mappings_dir: &Path) -> Self {
        let midi_input = match MidiInput::new("Bloop") {
            Ok(input) => input,
            Err(error) => {
                error!("Unable to connect to MIDI backend: {error}");
                return Self::default();
            }
        };

        let enabled_patterns: Vec<String> = if preferences.enabled_devices.is_empty() {
            vec![DEFAULT_ENABLED_DEVICE.to_string()]
        } else {
            preferences.enabled_devices.clone()
        };

        print_midi_inputs(&midi_input);

        let ports = midi_input.ports();

        let mut input_connections: Vec<MidiInputConnection<Context>> = Vec::new();

        for port in &ports {
            let name = match midi_input.port_name(port) {
                Ok(name) => name,
                Err(_) => continue,
            };

            let matches = enabled_patterns.iter().any(|pattern| name.contains(pattern.as_str()));
            if !matches {
                continue;
            }

            info!("Connecting to {name}");

            // `MidiInput::connect` consumes the `MidiInput`, so each connection needs its own instance.
            // Create a fresh `MidiInput` per matching port and keep the resulting connections alive.
            let port_name = name;
            let port_mappings: Vec<Mapping> = load_mappings(midi_mappings_dir)
                .into_iter()
                .filter(|dm| dm.device_regex.is_match(&port_name))
                .flat_map(|dm| dm.mappings)
                .collect();
            let action_tx_clone = action_tx.clone();

            let fresh_input = match MidiInput::new("Bloop") {
                Ok(input) => input,
                Err(error) => {
                    error!("Unable to create MIDI input for {port_name}: {error}");
                    continue;
                }
            };

            let fresh_ports = fresh_input.ports();
            let fresh_port = fresh_ports
                .iter()
                .find(|p| fresh_input.port_name(p).ok().as_deref() == Some(port_name.as_str()));

            let Some(fresh_port) = fresh_port else {
                error!("MIDI input port {port_name} disappeared before connecting");
                continue;
            };

            match fresh_input.connect(
                fresh_port,
                "Bloop Input",
                move |timestamp, message, context| {
                    on_midi_input(timestamp, message, mappings_clone.as_slice(), context)
                },
                Context {
                    action_tx: action_tx_clone,
                },
            ) {
                Ok(connection) => {
                    input_connections.push(connection);
                }
                Err(error) => {
                    error!("Unable to connect to MIDI input {port_name}: {error}");
                }
            }
        }

        Self { input_connections }
    }
}
