# Installation — MJQbe v2

## Prérequis

- Raspberry Pi 4 (4 Go RAM recommandé) ou machine de dev x86
- Docker + Docker Compose
- Git
- Pour l'app native : Qt6, CMake, Rust (installés sur le Pi)

## Setup rapide

```bash
# 1. Cloner le repo
git clone https://github.com/MoggleJ/MJQbe_v2.git
cd MJQbe_v2

# 2. Configurer
cp .env.example .env
# Éditer .env : POSTGRES_PASSWORD, SECRET_KEY (openssl rand -hex 32)

# 3. Lancer
chmod +x cli/dev
cli/dev up
```

L'interface web est accessible sur `http://localhost:4444`.

## Mode développement (hot-reload)

```bash
cli/dev watch
# Frontend : http://localhost:4444 (Vite HMR)
# API : http://localhost:4848 (uvicorn --reload)
```

## Configuration

Toute la configuration est dans `config/config.yml` :

```yaml
server:
  web_port: 4444      # Port interface web
  api_port: 4848      # Port API REST
  domain: ""          # Domaine (vide = IP directe)
  https: false
```

> **Note :** Modifier `config.yml` puis `dev up` pour appliquer.

## Variables d'environnement (.env)

| Variable | Description |
|---|---|
| `POSTGRES_PASSWORD` | Mot de passe PostgreSQL |
| `SECRET_KEY` | Clé JWT (openssl rand -hex 32) |
| `GOOGLE_CLIENT_ID` | OAuth Google (optionnel) |
| `GOOGLE_CLIENT_SECRET` | OAuth Google (optionnel) |
| `GITHUB_CLIENT_ID` | OAuth GitHub (optionnel) |
| `GITHUB_CLIENT_SECRET` | OAuth GitHub (optionnel) |

## Problème Docker snap (permission denied)

Si Docker est installé via snap :

```bash
sudo chown root:docker /var/run/docker.sock
```

Voir [Problèmes connus](Problemes#p1----docker-snap--permission-denied).

## Compte admin par défaut

Après le premier `dev up`, un compte admin est créé automatiquement par le seed.  
Modifier les credentials dans `api/app/infrastructure/db/seed.py` avant le premier lancement.
