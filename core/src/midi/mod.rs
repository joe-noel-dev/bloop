#[cfg(feature = "midi")]
mod controller;
#[cfg(feature = "midi")]
mod devices;
#[cfg(feature = "midi")]
mod mappings;
#[cfg(feature = "midi")]
mod matcher;

#[cfg(not(feature = "midi"))]
mod fallback;

#[cfg(feature = "midi")]
pub use controller::MidiController;
#[cfg(feature = "midi")]
pub use devices::get_midi_devices;

#[cfg(not(feature = "midi"))]
pub use fallback::get_midi_devices;
#[cfg(not(feature = "midi"))]
pub use fallback::MidiController;
