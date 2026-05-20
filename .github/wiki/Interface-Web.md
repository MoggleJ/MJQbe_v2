# Interface Web — React + FastAPI

Interface accessible depuis n'importe quel navigateur sur le réseau local.  
Modes disponibles : **TV et Desktop uniquement** (pas de Dev).

## Stack

| Couche | Tech |
|---|---|
| Frontend | TypeScript + React 18 + Vite |
| Backend | Python 3.11 + FastAPI |
| Auth | JWT + OAuth 2.0 (Google, GitHub) |
| DB | PostgreSQL 15 (Docker) |
| Proxy | Nginx |

## Endpoints API principaux

### Auth
```
POST /auth/login         → JWT
POST /auth/register      → 201
POST /auth/refresh       → nouveau JWT
GET  /auth/oauth/google  → redirect OAuth
GET  /auth/oauth/github  → redirect OAuth
```

### Apps & Catégories
```
GET  /apps               → liste (mode=tv|desktop, category_id=...)
GET  /apps/:id           → détail + log app_launch
POST /apps               → créer (admin)
PUT  /apps/:id           → modifier (admin)
DELETE /apps/:id         → supprimer (admin)
GET  /categories         → liste par mode
```

### Utilisateur
```
GET  /settings           → préférences
PUT  /settings           → mise à jour
GET  /favorites          → favoris
POST /favorites/:app_id  → ajouter
DELETE /favorites/:app_id → retirer
```

### Admin
```
GET  /admin/users
GET  /admin/logs
GET  /admin/config
PUT  /admin/config
GET  /admin/services
POST /admin/services/:name/restart
POST /admin/services/:name/stop
POST /admin/reboot       (re-auth obligatoire)
```

## Authentification

- JWT (access 24h + refresh 7j)
- OAuth Google et GitHub
- Rôles : `user` et `admin`
- Re-auth obligatoire pour les actions sensibles

## Lancer en développement

```bash
dev watch
# → http://localhost:4444
```

## Lancer en production

```bash
dev up
# → http://localhost:4444
```
