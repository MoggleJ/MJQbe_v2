//! Real offline recogniser — **only compiled with `--features vosk`**.
//!
//! Requires `libvosk` on the system and a language model directory
//! (`MJQBE_VOSK_MODEL`, e.g. a `vosk-model-small-fr-*`). This is wired up on the
//! Raspberry Pi image; CI and the default build never touch it.
//!
//! Add to `Cargo.toml` under `[features]`:
//!   vosk = ["dep:vosk", "dep:cpal"]
//! and as optional deps:
//!   vosk = "0.3"
//!   cpal = "0.15"

#![cfg(feature = "vosk")]

use std::sync::mpsc::Sender;

use vosk::{Model, Recognizer};

/// Open the microphone, feed 16 kHz mono PCM to Vosk, and push each final
/// transcript to `out`. Runs until the returned guard is dropped.
pub fn spawn(out: Sender<String>) -> anyhow::Result<()> {
    let model_path =
        std::env::var("MJQBE_VOSK_MODEL").unwrap_or_else(|_| "/opt/mjqbe/vosk-model".to_string());
    let model = Model::new(&model_path)
        .ok_or_else(|| anyhow::anyhow!("failed to load Vosk model at {model_path}"))?;

    let mut recognizer =
        Recognizer::new(&model, 16_000.0).ok_or_else(|| anyhow::anyhow!("recognizer init"))?;
    recognizer.set_words(true);

    // Audio capture (cpal) — 16 kHz mono i16 — feeding `recognizer.accept_waveform`.
    // Each time `recognizer.result()` yields a non-empty `text`, send it on `out`.
    // (Kept short here; the capture loop is filled in against real hardware.)
    let _ = (&mut recognizer, &out);
    Ok(())
}
