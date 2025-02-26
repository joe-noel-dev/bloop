use log::{debug, info, warn};
use rppal::gpio::{Gpio, InputPin};
use std::{
    collections::{HashMap, HashSet},
    thread::JoinHandle,
    time::{Duration, Instant},
};

use tokio::sync::mpsc;

use crate::{
    model::Action,
    preferences::{Gesture, SwitchMapping, SwitchPreferences},
};

pub fn run(action_tx: mpsc::Sender<Action>, preferences: SwitchPreferences) -> JoinHandle<()> {
    std::thread::spawn(move || run_thread(preferences, action_tx))
}

const HOLD_DURATION: Duration = Duration::from_millis(300);

fn run_thread(preferences: SwitchPreferences, action_tx: mpsc::Sender<Action>) {
    info!("Starting switch thread");

    let gpio = Gpio::new().expect("Error initializing GPIO");
    let pins = init_gpio_pins(&preferences, &gpio);
    let mut press_times = HashMap::new();

    info!("Input pins configured");

    loop {
        let timeout = Duration::from_millis(50);
        let pins_to_poll: Vec<&InputPin> = pins.iter().collect();

        let result = match gpio.poll_interrupts(&pins_to_poll, false, Some(timeout)) {
            Ok(poll_result) => poll_result,
            Err(error) => {
                warn!("Error polling interrupts: {}", error);
                continue;
            }
        };

        let (input_pin, event) = match result {
            Some(result) => result,
            None => {
                on_tick(&mut press_times, &preferences.mappings, &action_tx);
                continue;
            }
        };

        match event.trigger {
            rppal::gpio::Trigger::RisingEdge => {
                on_release(input_pin.pin(), &preferences.mappings, &mut press_times, &action_tx)
            }
            rppal::gpio::Trigger::FallingEdge => {
                on_press(input_pin.pin(), &preferences.mappings, &mut press_times, &action_tx)
            }
            _ => continue,
        }
    }
}

fn init_gpio_pins(preferences: &SwitchPreferences, gpio: &Gpio) -> Vec<InputPin> {
    let pins = preferences
        .mappings
        .iter()
        .map(|mapping| mapping.pin)
        .collect::<HashSet<u8>>();

    pins.iter()
        .map(|&pin| {
            let mut gpio_pin = gpio
                .get(pin)
                .unwrap_or_else(|_| panic!("Error getting pin: {}", pin))
                .into_input_pulldown();
            debug!("Configuring input pin: {}", pin);
            gpio_pin
                .set_interrupt(rppal::gpio::Trigger::Both, Some(Duration::from_millis(10)))
                .unwrap_or_else(|_| panic!("Error setting interrupt on pin: {}", pin));
            gpio_pin
        })
        .collect()
}

fn on_press(
    pin: u8,
    mappings: &[SwitchMapping],
    press_times: &mut HashMap<u8, Instant>,
    action_tx: &mpsc::Sender<Action>,
) {
    debug!("Pressed pin: {}", pin);

    press_times.insert(pin, Instant::now());

    let mapping = match mappings
        .iter()
        .find(|mapping| mapping.pin == pin && mapping.gesture == Gesture::Press)
    {
        Some(mapping) => mapping,
        None => return,
    };

    let _ = action_tx.blocking_send(mapping.action);
}

fn on_release(
    pin: u8,
    mappings: &[SwitchMapping],
    press_times: &mut HashMap<u8, Instant>,
    action_tx: &mpsc::Sender<Action>,
) {
    debug!("Released pin: {}", pin);

    let duration = match press_times.remove(&pin) {
        Some(instant) => instant.elapsed(),
        None => return,
    };

    if duration <= HOLD_DURATION {
        let mapping = match mappings
            .iter()
            .find(|mapping| mapping.pin == pin && mapping.gesture == Gesture::Release)
        {
            Some(mapping) => mapping,
            None => return,
        };

        let _ = action_tx.blocking_send(mapping.action);
    }
}

fn on_tick(press_times: &mut HashMap<u8, Instant>, mappings: &[SwitchMapping], action_tx: &mpsc::Sender<Action>) {
    for (index, press_time) in press_times.iter() {
        if press_time.elapsed() > HOLD_DURATION {
            let mapping = match mappings
                .iter()
                .find(|mapping| mapping.pin == *index && mapping.gesture == Gesture::Hold)
            {
                Some(mapping) => mapping,
                None => continue,
            };

            let _ = action_tx.blocking_send(mapping.action);
        }
    }
}
