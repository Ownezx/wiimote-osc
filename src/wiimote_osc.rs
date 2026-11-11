//! Wiimote OSC core logic

use open_sound_control::{OscArgument, OscMessage, OscSender};
use std::net::IpAddr;
use std::thread;
use std::time::Instant;
use wiiuse_rs::WiimoteSet;

use crate::constants::{
    FIND_TIMEOUT, MAX_WIIMOTES, OSC_SEND_INTERVAL, POLL_INTERVAL, WIIMOTE_NUMBER,
};

/// Connect to a Wii Remote and stream its accelerometer over OSC/UDP.
pub fn run(destination_ip: IpAddr, destination_port: u16) -> Result<(), String> {
    let destination_ip_text = destination_ip.to_string();
    let osc_sender = OscSender::new(destination_ip_text.clone(), u32::from(destination_port));

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
    println!("Sending accelerometer OSC to {destination_ip_text}:{destination_port} at 50 Hz.");
    println!("Addresses: /wm{WIIMOTE_NUMBER}/ax, /wm{WIIMOTE_NUMBER}/ay, /wm{WIIMOTE_NUMBER}/az");
    println!("Press Ctrl+C to stop.");

    let mut next_osc_send = Instant::now();

    loop {
        // Poll frequently, but throttle OSC output independently to 50 Hz.
        devices.poll();

        if let Some(state) = devices.state(0) {
            let now = Instant::now();
            if now >= next_osc_send {
                send_accelerometer(&osc_sender, WIIMOTE_NUMBER, state.accelerometer.gforce);
                next_osc_send = now + OSC_SEND_INTERVAL;
            }
        }

        thread::sleep(POLL_INTERVAL);
    }
}

/// Send X, Y, and Z G-force values as three OSC float messages.
fn send_accelerometer(sender: &OscSender, wiimote_number: usize, gforce: [f32; 3]) {
    let prefix = format!("/wm{wiimote_number}");

    for (axis, value) in [("ax", gforce[0]), ("ay", gforce[1]), ("az", gforce[2])] {
        let message = OscMessage {
            address: format!("{prefix}/{axis}"),
            arguments: vec![OscArgument::Float32(value)],
        };
        sender.send_message(&message);
    }
}
