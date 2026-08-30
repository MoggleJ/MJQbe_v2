# Sprint 9 — Reconnaissance vocale

**Terminé le** 2026-08-31 (~01:05 CEST) · **Branche** `sprint-09-actions`
**Issues fermées** : #54, #55, #56, #58 · **rescoping** : #53 + #57 → nouvelle issue (capture audio Vosk réelle)

---

## 1. Ce qui a été fait

### `native/core`

| Fichier | Rôle |
|---|---|
| `infrastructure/voice/grammar.rs` | `normalise` (minuscule + accents FR + ponctuation), `parse_utterance` (wake word en tête **ou** en milieu de phrase, ≤ 2 mots avant), `map_command` → `VoiceAction`. **13 tests**. |
| `infrastructure/voice/mod.rs` | `engine_name()` (`stub` / `vosk`) |
| `infrastructure/voice/vosk_engine.rs` | recogniser réel — **`#![cfg(feature = "vosk")]`**, non compilé par défaut ; squelette `spawn(Sender<String>)` (Model + Recognizer 16 kHz). Cargo : `[features] vosk = []`. |
| `application/voice.rs` | `VoiceService` : `interpret` (enregistre l'horodatage du wake), `set_enabled` / `is_enabled` (`AtomicBool`), `seconds_since_wake`, `engine` |
| `domain/entities.rs` | `VoiceAction` (`Cec{action}` / `Relay{relay,on}` / `LaunchApp{query}`, serde `tag="kind"`) + `ParsedUtterance` |
| `CatalogRepository::search_apps` + `CatalogService::find_app` | recherche floue `name ILIKE '%q%'` (tv+desktop) pour « lance X » |
| `interface/ipc/handler.rs` | `voice.status` / `voice.simulate` (ouverts) ; `voice.set_enabled` (token) ; `Handler::run_voice_action` (dispatch Cec/Relay/LaunchApp, respecte `is_enabled`) |

**53 tests** (52 unit + 1 intégration), `cargo clippy -D warnings` clean.

### `native/ui`

- `NativeBridge` : `voiceStatus` / `voiceSimulate` / `voiceSetEnabled` + signaux `voiceStatusReceived` / `voiceResult`.
- `Main.qml` : `Timer` 2 s toujours actif → `voice.status` ; propriété `voice` transmise à la `Sidebar`.
- `Sidebar.qml` : **indicateur** — point à côté du titre (visible si voix activée), pulse (`SequentialAnimation on scale`) quand `last_wake_secs ≤ 3`.
- `Dev.qml` : ligne statut voix + `TextField` phrase + bouton « Simuler » (→ `voice.simulate`, affiche `wake/action/result`) + bouton Activer/Désactiver (ré-auth).

### Daemon

- `av.c` : `cec_send` enveloppé dans `timeout 6 sh -c "…"` (cec-client peut se bloquer sans adaptateur).
- `DaemonClient` (Rust) : timeout 3 s → **8 s** (CEC peut prendre quelques secondes sur matériel réel).

---

## 2. Vérifications (Docker, daemon stub)

| Phrase / appel | Résultat |
|---|---|
| `voice.status` | `{enabled:true, engine:"stub", last_wake_secs:null}` |
| `"ok hub allume la télé"` | wake ✓, action `cec/tv_on` → daemon (`cec:tv_on`, ~0,5 s après le fix timeout) |
| `"ok hub allume le hub"` | action `relay{1,true}` → `relay:1=1` |
| `"ok hub lance netflix"` | action `launch_app{netflix}` → `launch:https://netflix.com` (résolu en base seed) |
| `"ok hub lance un truc inexistant"` | `launch_unresolved:truc inexistant` |
| `"allume la tv"` (sans wake) | `wake:false`, action quand même mappée |
| `voice.set_enabled` sans token | `reauth_required` |
| `voice.set_enabled` +token `false` puis `voice.simulate` | `voice_disabled` |
| `cargo test` | 53/53, clippy clean |
| UI smoke Docker offscreen | QML tree loaded (VmRSS ~50 Mo) |

---

## 3. Reste à faire

### Différé — **nouvelle issue** (capture audio Vosk réelle)
- `vosk_engine.rs` : boucle de capture `cpal` (16 kHz mono) → `Recognizer::accept_waveform` → `voice.simulate`-équivalent en interne.
- Ajouter les deps optionnelles `vosk` + `cpal` sous la feature `vosk`.
- Déploiement Pi : `libvosk.so` + modèle `vosk-model-small-fr-*` dans `/opt/mjqbe/vosk-model` (`MJQBE_VOSK_MODEL`).
- **ISD1820** : entrée GPIO de déclenchement (le daemon surveille une broche → notifie le core d'écouter) — ou micro USB en écoute continue.
- Test « OK hub allume la TV » à voix réelle sur Pi — issue #137.

### Dette
- `cec_send` : le flag `sent` n'est pas fiable **sans adaptateur** (`timeout`+pipe renvoie parfois 0). À valider sur Pi. Voir aussi la nouvelle issue « CEC async dans le daemon » (le `system()` bloque l'accept-loop du daemon jusqu'à 6 s).
- `run_voice_action` `LaunchApp` renvoie l'URL en `result` mais **n'ouvre rien** — sur le Pi, le core devra pousser un événement au client UI pour qu'il fasse `Qt.openUrlExternally`. Canal core→UI non prévu par le CDC — à concevoir (nouvelle issue possible).

---

## 4. Comment ça fonctionne

**Cible (Pi)** : micro → `vosk_engine` (feature `vosk`) → transcript texte → `VoiceService::interpret` → `parse_utterance` (wake + `map_command`) → si `wake` : `run_voice_action` → `HardwareService` (CEC/relais) ou `CatalogService::find_app`. L'`Sidebar` pulse via `voice.status` (`last_wake_secs`).

**Dev / test (sans micro)** : `Dev.qml` → `voice.simulate "ok hub allume la télé"` → `Handler` → même chemin `interpret` + `run_voice_action` → JSON `{wake, command, action, result}` renvoyé et affiché.
