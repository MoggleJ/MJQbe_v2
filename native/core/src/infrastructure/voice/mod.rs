//! Voice recognition.
//!
//! Sprint 9 ships the **grammar** (wake word + command → action) fully, plus a
//! stub audio engine. The real offline recogniser (Vosk) lives behind the
//! `vosk` cargo feature (`vosk_engine`) and is not built by default — it needs
//! `libvosk` and a language model on the target, which only exist on the Pi.

mod grammar;

pub use grammar::{normalise, parse_utterance};

#[cfg(feature = "vosk")]
mod vosk_engine;

/// Which recogniser backend is active.
pub fn engine_name() -> &'static str {
    if cfg!(feature = "vosk") {
        "vosk"
    } else {
        "stub"
    }
}
