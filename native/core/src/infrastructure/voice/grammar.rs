//! Wake-word + command grammar (French). Pure functions, heavily unit-tested.

use crate::domain::{ParsedUtterance, VoiceAction};

const WAKE_PHRASES: [&str; 4] = ["ok hub", "okay hub", "ok qube", "ok cube"];

/// Lowercase, strip common French accents, collapse whitespace.
pub fn normalise(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for ch in text.trim().to_lowercase().chars() {
        let mapped = match ch {
            'à' | 'â' | 'ä' => 'a',
            'é' | 'è' | 'ê' | 'ë' => 'e',
            'î' | 'ï' => 'i',
            'ô' | 'ö' => 'o',
            'ù' | 'û' | 'ü' => 'u',
            'ç' => 'c',
            c => c,
        };
        if mapped.is_alphanumeric() || mapped == ' ' {
            out.push(mapped);
        } else {
            out.push(' ');
        }
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Interpret one utterance: detect the wake word, then map the remainder.
pub fn parse_utterance(raw: &str) -> ParsedUtterance {
    let text = normalise(raw);

    let mut wake = false;
    let mut rest = text.as_str();
    for phrase in WAKE_PHRASES {
        if let Some(stripped) = text.strip_prefix(phrase) {
            wake = true;
            rest = stripped.trim();
            break;
        }
    }
    // Also accept the wake word anywhere, e.g. "dis hub allume la tv".
    if !wake {
        if let Some(pos) = text.find("hub ") {
            if text[..pos].split_whitespace().count() <= 2 {
                wake = true;
                rest = text[pos + 4..].trim();
            }
        }
    }

    let command = if rest.is_empty() {
        None
    } else {
        Some(rest.to_string())
    };
    let action = command.as_deref().and_then(map_command);

    ParsedUtterance {
        wake,
        command,
        action,
    }
}

/// Map a normalised command (no wake word) to an action.
pub fn map_command(cmd: &str) -> Option<VoiceAction> {
    let cec = |a: &str| {
        Some(VoiceAction::Cec {
            action: a.to_string(),
        })
    };
    let relay = |on| Some(VoiceAction::Relay { relay: 1, on });

    let on = ["allume", "démarre", "demarre", "lance", "ouvre", "active"];
    let off = ["eteins", "arrete", "arrête", "coupe", "ferme", "desactive"];

    let starts_with_any = |list: &[&str]| list.iter().any(|w| cmd.starts_with(w));
    let has = |needle: &str| cmd.contains(needle);

    let turning_on = starts_with_any(&on);
    let turning_off = starts_with_any(&off);

    if has("tv") || has("television") || has("tele") {
        return if turning_off {
            cec("tv_off")
        } else {
            cec("tv_on")
        };
    }
    if has("ps4") || has("playstation") || has("console") {
        return if turning_off {
            cec("ps4_off")
        } else {
            cec("ps4_on")
        };
    }
    if has("hub") || has("concentrateur") || has("station") {
        return if turning_off {
            relay(false)
        } else {
            relay(true)
        };
    }

    // "lance netflix" / "ouvre youtube" → app launch (the caller resolves the name)
    if turning_on {
        let query = cmd
            .split_whitespace()
            .skip(1) // drop the verb
            .filter(|w| !matches!(*w, "la" | "le" | "les" | "l" | "un" | "une"))
            .collect::<Vec<_>>()
            .join(" ");
        if !query.is_empty() {
            return Some(VoiceAction::LaunchApp { query });
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn action(s: &str) -> Option<VoiceAction> {
        parse_utterance(s).action
    }

    #[test]
    fn normalise_strips_accents_and_punct() {
        assert_eq!(normalise("  Éteins  la Télé !! "), "eteins la tele");
    }

    #[test]
    fn wake_word_detected_at_start() {
        let p = parse_utterance("OK hub allume la TV");
        assert!(p.wake);
        assert_eq!(p.command.as_deref(), Some("allume la tv"));
    }

    #[test]
    fn wake_word_detected_mid_phrase() {
        assert!(parse_utterance("dis hub eteins la tv").wake);
    }

    #[test]
    fn no_wake_word() {
        assert!(!parse_utterance("allume la tv").wake);
    }

    #[test]
    fn tv_on_off() {
        assert_eq!(
            action("ok hub allume la television"),
            Some(VoiceAction::Cec {
                action: "tv_on".into()
            })
        );
        assert_eq!(
            action("ok hub éteins la télé"),
            Some(VoiceAction::Cec {
                action: "tv_off".into()
            })
        );
    }

    #[test]
    fn ps4() {
        assert_eq!(
            action("ok hub démarre la ps4"),
            Some(VoiceAction::Cec {
                action: "ps4_on".into()
            })
        );
    }

    #[test]
    fn hub_relay() {
        assert_eq!(
            action("ok hub allume le hub"),
            Some(VoiceAction::Relay { relay: 1, on: true })
        );
        assert_eq!(
            action("ok hub coupe le concentrateur"),
            Some(VoiceAction::Relay {
                relay: 1,
                on: false
            })
        );
    }

    #[test]
    fn launch_app() {
        assert_eq!(
            action("ok hub lance netflix"),
            Some(VoiceAction::LaunchApp {
                query: "netflix".into()
            })
        );
        assert_eq!(
            action("ok hub ouvre le navigateur"),
            Some(VoiceAction::LaunchApp {
                query: "navigateur".into()
            })
        );
    }

    #[test]
    fn unknown_command() {
        assert_eq!(action("ok hub fais moi un cafe"), None);
    }
}
