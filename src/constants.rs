//! Wiimote OSC default values

use std::time::Duration;

/// Max wiimote connections possible
pub const MAX_WIIMOTES: usize = 1;
/// Single wiimote default number
pub const WIIMOTE_NUMBER: usize = 1;

/// Timout for wiimote connection
pub const FIND_TIMEOUT: Duration = Duration::from_secs(10);
/// Data poll interval
pub const POLL_INTERVAL: Duration = Duration::from_millis(2);
/// OSC data send interval
pub const OSC_SEND_INTERVAL: Duration = Duration::from_millis(20);

/// Default OSC stream target ip
pub const DEFAULT_DESTINATION_IP: &str = "127.0.0.1";
/// Default OSC stream target port
pub const DEFAULT_DESTINATION_PORT: u16 = 10123;

/// Long press time
pub const FUNCTION_LONG_PRESS: Duration = Duration::from_millis(250);
