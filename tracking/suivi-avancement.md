# Suivi d'avancement — MJQbe v2

Journal de tracking : tâches en cours, fichiers impactés, erreurs + tentatives, notes.
Chaque entrée est horodatée.

---

## État global (au 2026-08-30 22:40 CEST)

| Sprint | Statut | Note |
|---|---|---|
| 1 — Scaffolding & DevOps | ✅ terminé | branche `sprint-01-actions` |
| 2 — Base de données | ✅ terminé | branche `sprint-02-actions` ; tests pytest **manquants** (2 issues ouvertes) |
| 3 — Native scaffolding C++/Qt6 + Rust | ✅ terminé | branche `sprint-03-actions` ; voir `tracking/sprint-3.md` |
| 4 — Native mode TV + Desktop | ✅ terminé | branche `sprint-04-actions` ; voir `tracking/sprint-4.md` |
| 5 → 9 — Native (Dev/UX/daemon/AV/voix) | ⛔ non démarré | dépendances matériel Pi pour 5→9 |
| 10 → 15 — Web (auth/API/frontend) | ⛔ non démarré | testable en local via Docker |
| 16 — CLI complète | ⛔ non démarré | |
| 17 — Sécurité & déploiement | ⛔ non démarré | CI partiellement en place |

### Environnement vérifié
- Docker 29.6.1, accès direct OK, user dans groupe `docker`.
- Stack web MJQbe **déjà lancée** : `mjqbe_v2-frontend-1` (:4444, healthy), `mjqbe_v2-api-1` (:4848, healthy), `mjqbe_v2-db-1` (healthy), `mjqbe_v2-daemon-1` (running).

### Points ouverts / incohérences repérées
1. `docs/CDC.md §2.1` dit encore « PySide6 / QML » — le plan (autoritaire) et `§6.1` disent **C++/Qt6/QML + Rust**. À corriger.
2. `api/tests/` vide → `pytest` renvoie exit 5 ; risque d'échec CI `api-ci.yml`.
3. `frontend/package.json` sans script `test` → `npm run test` échouera (étape 4 du workflow) jusqu'au Sprint 13.
4. `daemon/main.c` = squelette 53 lignes, aucun traitement de commande.
5. Modifs non commitées : `requirements.txt` (bump deps, dont bcrypt 4→5 major), `session.py` (fix `sessionmaker`), `.claude/settings.json` (permissions).

---

## Entrées

### 2026-08-30 22:40 CEST — Reprise de contexte + mise en place du tracking
- **Action** : relecture complète des specs (CDC, data-model, plan, problemes, AGENTS, sprint-workflow) + inspection du code réel.
- **Fichiers créés** : `tracking/README.md`, `tracking/journal-commandes.md`, `tracking/suivi-avancement.md`, `tracking/actions-decisives.md`.
- **Erreurs** : aucune.

### 2026-08-30 22:45 CEST — Décisions utilisateur
- Ordre : **native-first** (Sprint 3 maintenant). Pas de Pi pour l'instant → code + stubs, livrables matériels « à vérifier sur Pi ».
- Autorisations : commit/push sans demander ; commandes root si besoin projet et sans impact décisif ; **Docker obligatoire pour les tests de fonctionnement** ; tout changement sur environnement/interface **isolé**.
- Message de commit : `sprint-XX: ...` **sans** trailer Co-Authored-By.

### 2026-08-30 23:15 CEST — Sprint 3 : app native (scaffolding C++/Qt6 + Rust)
- **Fait** : crate `native/core` (Rust, Clean Archi, IPC socket Unix JSON, sqlx, bcrypt) ; `native/ui` (Qt6/QML : Main + Sidebar + ThemeManager 10 thèmes + 5 pages + NativeBridge) ; 2 units systemd ; `docker-compose.native.yml` ; fix CDC §2.1 ; CI native-build élargie.
- **Setup** : Rust 1.98 installé (rustup, `~/.cargo`).
- **Tests** :
  - `cargo test` → **18/18** (17 unit + 1 intégration socket), `clippy -D warnings` clean, `cargo fmt` clean.
  - Core E2E contre PostgreSQL (seed) via `docker-compose.native.yml` (port 15432) : `apps.list`, `categories.list`, `auth.login` OK.
  - UI : smoke-test **Docker** `debian:bookworm` + Qt6, `QT_QPA_PLATFORM=offscreen` → arbre QML chargé sans erreur, app stable 5 s.
- **Erreurs rencontrées / tentatives** :
  1. `sessionmaker` (Sprint 2 leftover) — corrigé avant Sprint 3.
  2. `tokio-uds` demandé par le plan → obsolète → `tokio` feature `net`. **P8**. 1 tentative.
  3. `docker compose ... native.yml up db` → bind 5432 impossible (PostgreSQL système). **P10**. 2 tentatives (5432 → 15432).
  4. DB auth failed : mot de passe hardcodé ≠ `.env`. Corrigé (lecture `.env`). 2 tentatives.
  5. Sonde IPC `connect('')` : `&` bash applique au groupe AND-list entier → var `$SOCK` perdue. Résolu via `run_in_background` + chemin littéral. 3 tentatives.
  6. `pkill -f target/debug/mjqbe-core` tuait le shell lanceur (self-match). → `pkill -x mjqbe-core`. 2 tentatives.
  7. QML `QtQuick.Window` / `QtQuick.Templates` absents du poste + pas de sudo. **P11**. → retrait import Window + smoke-test dans Docker. 3 tentatives.
  8. `docker run` stdout inaccessible en pipe (snap AppArmor). **P9**. → capture via fichier bind-monté. 3 tentatives.
- **État machine après sprint** : stack Docker restaurée (db en config standard), artefacts de test supprimés, core arrêté, Rust laissé installé (`~/.cargo`).
- **Prochaine étape** : commit `sprint-3: ...`, push `dev` + `sprint-03-actions`, issues GitHub (vérif Pi, réorg QML, smoke-test QML en CI). Puis Sprint 4.

### 2026-08-30 23:36 CEST — Sprint 4 : mode TV + Desktop natif
- **Fait** : core (favorites/settings/recent/session + validation), UI (AppCard favori+iconSize, AppGrid, CategoryChips, Home favoris/récents, AllApps chips+filtre, Search, Settings 3 réglages persistés), outillage smoke-test Docker.
- **Tests** : 27 core (clippy clean) ; E2E DB (session/settings/favorites/recent) ; smoke-test Docker offscreen → QML OK.
- **Erreurs / tentatives** :
  1. Délégués `AppGrid` : `model` (ListModel) vs `modelData` (JS array). Choix = JS array + `modelData` partout. 1 correction.
  2. `import QtQuick.Window` déjà retiré au Sprint 3 → RAS.
- **Différé** : nav télécommande sur écran réel (#137), `QWebEngineView` embarqué, catégories groupées Desktop, réorg QML (#138).
- **État machine** : stack restaurée (db prod), core arrêté, image smoke supprimée.
- **Prochaine étape** : commit `sprint-04:`, push dev + `sprint-04-actions`, issues, puis Sprint 5 (mode Dev natif — monitoring système).

### 2026-08-30 23:48 CEST — Sprint 5 : mode Dev natif
- **Fait** : core `infrastructure/system/` (CPU/RAM/disque/réseau/temp via /proc+sysfs, process kill/nice via libc, docker CLI), tokens de ré-auth usage unique ; UI Dev.qml (gate + monitoring Gauge + process/docker lists + terminal QProcess + dialog ré-auth), Sidebar entrée MJ Dev.
- **Tests** : 37 core (clippy clean) ; E2E réel (snapshot temp 46°C, 14 conteneurs, token usage unique, anti-injection docker id) ; smoke-test Docker OK.
- **Erreurs / tentatives** :
  1. `tokio::process` absent → feature `process` ajoutée. 1 fix.
  2. clippy `sort_by` → `sort_by_key(Reverse(...))`. 1 fix.
- **Différé** : vérif Pi (#137), terminal PTY/ANSI (nouvelle issue), URL VNC configurable (nouvelle issue).
- **État machine** : stack restaurée (db prod), core arrêté, image smoke supprimée.
- **Prochaine étape** : commit `sprint-05:`, push + branche, issues, puis Sprint 6 (UX & animations natives).

### 2026-08-30 23:53 CEST — Sprint 6 : UX & animations natives
- **Fait** : LoadingCube (4 faces rotation Y, pas de Qt3D), overlay de chargement, transitions StackView « cube up » (Y+scale+opacity, layer.enabled transitoire), SidebarButton focusable, build Release.
- **Tests** : build Debug+Release OK ; smoke-test Docker → VmRSS ~50 Mo (< 150 Mo cible) ; 37 tests core inchangés.
- **Différé** : fluidité réelle + heaptrack sur Pi (#137), nav télécommande inter-sections complète, vrai cube 3D shader (nouvelle issue).
- **Prochaine étape** : commit `sprint-06:`, push + branche, issue, puis Sprint 7 (daemon C GPIO — bloc [NATIVE], vérif Pi différée).

### 2026-08-31 00:20 CEST — Sprint 7 : daemon C GPIO
- **Fait** : daemon C (cJSON, sysfs GPIO, stub auto, relais/LED), client Rust (`DaemonClient`+`HardwareService`+IPC token-gated), client Python + endpoints `/dev/gpio|relay|led`, CLI `dev gpio`/`dev relay`.
- **Tests** (Docker, daemon stub) : daemon direct OK ; API endpoints OK (422 sur invalides) ; CLI OK ; Rust core→daemon OK ; 39 tests core ; **CI api-ci de nouveau verte** (P12).
- **Aussi corrigé ce tour** : P12 (api-ci lint flake8 + 4 tests pytest + scrub GitGuardian) — commit séparé `fix(ci)`, CI verte.
- **Erreurs / tentatives** : détection stub daemon trop faible (`/sys/class/gpio` générique) → `should_stub()` device-tree (2 rebuilds) ; BufReader temporaire (clippy) — 1 fix.
- **Différé** : GPIO réel sur Pi (#137) ; protection `/dev/*` → Sprint 10 (#67).
- **Issues fermées** : #42 #43 #44 #45 #46.
- **Prochaine étape** : commit `sprint-07:`, push + branche, fermer issues, puis Sprint 8 (daemon AV : IR/CEC/BT).

### 2026-08-31 00:45 CEST — Sprint 8 : daemon AV (IR/CEC/BT)
- **Fait** : daemon `av.c` (CEC via cec-client, threads IR LIRC + BT UART, ir-map.json, dispatch_action, inject hooks), 5 cmds daemon ; clients Rust/Python `av_status`/`av_cec` ; IPC `av.send` (token) ; `POST/GET /dev/av` ; boutons AV + statut dans Dev.qml.
- **Tests** (Docker stub) : daemon OK (ir_inject/bt_inject/mapping), API OK (422 sur invalide), Rust core→daemon OK, 40 tests core, UI smoke OK.
- **Choix** : CEC via `cec-client` (sous-proc) et non libCEC (linkage) — évite une grosse dép de build.
- **Différé** : IR/CEC/BT réels sur Pi (#137) ; `nav_*` non câblés à une cible UI.
- **Issues fermées** : #47 #48 #49 #50 #51 #52.
- **Prochaine étape** : commit `sprint-08:`, push + branche, fermer issues, puis Sprint 9 (reconnaissance vocale).

### 2026-08-31 01:05 CEST — Sprint 9 : reconnaissance vocale
- **Fait** : grammaire wake-word + commandes FR (13 tests), VoiceService, dispatch CEC/GPIO/launch (find_app ILIKE), IPC voice.*, indicateur Sidebar pulsant, panneau test Dev.qml. Vosk réel = feature cargo `vosk` off (squelette `vosk_engine.rs`).
- **Tests** (Docker stub) : voice.simulate (télé/hub/netflix/inexistant), token gate, voice_disabled ; 53 tests core, clippy clean ; UI smoke OK.
- **Erreurs / tentatives** : fmt (vosk_engine manquant + feature non déclarée + import) 3 fixes ; `cec_send` bloquant → `timeout 6` + client 8 s (1 fix + rebuild).
- **Différé** : capture audio Vosk réelle (libvosk+cpal+ISD1820/micro) + LaunchApp qui n'ouvre rien → nouvelles issues ; test voix réel sur Pi (#137).
- **Issues fermées** : #54 #55 #56 #58 ; #53 + #57 rescopées → nouvelle issue.
- **Prochaine étape** : commit `sprint-09:`, push + branche, issues, puis **Sprint 10 (WEB — authentification JWT + OAuth)**.

### 2026-08-31 01:40 CEST — Sprint 10 : authentification WEB
- **Fait** : bcrypt + JWT HS256 (access/refresh), UserRepository, AuthService (register/login/refresh/oauth_upsert), OAuth Google+GitHub (providers + routes + state), deps FastAPI (get_current_user 401 / require_admin 403), `/dev/*` protégé.
- **Tests** : 15 api verts (11 auth), flake8 clean ; live : login→JWT, /dev sans token→401 / admin→200 / user→403, oauth/github→redirect.
- **Erreurs** : email-validator refuse `.test` → emails de test en `.com` (1 fix).
- **Différé** : `/admin/*` (routes au Sprint 12, #67) ; flux OAuth réseau réel (pas de client id/secret) ; révocation refresh (Sprint 17).
- **Issues fermées** : #59 #60 #61 #62 #63 #64 #65 #66 #68. #67 reste (dépend du Sprint 12).
- **Prochaine étape** : commit `sprint-10:`, push + branche, issues, puis **Sprint 11 (API apps & catégories CRUD)**.

### 2026-08-31 01:50 CEST — Sprint 11 : API apps & catégories
- **Fait** : AppRepository/CategoryRepository, CRUD `/apps` + `/categories` (lecture publique, écriture admin), schémas Pydantic (Mode Literal), 8 tests.
- **Tests** : 23 api verts, flake8 clean ; live OK (filtre mode, guards 401/403, CRUD).
- **Erreurs** : db recréé sans port 15432 → re-`up` override native (1 fix) ; constante 422 dépréciée → littéral.
- **Issues fermées** : #69 #70 #71 #72 #73 #74 #75 #76 #77 #78.
- **Prochaine étape** : commit `sprint-11:`, push + branche, issues, puis **Sprint 12 (settings/favoris/logs + admin système)**.
