use std::thread::JoinHandle;

use log::info;
use tokio::sync::mpsc;

use crate::{model::Action, preferences::SwitchPreferences};

pub fn run(_action_tx: mpsc::Sender<Action>, _preferences: SwitchPreferences) -> JoinHandle<()> {
    std::thread::spawn(move || {
        info!("Reading switches not implemented on this platform");
    })
}
