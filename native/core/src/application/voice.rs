//! Voice use case: enable/disable, interpret an utterance, expose status.
//! The IPC layer wires a resolved [`VoiceAction`] to hardware / the catalog.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use crate::domain::ParsedUtterance;
use crate::infrastructure::voice::{engine_name, parse_utterance};

pub struct VoiceService {
    enabled: AtomicBool,
    last_wake: Mutex<Option<Instant>>,
}

impl Default for VoiceService {
    fn default() -> Self {
        Self::new(true)
    }
}

impl VoiceService {
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled: AtomicBool::new(enabled),
            last_wake: Mutex::new(None),
        }
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::Relaxed);
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }

    /// Parse an utterance. Records the wake-word timestamp so the UI can flash
    /// its indicator. Returns the parse even when voice is disabled (callers
    /// decide whether to act).
    pub fn interpret(&self, text: &str) -> ParsedUtterance {
        let parsed = parse_utterance(text);
        if parsed.wake {
            *self.last_wake.lock().unwrap() = Some(Instant::now());
        }
        parsed
    }

    pub fn seconds_since_wake(&self) -> Option<u64> {
        self.last_wake
            .lock()
            .unwrap()
            .map(|t| t.elapsed().as_secs())
    }

    pub fn engine(&self) -> &'static str {
        engine_name()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::VoiceAction;

    #[test]
    fn interpret_records_wake_and_maps_action() {
        let svc = VoiceService::new(true);
        assert!(svc.seconds_since_wake().is_none());

        let p = svc.interpret("ok hub allume la tv");
        assert!(p.wake);
        assert_eq!(
            p.action,
            Some(VoiceAction::Cec {
                action: "tv_on".into()
            })
        );
        assert!(svc.seconds_since_wake().is_some());
    }

    #[test]
    fn enable_toggle() {
        let svc = VoiceService::new(true);
        assert!(svc.is_enabled());
        svc.set_enabled(false);
        assert!(!svc.is_enabled());
    }

    #[test]
    fn engine_is_stub_by_default() {
        assert_eq!(VoiceService::new(true).engine(), "stub");
    }
}
