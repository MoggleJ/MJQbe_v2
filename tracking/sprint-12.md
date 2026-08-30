# Sprint 12 — Settings, Favorites, Logs & Admin Système (WEB)

**Terminé le** 2026-08-31 (~02:20 CEST) · **Branche** `sprint-12-actions`
**Issues fermées** : #79–#92 + **#67** (`/admin/*` maintenant protégé)

---

## 1. Ce qui a été fait

| Fichier | Rôle |
|---|---|
| `infrastructure/db/user_data_repo.py` | `SettingsRepository` (get_or_create / update), `FavoritesRepository` (list_app_ids / add / remove), `LogRepository` (record / list / count), `list_users`. Enums de validation exportés. |
| `infrastructure/docker_client.py` | API Docker via `/var/run/docker.sock` (`httpx.HTTPTransport(uds=…)`). `list_services()` (filtré `com.docker.compose.project`), `service_action(name, restart\|stop\|start)`. `DockerUnavailable` si socket absent. |
| `infrastructure/config_file.py` | `read_config()` / `write_config()` — valide `server.web_port` + `server.api_port`, `yaml.safe_dump(sort_keys=False)`. |
| `application/auth_service.py` | `AuthService(users, settings=None)` — `_ensure_settings` appelé après `register` **et** `oauth_upsert`. |
| `interface/deps.py` | +`get_optional_user` (renvoie `None` au lieu de 401), +`verify_reauth(user, password)`. |
| `interface/routes/user_data.py` | `GET/PUT /settings`, `GET /favorites`, `POST/DELETE /favorites/{app_id}` (auth requise). |
| `interface/routes/admin.py` | routeur `/admin` avec `dependencies=[Depends(require_admin)]` : `/logs` (pagination), `/users`, `/config` (GET + PUT re-auth), `/services` (GET), `/services/{name}/{restart,stop}` (202), `/reboot` (re-auth, api en dernier hors thread). |
| `interface/routes/catalog.py` | `GET /apps/{id}` → `LogRepository.record("app_launch", …)` si viewer authentifié. |
| `main.py` | +`user_data` + `admin` routers. |
| `docker-compose.yml` | mount `./config` → **rw** (au lieu de `:ro`) + socket Docker sans `:ro` → `PUT /admin/config` et actions services fonctionnent. |

---

## 2. Vérifications

### `pytest` — **36 passed** (+13 vs Sprint 11)
- `test_user_data.py` : settings auto-créés au register + `PUT` persistant ; enum invalide → 422 ; `/settings` sans token → 401 ; favoris add/list/remove + idempotence + 404 app inconnue ; `GET /apps/{id}` authentifié → log `app_launch` compté dans `/admin/logs`.
- `test_admin.py` : toutes les routes `/admin/*` → 401 sans token / 403 non-admin ; `/logs` pagination ; `/users` contient `admin` ; `/config` **round-trip sur fichier temporaire** (`CONFIG_PATH` monkeypatché) + rejet structure invalide (422) + mauvais mot de passe (401) ; `/services` → liste ou 503 ; `/reboot` mauvais mot de passe → 401.

### Live (stack Docker)
- settings d'un nouveau user : `theme:"dark"` → `PUT {theme:"light-blue"}` OK
- `POST /favorites/4` → `{"app_ids":[4]}`
- `GET /apps/4` (Bearer user) → 200, puis `/admin/logs` : `total 2`, `items[0].action == "app_launch"`
- `GET /admin/users` → 27 ; `GET /admin/config` → `server.web_port = 4444`
- **`GET /admin/services`** → `[api, daemon, db, frontend]` avec `state`/`status` réels (socket Docker)

---

## 3. Reste à faire / décisions

- **`docker-compose.yml`** : `./config` et le socket Docker ne sont plus `:ro` pour l'`api`. Nécessaire pour l'admin panel. Le socket Docker en rw dans un conteneur = surface d'attaque → l'accès est **admin-only + re-auth** sur les actions destructives ; à revoir au Sprint 17 (sécu — envisager un proxy socket type `docker-socket-proxy`).
- `POST /admin/reboot` redémarre `api` en dernier via `threading.Timer` → la réponse 202 part avant la coupure. Le client doit se reconnecter ensuite. Non testable en intégration sans couper le conteneur — couvert seulement pour le refus re-auth.
- `/admin/logs` : pas de filtre par `action`/`user_id`/date — à ajouter si le panel en a besoin (Sprint 13).
- `get_config` (`deps.py`, lru_cache) sert le CORS/ports au boot ; `PUT /admin/config` écrit le fichier mais l'API ne recharge pas ses origines CORS à chaud (redémarrage nécessaire — cohérent avec le reste).

## 4. Comment ça fonctionne

**Settings à l'inscription** : `POST /auth/register` → `AuthService.register` → `UserRepository.create` → `_ensure_settings` → `SettingsRepository.get_or_create` (INSERT valeurs par défaut).

**app_launch** : le frontend ouvre le détail d'une app → `GET /apps/{id}` avec le Bearer → `get_optional_user` résout l'utilisateur → `LogRepository.record("app_launch", uid, {"app_id": id})`. `GET /admin/logs` les pagine (tri `created_at DESC`).

**Admin système** : `/admin/*` exige un JWT admin (dépendance de routeur). `PUT /admin/config` et `/reboot` exigent en plus le mot de passe de l'admin dans le body (`verify_reauth`). `/services*` parle au démon Docker via le socket Unix monté (`httpx` transport UDS), filtre sur le label `com.docker.compose.project = mjqbe_v2`.
