use std::time::Duration;

pub const MAX_WIIMOTES: usize = 1;
pub const WIIMOTE_NUMBER: usize = 1;

pub const FIND_TIMEOUT: Duration = Duration::from_secs(10);
pub const POLL_INTERVAL: Duration = Duration::from_millis(2);
pub const OSC_SEND_INTERVAL: Duration = Duration::from_millis(20);

pub const DEFAULT_DESTINATION_IP: &str = "127.0.0.1";
pub const DEFAULT_DESTINATION_PORT: u16 = 10123;
