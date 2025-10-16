use crate::bloop::SwitchPreferences;
use crate::model::Action;
use log::info;
use std::thread::JoinHandle;
use tokio::sync::mpsc;

pub fn run(_action_tx: mpsc::Sender<Action>, _preferences: SwitchPreferences) -> JoinHandle<()> {
    std::thread::spawn(move || {
        info!("Reading switches not implemented on this platform");
    })
}
