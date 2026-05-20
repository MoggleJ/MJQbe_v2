# App Native — C++/Qt6/QML + Rust

Application locale tournant directement sur le Pi comme processus systemd.

## Structure

```
native/
├── ui/                  # C++ + Qt6/QML
│   ├── CMakeLists.txt
│   ├── main.cpp
│   └── qml/
│       ├── Main.qml
│       ├── components/  # Theme.qml, Sidebar.qml, AppCard.qml
│       └── modes/
│           ├── tv/
│           ├── desktop/
│           └── dev/
├── core/                # Rust — logique + données
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs      # Serveur IPC Unix socket
│       ├── db/          # sqlx PostgreSQL
│       ├── system/      # /proc monitoring
│       └── daemon_client/
└── mjqbe-native.service # systemd unit
```

## Build

```bash
# Rust (core IPC server)
cd native/core
cargo build --release

# C++/Qt6
mkdir native/ui/build && cd native/ui/build
cmake .. -DCMAKE_BUILD_TYPE=Release
cmake --build . --parallel

# Lancer
./mjqbe-native
```

## IPC — C++ client ↔ Rust serveur

Le C++ est client, Rust est serveur. Communication via socket Unix JSON.

```
Socket : /run/mjqbe/native.sock

Requête  → { "action": "get_apps", "payload": { "mode": "tv" } }
Réponse  ← { "ok": true, "data": [...] }
```

Actions disponibles :
- `get_apps` — liste apps par mode
- `get_categories` — catégories par mode
- `search_apps` — recherche
- `get_favorites` / `add_favorite` / `remove_favorite`
- `auth_check` — vérification mot de passe admin
- `get_system_stats` — CPU/RAM/disque/temp (mode Dev)
- `get_processes` / `kill_process` — gestion processus
- `get_containers` / `start_container` / `stop_container`

## Modes

| Mode | Accès | Contenu |
|---|---|---|
| TV | Tous | Grille large, navigation télécommande |
| Desktop | Tous | Layout dense, catégories groupées |
| Dev | Admin (re-auth) | Monitoring, terminal, Docker, GPIO |

## Thèmes QML

Singleton `Theme.qml` avec propriétés bindées :

```qml
Theme.bgPrimary   // #0f0f0f (amoled)
Theme.bgSidebar   // #141414
Theme.accent      // #00bcd4
Theme.textPrimary // #ffffff
```

## Systemd

```bash
# Installer
sudo cp native/mjqbe-native.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable mjqbe-native
sudo systemctl start mjqbe-native

# Logs
journalctl -u mjqbe-native -f
```

## Mode stub (hors Pi)

```bash
MJQBE_PI=0 cargo run   # Désactive GPIO, socket daemon optionnel
```
