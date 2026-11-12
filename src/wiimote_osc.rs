//! Wiimote OSC core logic

use open_sound_control::{OscArgument, OscMessage, OscSender};
use std::net::IpAddr;
use std::thread;
use std::time::Instant;
use wiiuse_rs::{NunchukButtons, WiimoteButtons, WiimoteSet, WiimoteState};

use crate::constants::{
    FIND_TIMEOUT, FUNCTION_LONG_PRESS, MAX_WIIMOTES, OSC_SEND_INTERVAL, POLL_INTERVAL,
    WIIMOTE_NUMBER,
};

/// Tracks the state of an individual function toggle.
#[derive(Default)]
struct FunctionToggle {
    /// Time since the button has been started
    pressed_since: Option<Instant>,
    /// The button has already been triggered
    triggered: bool,
    /// Current status of the toggle.
    enabled: bool,
}

/// Tracks all four function toggles.
#[derive(Default)]
struct FunctionToggles {
    ///Array containing the value of all four toggles.
    functions: [FunctionToggle; 4],
}

/// Connect to a Wii Remote and stream its state over OSC/UDP.
pub fn run(destination_ip: IpAddr, destination_port: u16) -> Result<(), String> {
    let destination_ip_text = destination_ip.to_string();
    let osc_sender = OscSender::new(destination_ip_text.clone(), u32::from(destination_port));

    let mut devices = connect_wiimote()?;
    let mut function_toggles = FunctionToggles::default();

    println!("Sending Wiimote OSC to {destination_ip_text}:{destination_port} at 50 Hz.");
    println!("Press Ctrl+C to stop.");

    let mut next_osc_send = Instant::now();

    loop {
        devices.poll();

        if let Some(state) = devices.state(0) {
            // Update toggle states on every controller poll.
            update_function_toggles(&state, &mut function_toggles);

            let now = Instant::now();

            if now >= next_osc_send {
                send_accelerometer(&osc_sender, WIIMOTE_NUMBER, &state);
                send_buttons(&osc_sender, WIIMOTE_NUMBER, &state);
                send_function_states(&osc_sender, WIIMOTE_NUMBER, &function_toggles);

                next_osc_send = now + OSC_SEND_INTERVAL;
            }
        }

        thread::sleep(POLL_INTERVAL);
    }
}

/// Initialize, discover, and connect to a Wii Remote.
///
/// Returns an initialized WiimoteSet with accelerometer sensing enabled.
fn connect_wiimote() -> Result<WiimoteSet, String> {
    println!("Searching for Wii Remote...");
    println!("Press 1 + 2 on the controller.");

    // WiimoteSet owns the native device array and cleans it up automatically.
    let mut devices = WiimoteSet::new(MAX_WIIMOTES)
        .map_err(|error| format!("Could not allocate Wii Remote state: {error}"))?;

    let found = devices.find(FIND_TIMEOUT);

    if found == 0 {
        return Err("No Wii Remote found.".to_string());
    }

    println!("Found {found} Wii Remote(s). Connecting...");

    let connected = devices.connect();

    if connected == 0 {
        return Err("Could not connect to the Wii Remote.".to_string());
    }

    // Enable the Wii Remote accelerometer.
    devices.motion_sensing(0, true);

    println!("Connected.");

    Ok(devices)
}

/// Send Wii Remote and optional Nunchuk X, Y, and Z G-force values as OSC float messages.
fn send_accelerometer(sender: &OscSender, wiimote_number: usize, state: &WiimoteState) {
    let prefix = format!("/wm{wiimote_number}");
    let gforce = state.accelerometer.gforce;

    // Wii Remote accelerometer.
    for (axis, value) in [("ax", gforce[0]), ("ay", gforce[1]), ("az", gforce[2])] {
        let message = OscMessage {
            address: format!("{prefix}/{axis}"),
            arguments: vec![OscArgument::Float32(value)],
        };

        sender.send_message(&message);
    }

    // Nunchuk accelerometer (only when connected).
    if let Some(nunchuk) = &state.nunchuk {
        let gforce = nunchuk.accelerometer.gforce;

        for (axis, value) in [("ax", gforce[0]), ("ay", gforce[1]), ("az", gforce[2])] {
            let message = OscMessage {
                address: format!("{prefix}/n/{axis}"),
                arguments: vec![OscArgument::Float32(value)],
            };

            sender.send_message(&message);
        }
    }
}

/// Send Wii Remote and optional Nunchuk button states as OSC integer messages.
fn send_buttons(sender: &OscSender, wiimote_number: usize, state: &WiimoteState) {
    let prefix = format!("/wm{wiimote_number}");

    let buttons = [
        ("ba", WiimoteButtons::A),
        ("bb", WiimoteButtons::B),
        ("b1", WiimoteButtons::ONE),
        ("b2", WiimoteButtons::TWO),
        ("bp", WiimoteButtons::PLUS),
        ("bm", WiimoteButtons::MINUS),
        ("bh", WiimoteButtons::HOME),
    ];

    for (address, button) in buttons {
        let pressed = state.buttons.down.contains(button) || state.buttons.held.contains(button);

        let message = OscMessage {
            address: format!("{prefix}/{address}"),
            arguments: vec![OscArgument::Int32(if pressed { 1 } else { 0 })],
        };

        sender.send_message(&message);
    }

    // Send Nunchuk button state only while a Nunchuk is connected.
    if let Some(nunchuk) = &state.nunchuk {
        let pressed = nunchuk.buttons.down.contains(NunchukButtons::Z)
            || nunchuk.buttons.held.contains(NunchukButtons::Z);

        let message = OscMessage {
            address: format!("{prefix}/n/bz"),
            arguments: vec![OscArgument::Int32(if pressed { 1 } else { 0 })],
        };

        sender.send_message(&message);
    }
}

/// Send all four function toggle states as OSC integer messages.
fn send_function_states(sender: &OscSender, wiimote_number: usize, toggles: &FunctionToggles) {
    let prefix = format!("/wm{wiimote_number}");

    for (index, function) in toggles.functions.iter().enumerate() {
        let message = OscMessage {
            address: format!("{prefix}/f{}", index + 1),
            arguments: vec![OscArgument::Int32(if function.enabled { 1 } else { 0 })],
        };

        sender.send_message(&message);
    }
}

/// Handle Nunchuk C + D-pad long presses.
///
/// Each combination must be held continuously for one second.
/// A function toggles only once per long press.
fn update_function_toggles(state: &WiimoteState, toggles: &mut FunctionToggles) {
    let nunchuk = match &state.nunchuk {
        Some(nunchuk) => nunchuk,
        None => {
            for function in &mut toggles.functions {
                function.pressed_since = None;
                function.triggered = false;
            }
            return;
        }
    };

    let c_is_down = nunchuk.buttons.down.contains(NunchukButtons::C)
        || nunchuk.buttons.held.contains(NunchukButtons::C);

    let directions = [
        WiimoteButtons::UP,
        WiimoteButtons::RIGHT,
        WiimoteButtons::DOWN,
        WiimoteButtons::LEFT,
    ];

    let now = Instant::now();

    for (index, direction) in directions.into_iter().enumerate() {
        let direction_is_down =
            state.buttons.down.contains(direction) || state.buttons.held.contains(direction);

        let combination_is_down = c_is_down && direction_is_down;
        let function = &mut toggles.functions[index];

        if !combination_is_down {
            function.pressed_since = None;
            function.triggered = false;
            continue;
        }

        let pressed_since = *function.pressed_since.get_or_insert(now);

        if !function.triggered && now.duration_since(pressed_since) >= FUNCTION_LONG_PRESS {
            function.enabled = !function.enabled;
            function.triggered = true;
        }
    }
}
