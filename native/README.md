# native/ — Application native MJQbe

App locale du Pi (modes TV / Desktop / Dev). **Pas de Docker** — processus systemd.

```
native/
  core/                 Rust — logique métier + accès données + serveur IPC
    src/
      domain/           entités + traits repository (aucune I/O)
      application/       cas d'usage (catalog, auth)
      infrastructure/
        db/             adaptateurs PostgreSQL (sqlx, requêtes runtime)
        hardware/        détection plateforme + façade GPIO (stub hors-Pi)
      interface/ipc/     serveur socket Unix, protocole JSON lignes
  ui/                   C++ + Qt6/QML — interface
    src/                main.cpp + NativeBridge (client QLocalSocket)
    qml/                Main, Sidebar, ThemeManager (singleton, 10 thèmes)…
  mjqbe-core.service    unit systemd (core)
  mjqbe-native.service  unit systemd (UI, After=mjqbe-core)
```

## Protocole IPC

Socket Unix `MJQBE_NATIVE_SOCKET` (défaut `/run/mjqbe/native.sock`).
Un objet JSON par ligne, dans les deux sens.

| Méthode | Params | Réponse `data` |
|---|---|---|
| `ping` | — | `{ "pong": true }` |
| `health` | — | `{ "platform": "stub\|raspberry-pi", "version": "0.1.0" }` |
| `apps.list` | `{ "mode": "tv\|desktop\|dev", "category_id"?: int }` | `[App]` |
| `categories.list` | `{ "mode": "…" }` | `[Category]` |
| `auth.login` | `{ "username": "…", "password": "…" }` | `{ "user_id", "username", "role" }` |

Erreur : `{ "id", "ok": false, "error": { "code", "message" } }`
(`db_unavailable`, `invalid_credentials`, `hardware_unavailable`, `db_error`, `internal`, `bad_request`).

## Build & run (dev, hors-Pi)

Prérequis : Rust (rustup), CMake ≥ 3.21, Qt6 ≥ 6.4 (`qt6-base-dev qt6-declarative-dev qt6-declarative-dev-tools`).

```bash
# 1. PostgreSQL accessible sur l'hôte (port 15432 publié via l'override natif)
docker compose -f docker-compose.yml -f docker-compose.native.yml up -d db

# 2. Core (serveur IPC) — mode stub hors-Pi
#    Les identifiants viennent de .env (jamais en clair ici).
cd native/core
set -a && . ../../.env && set +a
MJQBE_STUB=1 \
MJQBE_NATIVE_SOCKET=/tmp/mjqbe-native.sock \
DATABASE_URL="postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@localhost:15432/${POSTGRES_DB}?sslmode=disable" \
cargo run

# 3. UI (autre terminal)
cd native/ui
cmake -S . -B build -DCMAKE_BUILD_TYPE=Debug
cmake --build build --parallel
MJQBE_NATIVE_SOCKET=/tmp/mjqbe-native.sock ./build/mjqbe-native --windowed
```

Sans le core, l'UI démarre quand même (bandeau « core: offline », listes vides).

## Tests

```bash
cd native/core && cargo test        # 18 tests (unit + IPC round-trip), aucune DB requise
```

## Déploiement Pi (aperçu — Sprint 6/17)

```bash
install -Dm755 native/core/target/release/mjqbe-core   /opt/mjqbe/bin/mjqbe-core
install -Dm755 native/ui/build/mjqbe-native            /opt/mjqbe/bin/mjqbe-native
install -Dm644 native/mjqbe-core.service   /etc/systemd/system/mjqbe-core.service
install -Dm644 native/mjqbe-native.service /etc/systemd/system/mjqbe-native.service
systemctl enable --now mjqbe-core.service mjqbe-native.service
```
