# Architecture logicielle — MJQbe v2

## 1. Vue d'ensemble

```
┌─────────────────────────────────────────────────┐
│                   Navigateur web                 │
│         React 18 + Vite (port 3000)             │
└───────────────────┬─────────────────────────────┘
                    │ HTTP / REST
┌───────────────────▼─────────────────────────────┐
│              API Backend                         │
│         Python 3.11 + FastAPI (port 8000)       │
└────┬──────────────┬──────────────────────────────┘
     │              │
┌────▼────┐   ┌─────▼──────────────────────────────┐
│PostgreSQL│   │     Hardware Daemon (C)             │
│  (5432)  │   │  GPIO, IR, HDMI CEC, Bluetooth     │
└──────────┘   └─────────────────────────────────────┘
```

---

## 2. Services Docker

### `docker-compose.yml` — services

| Service | Image | Port | Rôle |
|---|---|---|---|
| `db` | postgres:15 | 5432 (interne) | Base de données PostgreSQL |
| `api` | ./api (Python) | **4848** | Backend FastAPI |
| `frontend` | ./frontend (React + Nginx) | **8484** | Interface web servie par Nginx |
| `daemon` | ./daemon (C) | — | Contrôle matériel (Pi uniquement) |

**Ports configurables dans `config/config.yml`** :
- `server.api_port` : 4848
- `server.web_port` : 8484

**Réseau Docker :** `mjqbe-network` (bridge interne, seuls `frontend` et `api` exposent des ports)

**Volumes :**
- `postgres_data` → persistance BDD
- `daemon_sock` → socket Unix pour communication api ↔ daemon

---

## 3. Clean Architecture

```
mjqbe-api/
├── domain/          # Entités métier, interfaces, règles (pas de dépendances externes)
│   ├── entities/    # App, User, Category, Settings, Favorite, Log
│   └── ports/       # Interfaces des repositories
├── application/     # Cas d'usage (use cases), orchestration
│   └── use_cases/
├── infrastructure/  # Implémentations concrètes (BDD, OAuth, JWT, daemon)
│   ├── db/          # Repositories PostgreSQL (SQLAlchemy)
│   ├── auth/        # JWT, OAuth Google/GitHub
│   └── hardware/    # Client socket daemon C
└── interface/       # Routes FastAPI, schémas Pydantic, middlewares
    ├── routers/
    └── schemas/
```

**Règle absolue :** aucune couche ne dépend d'une couche au-dessus d'elle. La couche `domain` ne connaît rien de FastAPI ou PostgreSQL.

---

## 4. Flux d'authentification

### JWT (local)
```
Client → POST /auth/login {username, password}
API → vérifie bcrypt → génère JWT (access + refresh)
Client → stocke JWT → envoie dans header Authorization: Bearer <token>
API → middleware vérifie JWT à chaque requête protégée
```

### OAuth 2.0 (Google / GitHub)
```
Client → GET /auth/oauth/{provider}
API → redirige vers provider (Google/GitHub)
Provider → callback → GET /auth/oauth/{provider}/callback?code=...
API → échange code contre token → récupère profil → crée/trouve user → génère JWT
Client → reçoit JWT
```

---

## 5. Protection des routes

| Route | Accès |
|---|---|
| `GET /apps` | Authentifié |
| `POST /apps` | Admin uniquement |
| `GET /admin/*` | Admin + re-auth obligatoire |
| `GET /dev/*` | Admin + local uniquement (IP check) |
| `GET /logs` | Admin uniquement |
| `POST /auth/login` | Public |
| `GET /auth/oauth/*` | Public |

---

## 6. Communication API ↔ Daemon

Le daemon C tourne en local sur le Pi et expose un socket Unix (`/run/mjqbe/daemon.sock`).

L'API communique avec lui via des messages JSON simples :

```json
{ "action": "gpio_set", "pin": 17, "value": 1 }
{ "action": "tv_on" }
{ "action": "get_system_stats" }
```

Le daemon répond :
```json
{ "status": "ok", "data": { ... } }
```

En dehors du Raspberry Pi (dev local), le daemon est simulé par un stub Python.

---

## 7. Frontend — structure React

```
frontend/
├── src/
│   ├── components/     # Composants réutilisables (AppCard, Sidebar, SearchBar…)
│   ├── pages/          # Pages (Home, AllApps, Settings, DevMode…)
│   ├── modes/          # TV, Desktop (layouts spécifiques)
│   ├── hooks/          # Custom hooks (useAuth, useApps, useTheme…)
│   ├── services/       # Appels API (axios)
│   ├── store/          # État global (Zustand ou Context)
│   └── styles/         # Thèmes CSS (10 variables de thème)
└── nginx.conf          # Sert le build React, proxy /api → api:8000
```

---

## 8. CLI `dev` (Bash)

Script Bash unique, installé en tant que commande système sur le Pi.

```
dev up          # docker compose up -d
dev down        # docker compose down
dev logs [svc]  # docker compose logs -f [service]
dev restart     # docker compose restart
dev status      # état de tous les conteneurs
dev db          # ouvre psql dans le conteneur db
dev shell [svc] # ouvre un shell dans un conteneur
dev update      # git pull + rebuild + restart
dev sprint      # exécute le workflow de sprint (voir agents/sprint-workflow.md)
dev gpio <pin> <val>  # commande directe au daemon GPIO
```
