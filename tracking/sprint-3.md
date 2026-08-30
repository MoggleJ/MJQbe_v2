# Sprint 3 — App Native : Scaffolding C++/Qt6 + Rust

**Terminé le** 2026-08-30 (~23:15 CEST) · **Branche** `sprint-03-actions` · **Priorité** native-first

---

## 1. Ce qui a été fait

### 1.1 `native/core/` — Rust (logique + données + IPC)

Crate `mjqbe-core` (lib + bin), Clean Architecture stricte :

| Couche | Fichiers | Rôle |
|---|---|---|
| **domain** | `domain/entities.rs`, `error.rs`, `repository.rs` | `App`, `Category`, `AdminRecord` ; `CoreError` ; traits `CatalogRepository`, `AuthRepository` (async, `async-trait`). Zéro I/O. |
| **application** | `application/catalog.rs`, `auth.rs`, `mod.rs` | `CatalogService` (list_apps / list_categories + `validate_mode`), `AuthService` (login bcrypt). Repo `Option<Arc<dyn …>>` → `DbUnavailable` si pas de base. |
| **infrastructure** | `infrastructure/db/{mod,catalog_repo,auth_repo}.rs` | `Db` (PgPool sqlx), requêtes **runtime** (`sqlx::query(...).bind(...)`, pas de macro → CI sans base). |
| | `infrastructure/hardware/mod.rs` | `Platform::detect()` (`MJQBE_STUB=1` ou `/proc/device-tree/model`), façade `Gpio` → `HardwareUnavailable` hors-Pi. |
| **interface** | `interface/ipc/{mod,protocol,handler}.rs` | Serveur `UnixListener` (tokio `net`), JSON lignes, routage. Nettoyage socket au SIGINT/SIGTERM. |
| entrée | `config.rs`, `lib.rs`, `main.rs` | Config via env ; `main` = bootstrap (DB optionnelle → mode dégradé si down). |

**Protocole IPC** (`/run/mjqbe/native.sock` ou `$MJQBE_NATIVE_SOCKET`) — 1 objet JSON / ligne :

| method | params | data (succès) |
|---|---|---|
| `ping` | — | `{ "pong": true }` |
| `health` | — | `{ "platform", "version" }` |
| `apps.list` | `{ "mode", "category_id"? }` | `[App]` |
| `categories.list` | `{ "mode" }` | `[Category]` |
| `auth.login` | `{ "username", "password" }` | `{ "user_id", "username", "role" }` |

Erreur : `{ "id", "ok": false, "error": { "code", "message" } }` — codes : `db_unavailable`, `invalid_credentials`, `hardware_unavailable`, `db_error`, `internal`, `bad_request`.

**Tests** : 18 (17 unit + 1 intégration socket `tests/ipc_roundtrip.rs`), **aucune base requise**. `cargo fmt` clean, `cargo clippy -D warnings` clean.

### 1.2 `native/ui/` — C++ + Qt6/QML

- `CMakeLists.txt` : `qt_add_qml_module` (URI `MJQbe`, `RESOURCE_PREFIX /qt/qml`), `ThemeManager` en singleton.
- `src/main.cpp` : `QGuiApplication` + `QQmlApplicationEngine`, options `--windowed` / `--socket`, `Bridge` en context property.
- `src/NativeBridge.{h,cpp}` : client `QLocalSocket`, JSON lignes, reconnexion auto (2 s), corrélation par `id`, signaux `appsReceived` / `categoriesReceived` / `loginResult` / `coreError`. Si le core est absent → `connected == false`, l'UI tourne en mode dégradé.
- QML : `Main.qml` (ApplicationWindow, Row = Sidebar 250px + StackView), `Sidebar.qml` (titre dynamique MJ TV/Desktop/Dev, menu Home/All Apps/Search, switch de mode, Settings + `Clock` en bas), `ThemeManager.qml` (10 thèmes, amoled défaut), `SidebarButton`, `AppCard`, pages `Home` / `AllApps` (GridView 106×120) / `Search` (filtre live) / `Settings` (sélecteur 10 thèmes) / `Login` (auth admin).
- Navigation : `StackView.replace` via `window.go(page)`.

### 1.3 systemd

- `native/mjqbe-core.service` — serveur IPC, `RuntimeDirectory=mjqbe`, `EnvironmentFile=-/etc/mjqbe/core.env`.
- `native/mjqbe-native.service` — UI, `After=mjqbe-core.service`, attend le socket, `QT_QPA_PLATFORM=wayland;xcb`.

### 1.4 Hors `native/`

- `docker-compose.native.yml` — publie `db` sur `127.0.0.1:15432` (dev natif hors-Pi).
- `docs/CDC.md §2.1` — réécrit PySide6 → C++/Qt6/QML + Rust (incohérence spec corrigée).
- `.github/workflows/native-build.yml` — paquets Qt élargis (`qt6-declarative-dev-tools`, `libgl1-mesa-dev`, `ninja-build`).

---

## 2. Vérifications effectuées

| Vérif | Méthode | Résultat |
|---|---|---|
| `cargo build` / `cargo test` | local (Rust 1.98) | ✅ 18/18, 0 warning clippy |
| `cmake --build native/ui` | local (Qt 6.4.2) | ✅ binaire `mjqbe-native`, qmlcachegen OK sur tous les .qml |
| Core ↔ PostgreSQL (seed) | core lancé contre `db` Docker via `:15432`, sonde Python | ✅ `apps.list tv` = 7 apps, `categories.list desktop` = 3, `auth.login admin/admin` → role `admin`, mauvais mdp → `invalid_credentials`, mode invalide → `internal` |
| UI charge l'arbre QML | image Docker `debian:bookworm` (Qt 6.4 = base Pi), `QT_QPA_PLATFORM=offscreen`, socket core bind-monté | ✅ « UI still alive after 5s — QML tree loaded », aucune erreur QML |

---

## 3. Ce qui reste à faire sur cet élément

### Différé faute de matériel (pas de Pi)
- Vérification **sur écran réel / Pi** : fenêtre plein écran, rendu des thèmes, navigation télécommande.
- `Platform::RaspberryPi` : jamais exercé (toujours `Stub` hors-Pi).
- Installation des `qml6-module-qtquick-*` sur le Pi → à documenter dans `docs/deploiement.md` (Sprint 17).

### Reporté aux sprints suivants (par conception)
- **Sprint 4** : `AppCard` avec vraies icônes + favoris, `Home` (récents/favoris), `KeyNavigation` complète, `Settings` (layout + icon_size + persistance), ouverture apps `QWebEngineView`.
- **Sprint 5** : `Dev.qml` + gate re-auth, `infrastructure/system/` (CPU/RAM/`/proc`), liste process, Docker, terminal `QProcess`.
- **Sprint 7** : câbler la façade `Gpio` au daemon C (`/run/mjqbe/daemon.sock`).
- Réorganisation QML en `components/` + `modes/{tv,desktop,dev}/` (CDC §6.3) — à plat pour l'instant.

### Dette / à surveiller
- `sqlx` sans feature TLS → `sslmode=disable` (OK en LAN Pi ; ajouter `tls-rustls` si besoin).
- `native-build.yml` : le job `cpp-check` **build** seulement (pas de run) — pas de smoke-test QML en CI. Envisager un job Docker offscreen.

---

## 4. Comment ça fonctionne (démarrage dev, hors-Pi)

```bash
# 1. PostgreSQL joignable sur l'hôte
docker compose -f docker-compose.yml -f docker-compose.native.yml up -d db   # → 127.0.0.1:15432

# 2. Core (serveur IPC), mode stub
cd native/core
MJQBE_STUB=1 MJQBE_NATIVE_SOCKET=/tmp/mjqbe-native.sock \
DATABASE_URL='postgres://mjqbe:<pw>@localhost:15432/mjqbe?sslmode=disable' \
cargo run

# 3. UI (autre terminal)
cd native/ui && cmake -S . -B build && cmake --build build
MJQBE_NATIVE_SOCKET=/tmp/mjqbe-native.sock ./build/mjqbe-native --windowed
```

Flux : QML → `Bridge.listApps("tv")` → `NativeBridge` écrit `{"id","method":"apps.list","params":{"mode":"tv"}}\n` sur le socket → `mjqbe-core` route vers `CatalogService` → `PgCatalogRepository` (sqlx) → réponse JSON → `NativeBridge` émet `appsReceived("tv", [...])` → `AllApps.qml` remplit son `ListModel` → `GridView`.
Sans le core : `NativeBridge` reste `connected=false`, retries toutes les 2 s, l'UI affiche « core: offline ».

Restauration après tests : `docker compose -f docker-compose.yml -f docker-compose.prod.yml up -d db`.
