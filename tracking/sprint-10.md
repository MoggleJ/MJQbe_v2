# Sprint 10 — Authentification (WEB)

**Terminé le** 2026-08-31 (~01:40 CEST) · **Branche** `sprint-10-actions`
**Issues fermées** : #59, #60, #61, #62, #63, #64, #65, #66, #68 · **reste** : #67 (`/admin/*` → Sprint 12)

---

## 1. Ce qui a été fait (Clean Architecture FastAPI)

| Couche | Fichier | Rôle |
|---|---|---|
| infrastructure/security | `passwords.py` | `hash_password` / `verify_password` (bcrypt) |
| | `tokens.py` | JWT HS256 — `create_access_token` / `create_refresh_token` (lifetimes ← `config.yml` `auth.*`), `decode_token(expected_type)` ; secret ← `SECRET_KEY` (env) |
| infrastructure/db | `user_repo.py` | `UserRepository` : `get`, `get_by_username`, `get_by_email`, `get_by_oauth`, `create`, `touch_last_login`, `list_all` |
| infrastructure/oauth | `providers.py` | `OAuthProvider` (base), `GoogleProvider`, `GitHubProvider` — `authorization_url`, `exchange(code)` → `OAuthUser` ; `enabled` = client id+secret présents (env) |
| application | `auth_service.py` | `AuthService` : `register`, `login`, `refresh`, `oauth_upsert` (lie par `oauth_id`, sinon par email, sinon crée avec username unique). `AuthError(message, status)` |
| interface | `deps.py` | `get_db`, `get_config` (lru_cache), `get_users`, `get_current_user` (`HTTPBearer`, 401), `require_admin` (403) |
| interface/routes | `auth.py` | `POST /auth/{register,login,refresh}`, `GET /auth/me`, `GET /auth/oauth/{provider}` (+ `/callback`). State OAuth en mémoire (`secrets.token_urlsafe`). |
| | `main.py` | `include_router(auth)` ; `include_router(dev, dependencies=[Depends(require_admin)])` → **`/dev/*` admin-only** |

Deps ajoutées à `api/requirements.txt` : `pyjwt`, `httpx`, `email-validator`.

---

## 2. Vérifications

### Tests (`pytest`, contre PostgreSQL Docker) — **15 passed**
`test_health.py` (2) + `test_seed.py` (2) + `test_auth.py` (11) :
- register → 201 rôle `user` ; duplicate → 409 ; password court → 422
- login mauvais mdp → 401 ; `/auth/me` sans token → 401, avec token admin → 200 rôle `admin`
- refresh → nouveau access ; un **access** token en refresh → 401
- `/dev/hardware` : sans token → 401 ; token **user** → 403 ; token **admin** → 200/503
- `oauth/inconnu` → 404 ; `oauth/google` non configuré → 404 ; `oauth/github` configuré (monkeypatch env) → 302 vers `github.com/login/oauth/authorize?...client_id=...`
- `oauth_upsert` : 2e appel réutilise le même user

### Live (stack Docker)
- `POST /auth/login admin/admin` → JWT (183 c) ; `GET /auth/me` → `{id:1, role:"admin"}`
- `GET /dev/hardware` sans header → **401** ; avec `Bearer <admin>` → **200** `{backend:"stub"}`
- flake8 `app tests` → clean

---

## 3. Reste à faire

- **#67** `/admin/*` : aucune route admin n'existe encore — elles seront créées **au Sprint 12** et montées avec `dependencies=[Depends(require_admin)]` (même mécanisme que `/dev/*`).
- **Flux OAuth réel** : non testable ici (pas de `GOOGLE_/GITHUB_CLIENT_ID/SECRET`). L'`authorization_url`, la gestion du `state`, `exchange()` et `oauth_upsert` sont couverts ; l'aller-retour réseau avec Google/GitHub reste à valider avec de vrais identifiants + une redirect URI enregistrée.
- **Refresh tokens stateless** (JWT `type:refresh`, pas de store serveur ni de rotation/révocation). Suffisant pour un hub domestique ; une liste de révocation pourrait être ajoutée au Sprint 17 (sécu).
- `get_config` est `lru_cache` — un changement de `config.yml` à chaud n'est pas repris par l'API sans redémarrage (déjà le cas avant).

---

## 4. Comment ça fonctionne

**Local** : `POST /auth/login {username,password}` → `AuthService.login` → `UserRepository.get_by_username` + `verify_password` (bcrypt) → `touch_last_login` → `TokenPair` (access ~60 min, refresh ~30 j). Le client met `Authorization: Bearer <access>`. `get_current_user` décode, vérifie `type=access`, recharge le `User`. `require_admin` ajoute le contrôle `role == "admin"`.

**OAuth** : `GET /auth/oauth/google` → 302 vers Google avec `state` mémorisé → l'utilisateur autorise → Google appelle `GET /auth/oauth/google/callback?code&state` → `state` validé → `provider.exchange(code)` (token + userinfo) → `OAuthUser` → `oauth_upsert` → `TokenPair` renvoyée (le frontend récupère les tokens).

**Protection** : `main.py` monte le router `/dev` avec `Depends(require_admin)` → toute route `/dev/*` exige un JWT admin.
