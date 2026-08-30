# Actions décisives — MJQbe v2

Consigne uniquement ce qui change l'état du projet ou de la machine de façon non triviale :
- environnements / interfaces / conteneurs / réseaux créés
- fichiers modifiés (chemin, lignes, nature de la modif, raison)
- config système touchée (avec moyen de revenir en arrière)

Chaque entrée est horodatée.

---

## 2026-08-30

### 22:4x CEST — Création du dossier `tracking/`
- **Type** : ajout de fichiers de journalisation (non fonctionnels, docs).
- **Fichiers** : `tracking/README.md`, `tracking/journal-commandes.md`, `tracking/suivi-avancement.md`, `tracking/actions-decisives.md`.
- **Raison** : demande explicite — traçabilité des commandes, de l'avancement et des actions décisives.
- **Impact machine** : aucun (fichiers dans le repo).

### 22:4x CEST — Commit des modifications en attente sur `dev`
- **Type** : commit git.
- **Fichiers concernés** :
  - `api/requirements.txt` — montée de versions des dépendances (fastapi 0.111→0.136, uvicorn 0.30→0.47, pyyaml 6.0.2→6.0.3, sqlalchemy 2.0.31→2.0.49, alembic 1.13.2→1.18.4, psycopg2-binary 2.9.9→2.9.12, bcrypt 4.1.3→5.0.0). Raison : alignement sur versions à jour ; bcrypt 5.x reste compatible avec `bcrypt.hashpw/gensalt` utilisés dans `seed.py`.
  - `api/app/infrastructure/db/session.py:16` — `sessionmaker(autocommit=False, autoflush=False, bind=engine)` → `sessionmaker(engine, autocommit=False, autoflush=False)`. Raison : forme d'appel recommandée SQLAlchemy 2.x (engine en 1er argument positionnel).
  - `.claude/settings.json` — ajout de permissions d'outils (gh, docker exec, pip, lecture /tmp…) + `additionalDirectories`. Raison : réduire les prompts de permission ; aucun effet fonctionnel sur le projet.
- **Impact machine** : aucun (pas encore de `pip install` exécuté ; les conteneurs tournent sur les images déjà buildées).

---

## 2026-08-30 — Sprint 3 (app native)

### ~22:52 CEST — Installation de Rust (rustup)
- **Type** : install toolchain, **user-local** (pas root).
- **Emplacements** : `~/.cargo/`, `~/.rustup/` (+ ligne PATH dans `~/.bashrc`, `~/.profile`).
- **Version** : Rust/Cargo 1.98.0, composants `rustfmt` + `clippy` ajoutés.
- **Raison** : Sprint 3 = crate Rust `native/core` ; absent du poste.
- **Réversible** : `rustup self uninstall`.

### ~23:00 CEST — Création de `native/` (nouveau module projet)
- **Type** : nouveau code, ~41 fichiers.
- **Contenu** : `native/core/` (Rust) + `native/ui/` (C++/Qt6/QML) + `native/mjqbe-core.service` + `native/mjqbe-native.service` + `native/README.md` + `native/.gitignore`.
- **Ignoré git** : `native/core/target/`, `native/ui/build/`.
- **Raison** : livrable Sprint 3.

### ~23:05 CEST — `docker-compose.native.yml` (nouvel override)
- **Type** : nouveau fichier compose (override dev natif).
- **Effet** : publie le conteneur `db` sur `127.0.0.1:${MJQBE_DB_HOST_PORT:-15432}:5432` (**loopback uniquement**, jamais réseau).
- **Raison** : l'app native (processus hors Docker) doit joindre PostgreSQL ; 5432 est pris par le PostgreSQL système → port isolé 15432.
- **Réversible** : `docker compose -f docker-compose.yml -f docker-compose.prod.yml up -d db` (fait en fin de sprint — `db` remis en config standard, port interne seul).

### ~23:06 CEST — Conteneur `db` recréé 2×
- **Type** : recreate du conteneur `mjqbe_v2-db-1` (données préservées, volume `db-data` intact).
- 1er recreate : échec (bind 5432) → conteneur laissé `Created` → récupéré au 2e recreate (port 15432).
- Fin de sprint : 3e recreate en config prod standard. **État final : `Up (healthy)`, `5432/tcp` interne, aucune donnée perdue.**

### ~23:08 CEST — Image Docker `mjqbe-native-smoke` (temporaire, supprimée)
- **Type** : image de test throwaway (`debian:bookworm` + Qt6), `.native-smoke.Dockerfile` (gitignored).
- **Raison** : le poste n'a pas les `qml6-module-qtquick-*` → smoke-test QML dans un env Docker isolé (base Qt 6.4 identique au Pi).
- **Nettoyage** : `docker rmi mjqbe-native-smoke` + suppression du Dockerfile + `.smoke-out/` en fin de sprint. **Ne subsiste pas.**

### Fichiers modifiés (hors `native/`)
| Fichier | Modif | Pourquoi |
|---|---|---|
| `docs/CDC.md` §2.1 (l.19-27) | « PySide6 / QML » → « C++ / Qt6 / QML + Rust » + détails IPC/systemd | incohérence spec (le plan et §6.1 disaient déjà C++/Rust) |
| `docs/plan-implementation.md` Sprint 3 (l.53-73) | `[ ]`→`[x]` sur 11 tâches, titre `+✓`, bloc statut ajouté | sprint terminé |
| `.github/workflows/native-build.yml` (l.65-70) | paquets apt : `+qt6-base-dev-tools +qt6-declarative-dev-tools +ninja-build +libgl1-mesa-dev` | `qt_add_qml_module` a besoin des dev-tools pour builder en CI |
| `problemes.md` | +P8, P9, P10, P11 | problèmes rencontrés pendant le sprint |

---

## 2026-08-30 — Sprint 4 (mode TV + Desktop natif)

### ~23:20 CEST — Extension de `native/core` (schéma IPC élargi)
- **Type** : nouveau code + nouvelles méthodes IPC (compat ascendante : anciennes méthodes inchangées).
- **Fichiers** : +`application/favorites.rs`, +`application/settings.rs`, +`infrastructure/db/favorites_repo.rs`, +`infrastructure/db/settings_repo.rs` ; modifiés : `domain/{entities,mod,repository}.rs`, `application/{mod,auth,catalog}.rs`, `infrastructure/db/{mod,auth_repo,catalog_repo}.rs`, `interface/ipc/{mod,handler}.rs`, `main.rs`, `tests/ipc_roundtrip.rs`.
- **Nouvelles méthodes** : `session.current`, `apps.recent`, `favorites.list`, `favorites.toggle`, `settings.get`, `settings.update`.
- **Raison** : tâches Sprint 4 (favoris + settings persistés + récents).

### ~23:30 CEST — Extension de `native/ui`
- **Fichiers** : +`qml/AppGrid.qml`, +`qml/CategoryChips.qml`, +`native/ui/Dockerfile.smoketest`, +`native/ui/smoketest.sh` ; modifiés : `CMakeLists.txt`, `src/NativeBridge.{h,cpp}`, `qml/{Main,AppCard}.qml`, `qml/pages/{Home,AllApps,Search,Settings}.qml`.
- **Note** : `Dockerfile.smoketest` = outil de test QML offscreen, **pas** un artefact de déploiement (natif = systemd, CDC §10.2).

### Fichiers hors `native/` / `docs`
| Fichier | Modif | Pourquoi |
|---|---|---|
| `docs/plan-implementation.md` Sprint 4 | 10 tâches `[x]`, bloc statut | sprint terminé |
| `.gitignore` | +`/.smoke-out/` | sortie transitoire du smoke-test |

---

## 2026-08-30 — Sprint 5 (mode Dev natif)

### ~23:40 CEST — `native/core` : module `infrastructure/system/` + contrôle système
- **Type** : nouveau code ; **capacités système** ajoutées au core (lecture `/proc`+`sysfs`, `libc::kill`/`setpriority`, exécution de `docker`).
- **Fichiers** : +`infrastructure/system/{mod,metrics,processes,docker}.rs`, +`application/dev.rs` ; modifiés : `domain/{entities,error,mod}.rs`, `application/{mod,auth}.rs`, `interface/ipc/{mod,handler}.rs`, `main.rs`, `tests/ipc_roundtrip.rs`, `Cargo.toml` (+`libc`, +`rand`, tokio feature `process`).
- **Sécurité** : mutations (kill/nice/docker start-stop) exigent un token `auth.verify` (bcrypt, usage unique, TTL 120 s) ; `pid ≤ 1` refusé ; id conteneur validé (anti-injection d'arguments).
- **Raison** : tâches Sprint 5 (monitoring + contrôle process/Docker + re-auth).

### ~23:44 CEST — `native/ui` : mode Dev
- **Fichiers** : +`src/TerminalController.{h,cpp}` (QProcess→bash), +`qml/Gauge.qml`, +`qml/pages/Dev.qml` ; modifiés : `CMakeLists.txt`, `src/{main.cpp,NativeBridge.{h,cpp}}`, `qml/{Main,Sidebar}.qml`.
- **Note** : le terminal exécute `bash -i` **dans le process UI**, pas via le core. Sur le Pi c'est l'utilisateur `mjqbe` (droits de la session).

### Fichiers hors `native/`
| Fichier | Modif | Pourquoi |
|---|---|---|
| `docs/plan-implementation.md` Sprint 5 | 9 tâches `[x]`, bloc statut | sprint terminé |

---

## 2026-08-30 — Sprint 6 (UX & animations natives)

### ~23:52 CEST — `native/ui` : animations + focus
- **Fichiers** : +`qml/LoadingCube.qml` ; modifiés : `CMakeLists.txt`, `qml/Main.qml` (overlay + transitions StackView), `qml/SidebarButton.qml` (activeFocusOnTab + anneau focus), `native/ui/smoketest.sh` (report VmRSS), `native/.gitignore` (`ui/build*/`).
- **Choix** : animations 2.5D (`transform: Rotation` + `Scale`), **pas** de Qt3D/`ShaderEffect` — priorité conso RAM/GPU Pi 4.
- **Mesure** : VmRSS ~50 Mo (debug, offscreen, x86) — sous la cible 150 Mo.

### Fichiers hors `native/`
| Fichier | Modif | Pourquoi |
|---|---|---|
| `docs/plan-implementation.md` Sprint 6 | 6 tâches `[x]`, bloc statut | sprint terminé |

---

## 2026-08-31 — Fix CI (P12) + Sprint 7 (daemon GPIO)

### ~00:00 CEST — Fix CI api-ci + GitGuardian (commit `fix(ci)`)
- **Fichiers** : +`api/setup.cfg` (flake8 + pytest), +`api/requirements-dev.txt`, +`api/tests/{conftest,test_health,test_seed}.py` (4 tests) ; `.github/workflows/api-ci.yml` (lint `cd api && flake8 app tests`, install dev-reqs) ; `.gitguardian.yaml` (ignore `tracking/**`, `**/*.md`, `**/tests/**`) ; scrub `postgres://mjqbe:mjqbe@…` → `${POSTGRES_USER}:${POSTGRES_PASSWORD}` dans `docker-compose.native.yml` + `native/README.md`.
- **Résultat** : les 3 workflows CI verts. Incident GitGuardian existant → à clôturer dans le dashboard GG (hors CLI).

### ~00:05 CEST — `daemon/main.c` réécrit
- **Type** : le daemon **pilote du vrai matériel** sur le Pi (sysfs GPIO : export/direction/value). Hors-Pi : **stub** (aucune écriture matérielle) — détection via device-tree `raspberry pi`.
- **Fichiers** : `daemon/main.c` (réécrit), +`daemon/README.md`.
- **Câblage figé** : relais 1–4 → BCM 23/24/25/12 (actif-bas) ; LED RGB → 5/6/13.

### ~00:10 CEST — Capacité « hardware » ajoutée au core Rust + à l'API
- **core** : +`infrastructure/hardware/daemon_client.rs` (client socket daemon), +`application/hardware.rs` ; IPC `gpio.set`/`relay.set`/`led.set` **exigent un token de ré-auth**.
- **api** : +`app/infrastructure/hardware/daemon_client.py`, +`app/interface/routes/dev.py` (routes `/dev/*`). **Non authentifiées jusqu'au Sprint 10** (noté, issue #67).
- **cli/dev** : +`gpio` / +`relay` (via API).
- **Impact machine** : conteneurs `daemon` + `api` reconstruits et recréés (stack `dev` healthy).

### Fichiers hors `native/` / `daemon/` / `api/`
| Fichier | Modif | Pourquoi |
|---|---|---|
| `docs/plan-implementation.md` Sprint 7 | 8 tâches `[x]`, bloc statut | sprint terminé |
| `cli/dev` | +`cmd_gpio`, +`cmd_relay`, dispatch, help | tâche CLI |

---

## 2026-08-31 — Sprint 8 (daemon AV)

### ~00:35 CEST — `daemon` : sous-système AV
- **Type** : nouvelles **capacités matérielles** — le daemon exécute `cec-client` (sous-processus) et ouvre le socket LIRC + un device série (`/dev/serial0`).
- **Fichiers** : +`daemon/av.{c,h}`, +`daemon/ir-map.json` ; `daemon/main.c` (extraction `daemon_relay_set`, +5 cmds, `av_init`), `daemon/Makefile` (+av.c, `-lpthread`), `daemon/Dockerfile` (+`cec-utils`, `COPY ir-map.json` → `/etc/mjqbe/`).
- **Threads** : 2 pthreads détachés (IR, BT) démarrés à `av_init()` ; se terminent proprement si le device est absent.
- **Impact machine** : image `daemon` reconstruite (+`cec-utils` ~ paquets CEC) ; conteneurs `daemon`+`api` recréés.

### ~00:40 CEST — core + api + ui : surface AV
- **core** : IPC `av.send` **token de ré-auth** ; `av.status` ouvert.
- **api** : `GET/POST /dev/av` (non authentifié jusqu'au Sprint 10, #67).
- **ui** : `Dev.qml` boutons AV (ré-auth par action).

### Fichiers hors daemon/core/api/ui
| Fichier | Modif | Pourquoi |
|---|---|---|
| `docs/plan-implementation.md` Sprint 8 | 7 tâches `[x]`, bloc statut | sprint terminé |

---

## 2026-08-31 — Sprint 9 (reconnaissance vocale)

### ~01:00 CEST — `native/core` : module voix
- **Type** : nouveau code (grammaire + use case + IPC). Recogniser réel `vosk_engine.rs` **feature-gated** (`[features] vosk = []`) — non compilé par défaut, aucune nouvelle dépendance runtime.
- **Fichiers** : +`infrastructure/voice/{mod,grammar,vosk_engine}.rs`, +`application/voice.rs` ; `domain/{entities,mod}.rs` (+`VoiceAction`, `ParsedUtterance`), `domain/repository.rs` + `infrastructure/db/catalog_repo.rs` + `application/catalog.rs` (`search_apps`/`find_app`), `interface/ipc/handler.rs` (+3 méthodes + `run_voice_action`), `main.rs`, `tests/ipc_roundtrip.rs`, `Cargo.toml`.
- **IPC** : `voice.set_enabled` **token de ré-auth** ; `voice.status`/`voice.simulate` ouverts (simulate = outil de dev).

### ~01:02 CEST — `daemon/av.c` : garde-fou CEC
- `cec_send` → `timeout 6 sh -c "…cec-client…"` (évite le blocage sans adaptateur). `DaemonClient` (Rust) timeout 3 s → 8 s.
- **Impact machine** : image `daemon` reconstruite + conteneur recréé.

### ~01:03 CEST — `native/ui`
- `Sidebar.qml` : indicateur voix (point pulsant). `Main.qml` : Timer 2 s `voice.status`. `Dev.qml` : panneau simulate + toggle.

### Fichiers hors core/ui/daemon
| Fichier | Modif | Pourquoi |
|---|---|---|
| `docs/plan-implementation.md` Sprint 9 | tâches `[x]`/`[~]`, bloc statut | sprint terminé (capture audio différée) |

---

## 2026-08-31 — Sprint 10 (authentification WEB)

### ~01:20 CEST — API : couche auth complète
- **Type** : nouvelle surface HTTP (`/auth/*`) + **protection** de `/dev/*`.
- **Fichiers** : +`api/app/infrastructure/security/{__init__,passwords,tokens}.py`, +`api/app/infrastructure/db/user_repo.py`, +`api/app/infrastructure/oauth/{__init__,providers}.py`, +`api/app/application/auth_service.py`, +`api/app/interface/{deps,routes/auth}.py`, +`api/tests/test_auth.py` ; modifiés : `api/app/main.py` (réécrit), `api/requirements.txt` (+pyjwt, +httpx, +email-validator), `api/requirements-dev.txt`.
- **Sécurité** : `SECRET_KEY` (env) pour signer les JWT ; client id/secret OAuth **uniquement** via env (`GOOGLE_/GITHUB_CLIENT_ID/SECRET`) — jamais dans `config.yml`. `/dev/*` désormais **admin-only** (#67 partiel : `/admin/*` au Sprint 12).
- **Impact machine** : image `api` reconstruite + conteneur recréé (stack `dev` healthy). Nouveaux users possibles via `POST /auth/register` (base `mjqbe`).

### Fichiers hors api
| Fichier | Modif | Pourquoi |
|---|---|---|
| `docs/plan-implementation.md` Sprint 10 | 10 tâches `[x]`, bloc statut | sprint terminé |

---

## 2026-08-31 — Sprint 11 (API apps & catégories)

### ~01:45 CEST — API : CRUD apps + catégories
- **Fichiers** : +`api/app/infrastructure/db/catalog_repo.py`, +`api/app/interface/routes/catalog.py`, +`api/tests/test_catalog.py` ; `api/app/main.py` (+2 routers).
- **Accès** : GET `/apps` + `/categories` **publics** ; POST/PUT/DELETE **`Depends(require_admin)`**.
- **Impact machine** : image `api` reconstruite + recréée. Une app de test créée puis supprimée (base `mjqbe`).

### Fichiers hors api
| Fichier | Modif | Pourquoi |
|---|---|---|
| `docs/plan-implementation.md` Sprint 11 | 10 tâches `[x]`, bloc statut | sprint terminé |

---

## 2026-08-31 — Sprint 12 (settings/favoris/logs/admin système)

### ~02:00 CEST — API : données per-user + admin système
- **Fichiers** : +`api/app/infrastructure/db/user_data_repo.py`, +`api/app/infrastructure/docker_client.py`, +`api/app/infrastructure/config_file.py`, +`api/app/interface/routes/{user_data,admin}.py`, +`api/tests/test_{user_data,admin}.py` ; modifiés : `api/app/interface/{deps,routes/auth,routes/catalog}.py`, `api/app/application/auth_service.py`, `api/app/main.py`.
- **`docker-compose.yml`** : mount `./config:/app/config` et `/var/run/docker.sock` **sans `:ro`** (l'admin panel écrit config.yml et pilote les services). Retour arrière : remettre `:ro` (mais `PUT /admin/config` + actions services cassent).
- **Sécurité** : `/admin/*` = JWT admin ; `PUT /admin/config` + `/reboot` = re-auth mot de passe (body). Socket Docker rw dans l'api → à durcir S17.
- **Impact machine** : image `api` reconstruite + recréée ; ~27 users de test dans la base `mjqbe` (dev).

### Fichiers hors api
| Fichier | Modif | Pourquoi |
|---|---|---|
| `docs/plan-implementation.md` Sprint 12 | 16 tâches `[x]`, bloc statut | sprint terminé |
| `docker-compose.yml` (l.24-27) | `./config` + docker.sock → rw | admin panel (config write + services) |

---

## 2026-08-31 — Sprint 13 (Frontend React + TS)

### ~02:50 CEST — `frontend/` : réécriture React 18 + Vite + TypeScript
- **Type** : remplacement complet du stub `main.jsx` par une SPA (~25 fichiers `src/`).
- **Fichiers clés** : `src/api/client.ts` (refresh JWT auto), `src/auth/AuthContext.tsx`, `src/theme/{themes,UiContext}.tsx`, `src/pages/*`, `src/App.tsx`, `styles.css`, `package.json`/`tsconfig.json`/`vite.config.ts`, `frontend/.dockerignore` (nouveau).
- **Build/test** : impossible sur l'hôte (`node_modules` appartient à root, pas de sudo) → **tout via Docker** (`docker build ./frontend`, `docker run --target builder npx vitest run`).
- **Impact machine** : image `frontend` reconstruite + conteneur recréé (nginx sert la SPA + proxifie `/api`).

### Fichiers hors frontend
| Fichier | Modif | Pourquoi |
|---|---|---|
| `docs/plan-implementation.md` Sprint 13 | 17 tâches `[x]`, bloc statut | sprint terminé |

---
## 2026-08-31 — Sprint 14 (Frontend mode Desktop)
- **Fichiers** : +`frontend/src/components/GroupedApps.tsx`, +`frontend/src/pages/Favorites.tsx` ; modifiés : `App.tsx` (+route), `components/Sidebar.tsx` (+lien), `pages/{Home,AllApps}.tsx`, `styles.css`.
- `docs/plan-implementation.md` Sprint 14 → `[x]` + statut.

---
## 2026-08-31 — Sprint 15 (Frontend UX & animations)
- **Fichiers** : +`frontend/src/components/LoadingCube.tsx` ; modifiés : `frontend/src/App.tsx`, `frontend/src/pages/Settings.tsx`, `frontend/src/styles.css`.
- Image `frontend` reconstruite + recréée.
- `docs/plan-implementation.md` Sprint 15 → `[x]` + statut.
