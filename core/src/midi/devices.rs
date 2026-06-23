use crate::bloop::MidiDevices;
use log::warn;
use midir::MidiInput;

pub fn get_midi_devices() -> MidiDevices {
    let midi_input = match MidiInput::new("Bloop") {
        Ok(input) => input,
        Err(error) => {
            warn!("Unable to enumerate MIDI ports: {error}");
            return MidiDevices::default();
        }
    };

    let ports = midi_input.ports();
    let mut port_names = Vec::new();

    for port in &ports {
        match midi_input.port_name(port) {
            Ok(name) => port_names.push(name),
            Err(error) => warn!("Unable to get MIDI port name: {error}"),
        }
    }

    MidiDevices {
        port_names,
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_midi_devices_returns_midi_devices() {
        let devices = get_midi_devices();
        // On CI there may be no MIDI ports; we just verify the struct is well-formed.
        assert!(devices.port_names.iter().all(|n| !n.is_empty()));
    }
}
