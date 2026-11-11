//! Wiimote OSC maps Wii remote data to specific OSC actions in order to control lights

mod constants;
mod wiimote_osc;

use clap::Parser;
use constants::{DEFAULT_DESTINATION_IP, DEFAULT_DESTINATION_PORT};
use std::net::IpAddr;

/// Stream Wii Remote accelerometer readings as OSC floats over UDP.
#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    /// Destination IP address for OSC/UDP messages.
    #[arg(short, long, default_value = DEFAULT_DESTINATION_IP, value_name = "IP")]
    ip: IpAddr,

    /// Destination UDP port for OSC messages.
    #[arg(short, long, default_value_t = DEFAULT_DESTINATION_PORT, value_name = "PORT")]
    port: u16,
}

fn main() {
    let args = Args::parse();

    println!("OSC stream target: {}:{}", args.ip, args.port);

    if let Err(error) = wiimote_osc::run(args.ip, args.port) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
