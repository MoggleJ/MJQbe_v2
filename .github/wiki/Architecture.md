# Architecture — MJQbe v2

## Stack complète

| Composant | Langage | Notes |
|---|---|---|
| App native — UI | C++ + Qt6/QML | GPU OpenGL ES, animations fluides |
| App native — logique | Rust | sqlx, tokio, monitoring système |
| Daemon matériel | C | GPIO, IR, CEC, Bluetooth, Unix sockets |
| Web backend | Python 3.11 + FastAPI | OAuth, JWT, ORM SQLAlchemy |
| Web frontend | TypeScript + React 18 | Vite, CSS variables thèmes |
| Base de données | PostgreSQL 15 | Partagée entre native et web |
| CLI | Bash | Orchestration Docker + commandes système |

## Services Docker

```
docker-compose.yml          ← base (db, api, frontend, daemon)
docker-compose.prod.yml     ← port 4444:80 pour frontend
docker-compose.dev.yml      ← hot-reload (Vite HMR + uvicorn --reload)
```

## Architecture Clean (Domain → Application → Infrastructure → Interface)

```
api/app/
├── domain/           ← entités, interfaces repository (aucune dépendance)
├── application/      ← use cases (dépend de domain)
├── infrastructure/   ← PostgreSQL, Docker socket, hardware
└── interface/        ← routes FastAPI, schemas Pydantic
```

## Flux de données — App native

```
QML (C++) ──── IpcClient (QLocalSocket) ──→ /run/mjqbe/native.sock
                                              │
                                         Rust server (tokio)
                                              │
                                    ┌─────────┴──────────┐
                                PostgreSQL            Daemon C
                              (sqlx, async)    (/run/mjqbe/daemon.sock)
                                                         │
                                                    GPIO / CEC / IR
```

## Flux de données — Interface web

```
Browser ──── HTTPS → Nginx :4444 ──── proxy /api → FastAPI :4848
                                              │
                                    PostgreSQL :5432 (Docker)
                                              │
                                    Daemon C (socket Unix)
```

## IPC Protocole (C++ ↔ Rust)

Requête :
```json
{ "action": "get_apps", "payload": { "mode": "tv" } }
```

Réponse :
```json
{ "ok": true, "data": [ { "id": 1, "name": "Netflix", ... } ] }
```

## Modèle de données

| Table | Rôle |
|---|---|
| users | Comptes utilisateurs web |
| apps | Applications disponibles (toutes sources) |
| categories | Catégories par mode (tv/desktop/dev) |
| settings | Préférences par utilisateur |
| favorites | Apps favorites par utilisateur |
| logs | Journal des actions web |

Voir `docs/data-model.md` pour le schéma SQL complet.
