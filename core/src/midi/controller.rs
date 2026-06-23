use super::mappings::{load_mappings, Mapping};
use crate::bloop::{Action, MidiDevices, MidiPreferences, Response};
use log::{error, info};
use midir::{MidiInput, MidiInputConnection};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use tokio::sync::{broadcast, mpsc};
use tokio::time::{self, Duration};

const DEFAULT_ENABLED_DEVICE: &str = "iCON G_Boar";
const POLL_INTERVAL_SECS: u64 = 2;

struct Context {
    action_tx: mpsc::Sender<Action>,
}

struct SharedState {
    enabled_patterns: Vec<String>,
    midi_mappings_dir: PathBuf,
    action_tx: mpsc::Sender<Action>,
    /// Active connections, keyed by port name.
    input_connections: Vec<(String, MidiInputConnection<Context>)>,
    /// All port names visible at the last poll.
    known_port_names: Vec<String>,
}

#[allow(dead_code)]
pub struct MidiController {
    shared: Arc<Mutex<SharedState>>,
    poller: tokio::task::JoinHandle<()>,
}

impl Drop for MidiController {
    fn drop(&mut self) {
        self.poller.abort();
    }
}

fn enumerate_port_names() -> Vec<String> {
    super::devices::get_midi_devices().port_names
}

fn try_connect(
    port_name: &str,
    midi_mappings_dir: &Path,
    action_tx: &mpsc::Sender<Action>,
) -> Option<MidiInputConnection<Context>> {
    let port_mappings: Vec<Mapping> = load_mappings(midi_mappings_dir)
        .into_iter()
        .filter(|dm| dm.device_regex.is_match(port_name))
        .flat_map(|dm| dm.mappings)
        .collect();

    let midi_input = match MidiInput::new("Bloop") {
        Ok(input) => input,
        Err(error) => {
            error!("Unable to create MIDI input for {port_name}: {error}");
            return None;
        }
    };

    let ports = midi_input.ports();
    let port = ports
        .iter()
        .find(|p| midi_input.port_name(p).ok().as_deref() == Some(port_name))?;

    match midi_input.connect(
        port,
        "Bloop Input",
        move |_timestamp, message, context| {
            port_mappings.iter().filter(|m| m.matches(message)).for_each(|m| {
                let _ = context.action_tx.try_send(m.action);
            });
        },
        Context {
            action_tx: action_tx.clone(),
        },
    ) {
        Ok(connection) => {
            info!("Connected to MIDI port: {port_name}");
            Some(connection)
        }
        Err(error) => {
            error!("Unable to connect to MIDI port {port_name}: {error}");
            None
        }
    }
}

/// Returns port names from `current_ports` that should have a new connection opened:
/// they are not already connected and match at least one enabled pattern.
pub(crate) fn ports_to_connect<'a>(
    current_ports: &'a [String],
    connected_ports: &[String],
    enabled_patterns: &[String],
) -> Vec<&'a str> {
    current_ports
        .iter()
        .filter(|name| !connected_ports.contains(name))
        .filter(|name| enabled_patterns.iter().any(|p| name.contains(p.as_str())))
        .map(|s| s.as_str())
        .collect()
}

/// Synchronise `state.input_connections` with `current_ports`: drop connections for
/// ports that have disappeared or no longer match any enabled pattern, open new
/// connections for newly visible matching ports.
fn sync_connections(state: &mut SharedState, current_ports: &[String]) {
    let prev_connected: Vec<String> = state.input_connections.iter().map(|(n, _)| n.clone()).collect();

    let enabled_patterns = state.enabled_patterns.clone();
    state
        .input_connections
        .retain(|(name, _)| current_ports.contains(name) && enabled_patterns.iter().any(|p| name.contains(p.as_str())));

    let connected: Vec<String> = state.input_connections.iter().map(|(n, _)| n.clone()).collect();
    let disconnected: Vec<&str> = prev_connected
        .iter()
        .filter(|n| !connected.contains(n))
        .map(|s| s.as_str())
        .collect();
    if !disconnected.is_empty() {
        info!("Disconnecting MIDI ports: {disconnected:?}");
    }
    let to_connect = ports_to_connect(current_ports, &connected, &state.enabled_patterns);
    for port_name in to_connect {
        if let Some(conn) = try_connect(port_name, &state.midi_mappings_dir.clone(), &state.action_tx.clone()) {
            state.input_connections.push((port_name.to_string(), conn));
        }
    }

    state.known_port_names = current_ports.to_vec();
}

fn log_midi_ports(current_ports: &[String]) {
    if current_ports.is_empty() {
        info!("No MIDI ports found");
    } else {
        info!("MIDI ports changed:");
        current_ports.iter().for_each(|p| info!("{p}"));
    }
}

async fn run_poller(shared: Arc<Mutex<SharedState>>, response_tx: broadcast::Sender<Response>) {
    let mut interval = time::interval(Duration::from_secs(POLL_INTERVAL_SECS));
    loop {
        interval.tick().await;

        let current_ports = enumerate_port_names();

        let changed = {
            let state = shared.lock().unwrap();
            state.known_port_names != current_ports
        };

        if changed {
            log_midi_ports(&current_ports);
            let mut state = shared.lock().unwrap();
            sync_connections(&mut state, &current_ports);
            let _ = response_tx.send(Response::default().with_midi_devices(&MidiDevices {
                port_names: current_ports,
                ..Default::default()
            }));
        }
    }
}

impl MidiController {
    pub fn new(
        action_tx: mpsc::Sender<Action>,
        preferences: MidiPreferences,
        midi_mappings_dir: &Path,
        response_tx: broadcast::Sender<Response>,
    ) -> Self {
        let enabled_patterns = if preferences.enabled_devices.is_empty() {
            vec![DEFAULT_ENABLED_DEVICE.to_string()]
        } else {
            preferences.enabled_devices.clone()
        };

        let current_ports = enumerate_port_names();

        let mut state = SharedState {
            enabled_patterns,
            midi_mappings_dir: midi_mappings_dir.to_path_buf(),
            action_tx,
            input_connections: Vec::new(),
            known_port_names: Vec::new(),
        };
        sync_connections(&mut state, &current_ports);

        let shared = Arc::new(Mutex::new(state));
        let poller = tokio::spawn(run_poller(shared.clone(), response_tx));

        Self { shared, poller }
    }

    /// Update the enabled device patterns and immediately re-evaluate connections
    /// against the last-known port list.
    pub fn update_preferences(&self, preferences: MidiPreferences) {
        let enabled_patterns = if preferences.enabled_devices.is_empty() {
            vec![DEFAULT_ENABLED_DEVICE.to_string()]
        } else {
            preferences.enabled_devices.clone()
        };

        let mut state = self.shared.lock().unwrap();
        state.enabled_patterns = enabled_patterns;
        let current_ports = state.known_port_names.clone();
        sync_connections(&mut state, &current_ports);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ports_to_disconnect<'a>(connected_ports: &'a [String], current_ports: &[String]) -> Vec<&'a str> {
        connected_ports
            .iter()
            .filter(|name| !current_ports.contains(name))
            .map(|s| s.as_str())
            .collect()
    }

    #[test]
    fn ports_to_connect_returns_new_matching_ports() {
        let current = vec!["iCON G_Boar V1.03".to_string(), "Generic MIDI".to_string()];
        let connected: Vec<String> = vec![];
        let patterns = vec!["iCON G_Boar".to_string()];

        let result = ports_to_connect(&current, &connected, &patterns);

        assert_eq!(result, vec!["iCON G_Boar V1.03"]);
    }

    #[test]
    fn ports_to_connect_skips_already_connected_ports() {
        let current = vec!["iCON G_Boar V1.03".to_string()];
        let connected = vec!["iCON G_Boar V1.03".to_string()];
        let patterns = vec!["iCON G_Boar".to_string()];

        let result = ports_to_connect(&current, &connected, &patterns);

        assert!(result.is_empty());
    }

    #[test]
    fn ports_to_connect_skips_non_matching_ports() {
        let current = vec!["Generic MIDI".to_string()];
        let connected: Vec<String> = vec![];
        let patterns = vec!["iCON G_Boar".to_string()];

        let result = ports_to_connect(&current, &connected, &patterns);

        assert!(result.is_empty());
    }

    #[test]
    fn ports_to_disconnect_returns_removed_ports() {
        let connected = vec!["iCON G_Boar V1.03".to_string()];
        let current: Vec<String> = vec![];

        let result = ports_to_disconnect(&connected, &current);

        assert_eq!(result, vec!["iCON G_Boar V1.03"]);
    }

    #[test]
    fn ports_to_disconnect_keeps_still_present_ports() {
        let connected = vec!["iCON G_Boar V1.03".to_string()];
        let current = vec!["iCON G_Boar V1.03".to_string()];

        let result = ports_to_disconnect(&connected, &current);

        assert!(result.is_empty());
    }

    #[test]
    fn ports_to_connect_with_updated_patterns_matches_newly_enabled_port() {
        let known_ports = vec!["Launchpad Mini".to_string(), "Generic MIDI".to_string()];
        let connected: Vec<String> = vec![];
        let new_patterns = vec!["Launchpad".to_string()];

        let result = ports_to_connect(&known_ports, &connected, &new_patterns);

        assert_eq!(result, vec!["Launchpad Mini"]);
    }

    #[test]
    fn port_present_but_pattern_removed_is_not_retained() {
        // The retain condition in sync_connections: present AND matches a pattern.
        // Simulate: "iCON G_Boar V1.03" is still physically present but its
        // pattern was removed from enabled_devices.
        let port_name = "iCON G_Boar V1.03".to_string();
        let current_ports = [port_name.clone()];
        let new_patterns = ["Launchpad".to_string()];

        let retained =
            current_ports.contains(&port_name) && new_patterns.iter().any(|p| port_name.contains(p.as_str()));

        assert!(
            !retained,
            "port still present but pattern removed — should be disconnected"
        );
    }

    #[test]
    fn port_present_and_pattern_matches_is_retained() {
        let port_name = "iCON G_Boar V1.03".to_string();
        let current_ports = [port_name.clone()];
        let patterns = ["iCON G_Boar".to_string()];

        let retained = current_ports.contains(&port_name) && patterns.iter().any(|p| port_name.contains(p.as_str()));

        assert!(retained);
    }
}
