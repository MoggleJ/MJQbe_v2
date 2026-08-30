# Sprint 5 — App Native : Mode Dev

**Terminé le** 2026-08-30 (~23:48 CEST) · **Branche** `sprint-05-actions`

---

## 1. Ce qui a été fait

### 1.1 `native/core` — `infrastructure/system/` + Dev use cases

| Fichier | Rôle |
|---|---|
| `infrastructure/system/metrics.rs` | `snapshot()` : CPU % (`/proc/stat`, fenêtre 200 ms), RAM/swap (`/proc/meminfo`), disque (`libc::statvfs`), réseau (débit `/proc/net/dev` hors `lo`), température (`/sys/class/thermal/thermal_zone*/temp`), load, uptime |
| `infrastructure/system/processes.rs` | `list_processes` (tri RSS, `/proc/<pid>/{status,stat}`), `terminate` (`libc::kill`), `renice` (`libc::setpriority`) — refus `pid ≤ 1`, mapping EPERM/EACCES→`PermissionDenied`, ESRCH→`NotFound` |
| `infrastructure/system/docker.rs` | `DockerCli` : `list` (`docker ps -a --format`), `start` / `stop` — **validation d'id** (alphanum + `_.-` seulement, anti-injection) |
| `application/dev.rs` | `DevService` : snapshot / list_processes / kill / renice / list_containers / start / stop |
| `application/auth.rs` | `AuthService` : store de **tokens de ré-auth** (`verify(password) → (token, ttl 120 s)`, `check_token` **usage unique**) |
| domain | `SystemSnapshot`, `ProcessInfo`, `DockerContainer` ; `CoreError::{ReauthRequired, PermissionDenied}` |
| interface/ipc | `auth.verify`, `system.snapshot`, `process.list` (ouverts) ; `process.kill`, `process.nice`, `docker.start`, `docker.stop` (token obligatoire) ; `docker.list` (ouvert) |

Deps ajoutées : `libc`, `rand` ; feature tokio `process`.
**Tests** : 37 (36 unit + 1 intégration). `cargo clippy -D warnings` clean.

### 1.2 `native/ui`

- **`src/TerminalController.{h,cpp}`** : `QProcess` → `bash -i`, `MergedChannels`, signal `output(chunk)`, `send(line)`, `start`/`stop`. Local à l'UI (pas via le core).
- **`src/NativeBridge`** : `verify`, `systemSnapshot`, `listProcesses`, `killProcess`, `niceProcess`, `listContainers`, `dockerStart`, `dockerStop` + signaux `verifyResult`, `snapshotReceived`, `processesReceived`, `containersReceived`, `devActionResult`.
- **`src/main.cpp`** : `Terminal` exposé en context property.
- **`qml/Gauge.qml`** : barre 0–100 % (couleur seuils 70/90).
- **`qml/pages/Dev.qml`** :
  - **gate** : mot de passe admin → `Bridge.verify` → `unlocked`.
  - **monitoring** : `Timer` 2 s → `system.snapshot` + `process.list` + `docker.list` ; 5 `Gauge` (CPU, RAM, disque, température, réseau).
  - **process list** : `ListView` (pid/nom/état/RSS/nice) + boutons `kill` / `nice +5`.
  - **Docker** : `ListView` (nom/image/état) + `start` / `stop`.
  - **terminal** : `TextArea` (append `Terminal.output`) + `TextField` → `Terminal.send`.
  - **Interface graphique Pi** : bouton → `Qt.openUrlExternally("vnc://localhost:5900")`.
  - **ré-auth** : `Dialog` modal ; toute action destructive passe par `requireReauth(fn)` → `Bridge.verify` → `verifyResult` fournit le token → `fn(token)`.
- **`qml/Sidebar.qml`** : entrée « 🔒 MJ Dev ».
- **`qml/Main.qml`** : page `Dev` dans le `StackView` ; bascule `window.mode = "dev"` (restaure le mode précédent en sortie).

---

## 2. Vérifications

| Vérif | Résultat |
|---|---|
| `cargo test` | ✅ 37/37, clippy `-D warnings` clean |
| `cmake --build native/ui` | ✅ (Gauge, Dev, TerminalController inclus) |
| E2E core (probe) | ✅ `system.snapshot` : CPU 5 %, RAM 5366/31950 Mo, disque 207/478 Go, **temp 46 °C**, net 430/570 o/s ; `process.list` tri RSS réel ; `docker.list` 14 conteneurs ; `process.kill` sans token → `reauth_required` ; `auth.verify` → token (ttl 120) ; token **rejeté à la 2ᵉ utilisation** ; `docker.stop` id `"bad;rm"` → `internal / invalid container id` |
| `smoketest.sh` (Docker offscreen) | ✅ « VERDICT: OK — QML tree loaded » |

---

## 3. Reste à faire sur cet élément

### Différé (matériel / portée)
- Vérif **sur Pi** : thermal zone Pi (`/sys/.../thermal_zone0`), `kill`/`nice` sur de vrais process, `docker start/stop` réels, lancement au boot via systemd — issue #137.
- **Terminal** : actuellement ligne-à-ligne (`bash -i` en pipe, pas de PTY, pas d'ANSI/couleurs, pas de `Ctrl-C`). Un vrai PTY (`forkpty`) + rendu ANSI serait mieux — **nouvelle issue**.
- **Interface graphique Pi** : URL `vnc://localhost:5900` en dur — à rendre configurable (`config.yml` / settings) — **nouvelle issue**.
- Gauges = barres simples ; le CDC parle de « widgets » — anneaux/graphes possibles au Sprint 6 (UX).
- `docker` doit être dans le `PATH` du service et l'utilisateur `mjqbe` dans le groupe `docker` (ou socket accessible) — à documenter (`docs/deploiement.md`, Sprint 17).

### Sécurité
- Tokens de ré-auth : en mémoire, usage unique, TTL 120 s, purgés à chaque `verify`/`check`. Pas de rate-limiting sur `auth.verify` côté natif (local, mono-siège) — acceptable ; à revoir si l'IPC devient multi-client.

---

## 4. Comment ça fonctionne

`Sidebar` « 🔒 MJ Dev » → `Main.go("Dev")` (mode → `dev`) → `Dev.qml` verrouillé.
Saisie mot de passe → `Bridge.verify(pw)` → core `auth.verify` (bcrypt) → `verifyResult(true, token)` → `unlocked = true` (token jeté).
`Timer` 2 s : `system.snapshot` (le core lit `/proc` + `sysfs`, échantillonne CPU/net sur 200 ms) → `snapshotReceived` → binding des `Gauge` ; idem `process.list`, `docker.list`.
Action destructive (kill / nice / docker start-stop) → `requireReauth(fn)` ouvre le `Dialog` → `Bridge.verify(pw)` → `verifyResult` porte un **nouveau** token → `fn(token)` envoie p. ex. `process.kill {token, pid}` → le core `check_token` (consomme) puis `libc::kill`.
Terminal : `TerminalController` lance `bash -i` à l'ouverture de la page ; `TextField` → `Terminal.send("cmd")` → `process.write("cmd\n")` ; sortie fusionnée → signal `output` → `TextArea.insert`.
