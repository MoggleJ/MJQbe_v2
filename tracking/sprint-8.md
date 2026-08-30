# Sprint 8 — Daemon C : AV (IR + CEC + Bluetooth)

**Terminé le** 2026-08-31 (~00:45 CEST) · **Branche** `sprint-08-actions`
**Issues fermées** : #47, #48, #49, #50, #51, #52

---

## 1. Ce qui a été fait

### 1.1 `daemon/av.{c,h}` (+ `ir-map.json`)

| Sous-système | Implémentation | Hors-Pi |
|---|---|---|
| **CEC** | `av_cec(action)` → `echo '<seq>' \| cec-client -s -d 1` (paquet `cec-utils`). `tv_on`=`on 0`, `tv_off`=`standby 0`, `tv_toggle`=`pow 0`, `ps4_on/off`=`tx …` | `cec-client` présent → `cec:true` mais `sent:false` sans adaptateur |
| **IR** | thread : connexion socket LIRC (`LIRC_SOCKET`, défaut `/var/run/lirc/lircd`), parse `code repeat button remote`, agit sur `repeat==00`, mappe via `ir-map.json` | pas de socket → `ir:false`, thread se termine |
| **BT** | thread : `open(BT_SERIAL)` (défaut `/dev/serial0`), `cfmakeraw` 9600, lignes → `bt_action_for` (`TV_ON`→`tv_on`, `HUB_ON`→`hub_on`…) | pas de device → `bt:false` |

- **`dispatch_action(action)`** : `tv_*`/`ps4_*` → `av_cec` ; `hub_on`/`hub_off` → `daemon_relay_set(1, …)` (extrait de `main.c`, appelé aussi depuis `av.c`) ; `nav_*` → `noop` (cible à câbler plus tard).
- **`daemon/ir-map.json`** copié dans l'image (`/etc/mjqbe/ir-map.json`) ; fallback intégré si absent. Surcharge : `MJQBE_IR_MAP`.
- Threads `pthread` détachés, statut protégé par mutex.

### 1.2 `daemon/main.c` — nouvelles commandes

`cec_send {action}`, `av_status`, `ir_map`, `ir_inject {name}`, `bt_inject {line}`.
`av_init()` appelé au démarrage. Makefile : `av.c` + `-lpthread`. Dockerfile : `+ cec-utils`, `COPY ir-map.json`.

### 1.3 Clients + endpoint + UI

- **Rust** : `DaemonClient::{cec_send, av_status}` ; `HardwareService::{av_cec (valide l'action), av_status}` ; IPC `av.status` (ouvert), `av.send {token, action}` (token de ré-auth). 40 tests, clippy clean.
- **Python** : `daemon_client.{av_status, av_cec}` ; routes `GET /dev/av`, `POST /dev/av {action}` (Pydantic `Literal[tv_on,tv_off,tv_toggle,ps4_on,ps4_off]`).
- **`Dev.qml`** : ligne statut `cec/ir/bt` + `Flow` de 4 boutons (Allume/Éteins TV, PS4 on/off) → `requireReauth(t => Bridge.avSend(t, action))` ; poll `av.status` dans le `Timer` 2 s. `NativeBridge` : `avStatus`, `avSend` + signal `avStatusReceived`.

---

## 2. Vérifications (Docker, daemon **stub**)

| Vérif | Résultat |
|---|---|
| `docker build ./daemon` (av.c + pthread) | ✅ 0 warning |
| daemon direct | ✅ `av_status` `{cec:true,ir:false,bt:false}` ; `ir_map` chargé depuis `/etc/mjqbe/ir-map.json` ; `cec_send bogus` → `unknown cec action` ; `ir_inject KEY_POWER` → `hub_on` (résultat `hub_on`) ; `ir_inject KEY_UP` → `nav_up`/`noop` ; `ir_inject KEY_NOPE` → non géré ; `bt_inject TV_ON` → `tv_on`/`cec_unavailable` ; `bt_inject HUB_ON` → `hub_on` ; `bt_inject garbage` → non géré |
| API `GET /dev/av` | ✅ `{"cec":true,"ir":false,"bt":false}` |
| API `POST /dev/av tv_on` | ✅ `{"action":"tv_on","sent":false,"error":"cec-client failed"}` (pas d'adaptateur — attendu) |
| API `POST /dev/av explode` | ✅ 422 |
| Rust core `av.send` sans token / +token / action invalide | ✅ `reauth_required` / `cec-client failed` / `invalid AV action` |
| `cargo test` | ✅ 40/40, clippy clean |
| UI smoke Docker offscreen | ✅ QML tree loaded (VmRSS ~50 Mo) |

---

## 3. Reste à faire sur cet élément

### Différé (pas de Pi) — issue #137
- IR réel : brancher un récepteur + LIRC configuré, valider le parsing et la carte.
- CEC réel : adaptateur HDMI-CEC (Pi le fait nativement), valider `on 0` / `standby 0` / séquences PS4.
- BT réel : module HC-05 sur l'UART, appairage, débit, parsing.
- `nav_*` : actions non câblées à une cible (navigation UI native) — à connecter (probablement via un canal daemon → core, non prévu par le CDC pour l'instant).

### Choix assumés
- **CEC via `cec-client`** (sous-processus) plutôt que **libCEC** (linkage C) : évite une grosse dépendance de build, `cec-client` est le chemin standard sur Pi. Passage à libCEC possible plus tard si latence/robustesse insuffisantes — **peut faire l'objet d'une issue**.
- Séquences PS4 (`tx 4F:82:10:00`, `tx 4F:36`) sont des approximations CEC à valider sur matériel réel.

---

## 4. Comment ça fonctionne

**Bouton « Allume TV » (natif)** : `Dev.qml` → `requireReauth` (dialog mot de passe) → `Bridge.verify` → `verifyResult(token)` → `Bridge.avSend(token, "tv_on")` → IPC `av.send` → `Handler` : `check_token` → `HardwareService::av_cec` (valide l'action) → `DaemonClient::cec_send` → daemon `cec_send` → `av_cec` → `cec-client` → TV.

**Télécommande IR** : `ir_thread` lit une ligne LIRC → `KEY_POWER` → `ir_action_for` → `"hub_on"` → `dispatch_action` → `daemon_relay_set(1, 1)` → le relais alimente le hub.

**Bluetooth** : `bt_thread` lit `HUB_ON\n` sur `/dev/serial0` → `"hub_on"` → même dispatch.

**Web** : `POST /dev/av {"action":"tv_on"}` → route FastAPI → `daemon_client.av_cec` → daemon (même socket, volume Docker).
