use std::env;
use std::thread;
use std::time::{Duration, Instant};

use open_sound_control::{OscArgument, OscMessage, OscSender};
use wiiuse_rs::WiimoteSet;

const MAX_WIIMOTES: usize = 1;
const WIIMOTE_NUMBER: usize = 1;
const FIND_TIMEOUT: Duration = Duration::from_secs(10);
const POLL_INTERVAL: Duration = Duration::from_millis(2);
const OSC_SEND_INTERVAL: Duration = Duration::from_millis(20); // 50 Hz

fn main() {
    let mut args = env::args().skip(1);
    let destination_ip = args.next().unwrap_or_else(|| "127.0.0.1".to_string());
    let destination_port = args
        .next()
        .map(|value| {
            value
                .parse::<u16>()
                .unwrap_or_else(|_| panic!("Invalid UDP port: {value}"))
        })
        .unwrap_or(9000);

    // open_sound_control sends OSC messages over UDP.
    let osc_sender = OscSender::new(destination_ip.clone(), u32::from(destination_port));

    println!("Searching for Wii Remote...");
    println!("Press 1 + 2 on the controller.");

    // WiimoteSet owns the native device array and cleans it up automatically.
    let mut devices = match WiimoteSet::new(MAX_WIIMOTES) {
        Ok(devices) => devices,
        Err(error) => {
            eprintln!("Could not allocate Wii Remote state: {error}");
            return;
        }
    };

    let found = devices.find(FIND_TIMEOUT);
    if found == 0 {
        eprintln!("No Wii Remote found.");
        return;
    }

    println!("Found {found} Wii Remote(s). Connecting...");
    let connected = devices.connect();
    if connected == 0 {
        eprintln!("Could not connect to the Wii Remote.");
        return;
    }

    // Enable the Wii Remote accelerometer.
    devices.motion_sensing(0, true);

    println!("Connected.");
    println!("Sending accelerometer OSC to {destination_ip}:{destination_port} at 50 Hz.");
    println!("Addresses: /wm{WIIMOTE_NUMBER}/ax, /ay, /az");
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

/// Send the X, Y, and Z G-force values as three OSC float messages.
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
