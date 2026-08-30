//! Platform detection + a GPIO facade.
//!
//! Real GPIO lands in Sprint 7 (client to the C daemon over
//! `/run/mjqbe/daemon.sock`). Until then, and on any non-Pi host, every
//! hardware call fails cleanly with [`CoreError::HardwareUnavailable`] so the
//! rest of the app keeps working ("stub mode hors Pi").

mod daemon_client;
pub use daemon_client::DaemonClient;

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    RaspberryPi,
    Stub,
}

impl Platform {
    /// `MJQBE_STUB=1` forces stub mode; otherwise we look at the device tree.
    pub fn detect() -> Self {
        if std::env::var("MJQBE_STUB")
            .map(|v| v == "1")
            .unwrap_or(false)
        {
            return Platform::Stub;
        }
        match std::fs::read_to_string("/proc/device-tree/model") {
            Ok(model) if model.to_lowercase().contains("raspberry pi") => Platform::RaspberryPi,
            _ => Platform::Stub,
        }
    }

    pub fn is_pi(self) -> bool {
        matches!(self, Platform::RaspberryPi)
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Platform::RaspberryPi => "raspberry-pi",
            Platform::Stub => "stub",
        })
    }
}

// GPIO / relay / LED transport lives in [`DaemonClient`] (client to the C
// daemon over `/run/mjqbe/daemon.sock`). Off-Pi, either the socket is absent
// (→ `HardwareUnavailable`) or the daemon itself runs in stub mode.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stub_is_not_pi() {
        assert!(!Platform::Stub.is_pi());
        assert_eq!(Platform::Stub.to_string(), "stub");
        assert_eq!(Platform::RaspberryPi.to_string(), "raspberry-pi");
    }
}
