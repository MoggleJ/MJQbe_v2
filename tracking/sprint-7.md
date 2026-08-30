# Sprint 7 — Daemon C : GPIO

**Terminé le** 2026-08-31 (~00:20 CEST) · **Branche** `sprint-07-actions`
**Issues fermées** : #42, #43, #44, #45, #46

---

## 1. Ce qui a été fait

### 1.1 `daemon/` — mjqbe-daemon (C)

- **`daemon/main.c`** réécrit : serveur socket Unix, **un objet JSON par ligne** (cohérent avec l'IPC natif), parsing **cJSON** (`libcjson`, déjà dans le Dockerfile).
- Commandes : `ping`, `info`, `gpio_set {pin,value}`, `gpio_get {pin}`, `relay_set {relay,state}`, `led_set {r,g,b}`.
- **GPIO** : backend sysfs (`/sys/class/gpio`), export + direction + value, cache implicite via `stat`.
- **Détection Pi robuste** (`should_stub()`) : `MJQBE_GPIO_STUB=1` → stub ; `MJQBE_GPIO_FORCE=1` → réel ; sinon réel **seulement si** `/proc/device-tree/model` (ou `/sys/firmware/devicetree/base/model`) contient « raspberry pi ». (`/sys/class/gpio` existe sur tout noyau Linux → insuffisant, cf. bug corrigé en cours de sprint.)
- Relais 1–4 → GPIO **23 / 24 / 25 / 12**, **actif-bas** par défaut (`MJQBE_RELAY_ACTIVE_HIGH=1` pour inverser). LED RGB → GPIO 5/6/13 (`MJQBE_LED_R/_G/_B`).
- Socket : `DAEMON_SOCKET` (défaut `/run/mjqbe/daemon.sock`), `chmod 0660`.
- **`daemon/README.md`** : protocole + câblage + build/test.

### 1.2 `native/core` — client + use case + IPC

- **`infrastructure/hardware/daemon_client.rs`** : `DaemonClient` (tokio `UnixStream`, timeout 3 s, 1 connexion/req). Socket absent → `HardwareUnavailable`.
- **`application/hardware.rs`** : `HardwareService` (info / gpio_set / gpio_get / relay_set (valide 1–4) / led_set).
- **IPC** : `hardware.info`, `gpio.get` (ouverts) ; `gpio.set`, `relay.set`, `led.set` (**token de ré-auth** obligatoire, comme les actions Dev).
- L'ancien stub `Gpio` supprimé. **39 tests** (38 unit + 1 intégration), clippy `-D warnings` clean.

### 1.3 `api/` — client Python + endpoints

- **`api/app/infrastructure/hardware/daemon_client.py`** : même protocole (socket AF_UNIX, JSON/ligne). Exceptions `HardwareUnavailable` / `DaemonError`.
- **`api/app/interface/routes/dev.py`** : router `/dev` — `POST /dev/gpio`, `POST /dev/relay`, `POST /dev/led`, `GET /dev/hardware`, `GET /dev/gpio/{pin}`. Schémas Pydantic (`pin` 0–53, `relay` 1–4, `state`/`value` 0–1). `HardwareUnavailable` → 503, `DaemonError` → 400.
- `main.py` : `include_router`. ⚠️ Non protégé pour l'instant — passera derrière le guard JWT admin au **Sprint 10** (issue #67).

### 1.4 `cli/dev`

- `dev gpio <pin> <0|1>` et `dev relay <1-4> <0|1>` → `curl POST` vers l'API (CLI → API → daemon, jamais d'accès direct).

---

## 2. Vérifications (toutes via Docker, daemon en **stub**)

| Vérif | Résultat |
|---|---|
| `docker build ./daemon` | ✅ (1 warning `strncpy` corrigé → `snprintf`) |
| daemon direct (socket bind-monté) | ✅ `ping`, `info` (`backend:stub`), `gpio_set/get`, `relay_set` (relay 1→pin 23), `led_set`, pin 99 → `bad pin/value`, cmd inconnue → `unknown cmd` |
| API : `GET /dev/hardware` | ✅ `{"backend":"stub","pi":false,"relays":4}` |
| API : `POST /dev/gpio {23,1}` | ✅ `{"pin":23,"value":1}` |
| API : `POST /dev/relay {2,1}` | ✅ `{"relay":2,"state":1,"pin":24}` |
| API : `POST /dev/led {255,0,10}` | ✅ `{"r":1,"g":0,"b":1}` (normalisé on/off) |
| API : pin 99 / relay 9 | ✅ 422 (Pydantic) |
| `cli/dev gpio 17 1` | ✅ `{"pin":17,"value":1}` |
| `cli/dev relay 3 0` | ✅ `{"relay":3,"state":0,"pin":25}` |
| Rust core → daemon : `gpio.set` sans token | ✅ `reauth_required` |
| Rust core → daemon : `gpio.set` + token | ✅ `{"pin":23,"value":1}` |
| Rust core → daemon : `relay.set` + token | ✅ `{"relay":1,"state":1,"pin":23}` |
| `cargo test` | ✅ 39/39, clippy clean |
| CI api-ci / docker-build / native-build | ✅ verts (fix P12) |

---

## 3. Reste à faire sur cet élément

### Différé (pas de Pi)
- **GPIO réel** : export sysfs, niveaux relais, LED — jamais exercé (stub partout hors-Pi) — issue #137.
- `gpio_get` en stub renvoie toujours `0` (pas de persistance d'état) — normal ; sur Pi il lira la vraie valeur.
- Permissions : sur le Pi, l'utilisateur du daemon doit être dans `gpio` (ou le daemon root) ; socket `0660` → clients dans le même groupe. À documenter (`docs/deploiement.md`, Sprint 17).

### Dépendances de sprints suivants
- **Protection `/dev/*`** : les endpoints sont ouverts jusqu'au Sprint 10 (JWT admin) — issue #67.
- Sprint 8 (IR/CEC/BT) et Sprint 9 (voix) étendront le même daemon + le même schéma d'endpoints.

---

## 4. Comment ça fonctionne

`cli/dev gpio 23 1` → `POST http://localhost:4848/dev/gpio {"pin":23,"value":1}` → route FastAPI → `daemon_client.gpio_set` ouvre `DAEMON_SOCKET`, écrit `{"cmd":"gpio_set","pin":23,"value":1}\n` → `mjqbe-daemon` : `handle()` → `gpio_write(23, 1)` (sysfs `export` → `direction=out` → `value=1`, ou no-op en stub) → `{"ok":true,"data":{"pin":23,"value":1}}\n` → réponse HTTP.

Côté natif : `Dev.qml` (Sprint 8+) → IPC `gpio.set {token,pin,value}` → `Handler` : `auth.check_token` (consomme le token de `auth.verify`) → `HardwareService` → `DaemonClient` → même daemon, même socket que l'API (volume Docker `daemon-socket`, ou `/run/mjqbe/` sur le Pi).
