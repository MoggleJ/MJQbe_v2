//! Platform detection + a GPIO facade.
//!
//! Real GPIO lands in Sprint 7 (client to the C daemon over
//! `/run/mjqbe/daemon.sock`). Until then, and on any non-Pi host, every
//! hardware call fails cleanly with [`CoreError::HardwareUnavailable`] so the
//! rest of the app keeps working ("stub mode hors Pi").

use std::fmt;

use crate::domain::CoreError;

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

/// GPIO entry point. Sprint 3 only wires platform-awareness; the transport to
/// the C daemon is added in Sprint 7.
pub struct Gpio {
    platform: Platform,
}

impl Gpio {
    pub fn new(platform: Platform) -> Self {
        Self { platform }
    }

    pub fn set(&self, _pin: u8, _value: bool) -> Result<(), CoreError> {
        if self.platform.is_pi() {
            Err(CoreError::Internal(
                "GPIO transport not implemented until Sprint 7".into(),
            ))
        } else {
            Err(CoreError::HardwareUnavailable)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stub_is_not_pi() {
        assert!(!Platform::Stub.is_pi());
        assert_eq!(Platform::Stub.to_string(), "stub");
        assert_eq!(Platform::RaspberryPi.to_string(), "raspberry-pi");
    }

    #[test]
    fn gpio_on_stub_reports_unavailable() {
        let gpio = Gpio::new(Platform::Stub);
        assert!(matches!(
            gpio.set(23, true).unwrap_err(),
            CoreError::HardwareUnavailable
        ));
    }
}
