#[cfg(feature = "midi")]
mod controller;
#[cfg(feature = "midi")]
mod matcher;

#[cfg(not(feature = "midi"))]
mod fallback;

#[cfg(feature = "midi")]
pub use controller::MidiController;

#[cfg(not(feature = "midi"))]
pub use fallback::MidiController;
