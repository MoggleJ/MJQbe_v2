# Journal des commandes — MJQbe v2

Historique de toutes les commandes exécutées sur le PC par l'agent.
Format : `HH:MM:SS` — `commande` — _but / résultat_.
cwd par défaut : `/home/mogglej/Documents/Projets_persos/MJQbe/MJQbe_v2` (noté si différent).

---

## 2026-08-30

### Session — reprise de contexte + mise en place tracking

- `22:38:xx` — `find . -maxdepth 3 -type d ...` / `ls -la` — _inventaire structure projet (lecture seule)._
- `22:38:xx` — `git branch -a` / `git log --oneline -20` / `git diff` / `git diff --staged` — _état du dépôt. 3 fichiers modifiés non commités._
- `22:38:xx` — lectures : `CLAUDE.md`, `agents/AGENTS.md`, `agents/sprint-workflow.md`, `docs/CDC.md`, `docs/data-model.md`, `docs/plan-implementation.md`, `problemes.md`, `api/app/main.py`, `api/app/infrastructure/db/seed.py`, entités, `cli/dev`, `config/config.yml`, workflows CI — _reprise de contexte._
- `22:40:23` — `date '+%Y-%m-%d %H:%M:%S %Z'` — _horodatage de référence._
- `22:40:xx` — `docker info` / `id -nG` / `cat .gitignore` / `git status --short` — _vérif accès Docker : OK (accès direct, user dans groupe `docker`)._
- `22:41:xx` — `docker version` / `docker compose version` / `docker ps` — _Docker 29.6.1 ; stack MJQbe déjà UP (frontend:4444, api:4848, db, daemon) — conteneurs `healthy` depuis ~27 min._
- `22:4x:xx` — création `tracking/README.md`, `tracking/journal-commandes.md`, `tracking/suivi-avancement.md`, `tracking/actions-decisives.md` — _mise en place du système de journalisation._
- `22:45:xx` — `git add api/requirements.txt api/app/infrastructure/db/session.py .claude/settings.json && git commit -m "chore: bump api deps ..."` — _commit 9f40bea._
- `22:45:xx` — `git add tracking/ && git commit -m "tracking: mise en place des journaux ..."` — _commit 7649929._
- `22:45:xx` — `git push origin dev` — _db3d669..7649929 poussé sur origin/dev._

### Session — Sprint 3 (app native)

- `22:5x` — `curl https://sh.rustup.rs | sh -s -- -y --profile minimal` — _install Rust 1.98 (user-local `~/.cargo`, `~/.rustup` ; réversible `rustup self uninstall`)._
- `22:5x` — `rustc --version` / `cargo --version` — _1.98.0 OK._
- `22:5x` — `rustc/cmake/qmake6/gcc/pkg-config` checks — _Qt 6.4.2, CMake 3.28.3, gcc 13.3 présents ; Rust absent (installé)._
- `22:5x` — `docker exec mjqbe_v2-db-1 psql -U mjqbe -c "\dt"` — _7 tables présentes (migrations OK)._
- `23:0x` — `mkdir -p native/core/src/{domain,application,...} native/ui/{src,qml/pages}` — _arbo native/._
- `23:0x` — `cargo build` puis `cargo test` (`cd native/core`) — _build OK ; 17 unit + 1 intégration = 18 tests OK._
- `23:0x` — `cmake -S native/ui -B build -DCMAKE_BUILD_TYPE=Debug` puis `cmake --build build` — _binaire `mjqbe-native` OK._
- `23:0x` — `docker compose -f docker-compose.yml -f docker-compose.native.yml up -d db` — _❌ port 127.0.0.1:5432 déjà pris (PostgreSQL système) → P10. Fichier override corrigé → port 15432._
- `23:0x` — `docker compose -f ... -f docker-compose.native.yml up -d db` (port 15432) — _OK, `127.0.0.1:15432` bindé._
- `23:0x` — `psql -h 127.0.0.1 -p 15432 -U mjqbe -d mjqbe -c "select count(*) from apps; ..."` — _19 apps, 1 user (seed OK)._
- `23:0x` — `MJQBE_STUB=1 MJQBE_NATIVE_SOCKET=/tmp/mjqbe-dev.sock DATABASE_URL=... ./target/debug/mjqbe-core` (background task) — _core UP, connecté à PostgreSQL._
- `23:0x` — `python3 scratchpad/ipc_probe.py /tmp/mjqbe-dev.sock` — _ping/health/apps.list/categories.list/auth.login : tous OK contre le seed._
- `23:0x` — `docker build -f .native-smoke.Dockerfile -t mjqbe-native-smoke .` — _image test Qt6 (debian:bookworm) buildée ; build QML OK._
- `23:0x` — `docker run -d -v .smoke-out:/out -v /tmp/mjqbe-dev.sock:/run/mjqbe/native.sock mjqbe-native-smoke sh -c '...'` — _P9 : stdout conteneur inaccessible en pipe (snap) → capture via fichier bind-monté. Verdict : « UI still alive after 5s — QML tree loaded »._
- `23:1x` — `pkill -x mjqbe-core` ; `docker compose -f docker-compose.yml -f docker-compose.prod.yml up -d db` — _core arrêté, `db` restauré en config standard (port interne seul)._
- `23:1x` — `docker rmi mjqbe-native-smoke` ; `rm -rf .smoke-out .native-smoke.Dockerfile` ; `git checkout .gitignore` — _nettoyage artefacts de test._
- `23:1x` — `rustup component add rustfmt clippy` ; `cargo fmt` ; `cargo clippy --all-targets -- -D warnings` — _fmt clean, 0 warning._
- `23:1x` — `git add -A && git commit -m "sprint-03: app native ..."` — _commit 0e0750c (50 fichiers, +4541)._
- `23:1x` — `git push origin dev` — _7649929..0e0750c._
- `23:1x` — `git checkout -b sprint-03-actions && git push -u origin sprint-03-actions && git checkout dev` — _branche de sprint archivée, retour sur dev._
- `23:1x` — `gh issue create` ×3 — _issues #137 (vérif Pi), #138 (réorg QML), #139 (smoke-test QML CI)._

### Session — Sprint 4 (mode TV + Desktop natif)

- `23:2x` — édition core : domain (Settings + traits favorites/settings), application (FavoritesService, SettingsService), infra/db (4 repos), interface/ipc (Services + 6 méthodes).
- `23:2x` — `cargo test` — _26 unit + 1 intégration = 27 OK._ `cargo fmt` + `cargo clippy --all-targets -- -D warnings` — _clean._
- `23:3x` — `docker compose -f docker-compose.yml -f docker-compose.native.yml up -d db` — _db sur 15432._
- `23:3x` — `mjqbe-core` (background) + `python3 scratchpad/ipc_probe4.py /tmp/mjqbe-dev.sock` — _session/settings/favorites/recent E2E OK contre le seed._
- `23:3x` — édition UI : `NativeBridge` (+méthodes/signaux), `AppCard` (favori+iconSize), `AppGrid` + `CategoryChips` (nouveaux), `Main` (état transverse), pages Home/AllApps/Search/Settings réécrites.
- `23:3x` — `cmake --build native/ui/build` — _OK (qmlcachegen tous .qml)._
- `23:3x` — création `native/ui/Dockerfile.smoketest` + `native/ui/smoketest.sh`.
- `23:3x` — `bash native/ui/smoketest.sh /tmp/mjqbe-dev.sock` — _« VERDICT: OK — QML tree loaded »._
- `23:35` — `pkill -x mjqbe-core` ; `docker rmi mjqbe-native-smoke` ; `docker compose ... prod.yml up -d db` — _nettoyage, db restaurée._

### Session — Sprint 5 (mode Dev natif)

- `23:4x` — core : +`infrastructure/system/{metrics,processes,docker}.rs`, +`application/dev.rs`, tokens ré-auth dans `auth.rs`, 8 méthodes IPC.
- `23:4x` — `cargo test` → 36 unit + 1 intégration = 37 OK ; `cargo clippy -D warnings` → 1 fix (`sort_by_key`) → clean ; `cargo fmt`.
- `23:4x` — `cargo add` implicite : `libc`, `rand` + feature tokio `process` (build initial : erreur `tokio::process` absent → feature ajoutée).
- `23:4x` — UI : +`src/TerminalController.{h,cpp}`, +`qml/Gauge.qml`, +`qml/pages/Dev.qml`, `NativeBridge` +8 méthodes, `Sidebar`/`Main` entrée Dev.
- `23:4x` — `cmake -S native/ui -B build && cmake --build build` — OK.
- `23:4x` — core (background) + `python3 scratchpad/ipc_probe5.py /tmp/mjqbe-dev.sock admin` — snapshot/process/docker/verify/token E2E OK.
- `23:47` — `bash native/ui/smoketest.sh /tmp/mjqbe-dev.sock` — « VERDICT: OK ».
- `23:47` — `pkill -x mjqbe-core` ; `docker rmi mjqbe-native-smoke` ; `docker compose ... prod.yml up -d db` — nettoyage.

### Session — Sprint 6 (UX & animations natives)

- `23:5x` — UI : +`qml/LoadingCube.qml`, overlay chargement + transitions `StackView` cube-up dans `Main.qml`, `SidebarButton` focusable.
- `23:5x` — `cmake -S native/ui -B build-release -DCMAKE_BUILD_TYPE=Release && cmake --build build-release` — OK (binaire 700 Ko).
- `23:5x` — `sed -i 's#ui/build/#ui/build*/#' native/.gitignore` — ignore aussi build-release/.
- `23:5x` — `bash native/ui/smoketest.sh` (Docker offscreen, sans core) — « QML tree loaded (VmRSS 49940 kB) » → ~50 Mo < 150 Mo cible.

### Session — Fix CI api-ci + GitGuardian (2026-08-31)

- `00:0x` — `gh run list` / `gh run view 33334393947` — _échec = étape **Lint** (flake8 E221/E501), tests skipped._
- `00:0x` — `python3 -m venv /tmp/flake8venv && flake8 api/app ...` — _reproduit : E221 (alignement `=`) + E501 seed.py/main.py._
- `00:0x` — création `api/setup.cfg`, `api/requirements-dev.txt`, `api/tests/{conftest,test_health,test_seed}.py` ; édition `api-ci.yml`.
- `00:0x` — scrub `postgres://mjqbe:mjqbe@…` → `${POSTGRES_USER}:${POSTGRES_PASSWORD}` dans `docker-compose.native.yml` + `native/README.md` ; `.gitguardian.yaml` étendu.
- `00:0x` — `flake8 app tests` (via `api/setup.cfg`) — _exit 0._
- `00:0x` — `venv + pip install -r api/requirements-dev.txt ; pytest tests/ -v` (contre db Docker :15432) — _4 passed._

### Session — Sprint 7 (daemon C — GPIO) [2026-08-31]

- `00:0x` — réécriture `daemon/main.c` (cJSON, socket JSON/ligne, sysfs GPIO, stub) + `daemon/README.md`.
- `00:0x` — `docker build ./daemon` — OK (warning `strncpy` → `snprintf`).
- `00:0x` — smoke daemon (container `--user`, socket bind-monté) — ping/info/gpio/relay/led + erreurs OK.
- `00:0x` — core : +`infrastructure/hardware/daemon_client.rs`, +`application/hardware.rs`, IPC `hardware.info`/`gpio.get`/`gpio.set`/`relay.set`/`led.set` ; suppression stub `Gpio`.
- `00:0x` — `cargo test` → 39 OK ; `cargo clippy -D warnings` → 1 fix (BufReader temporaire) → clean ; `cargo fmt`.
- `00:0x` — api : +`app/infrastructure/hardware/daemon_client.py`, +`app/interface/routes/dev.py`, `main.py` include_router.
- `00:0x` — `cli/dev` : +`gpio` / +`relay`.
- `00:1x` — `docker compose build daemon api && up -d daemon api` — recréés.
- `00:1x` — bug : daemon détecté « sysfs » sur x86 (`/sys/class/gpio` existe) → 400. Fix `should_stub()` (device-tree « raspberry pi »). Rebuild.
- `00:1x` — tests API : `/dev/hardware`, `POST /dev/gpio|relay|led` (+ header JSON), 422 sur valeurs invalides ; `dev gpio 17 1`, `dev relay 3 0` — OK.
- `00:1x` — E2E Rust core → daemon (stub) : `hardware.info`, `gpio.set` (token), `gpio.get`, `relay.set` — OK.
- `00:2x` — nettoyage (daemon standalone supprimé, db restaurée prod).

### Session — Sprint 8 (daemon AV : IR/CEC/BT) [2026-08-31]

- `00:3x` — +`daemon/av.{c,h}` (CEC via cec-client, threads LIRC + UART, dispatch), +`daemon/ir-map.json` ; `main.c` : extraction `daemon_relay_set`, +5 cmds, `av_init()` ; Makefile +av.c +pthread ; Dockerfile +cec-utils +COPY ir-map.json.
- `00:3x` — `docker build ./daemon` — OK 0 warning ; smoke : av_status/ir_map/cec_send/ir_inject/bt_inject OK (stub).
- `00:3x` — core : `DaemonClient::{cec_send,av_status}`, `HardwareService::{av_cec,av_status}`, IPC `av.status`/`av.send`. `cargo test` → 40 OK, clippy clean.
- `00:3x` — api : `daemon_client.{av_status,av_cec}`, routes `GET/POST /dev/av`. `flake8` OK.
- `00:3x` — UI : `NativeBridge` +avStatus/avSend, `Dev.qml` +boutons AV + statut. `cmake --build` OK.
- `00:4x` — `docker compose build daemon api && up -d` ; `GET /dev/av` `{cec:true,ir:false,bt:false}` ; `POST /dev/av tv_on` → `cec-client failed` (pas d'adaptateur) ; `explode` → 422.
- `00:4x` — E2E Rust core → daemon : `av.status`, `av.send` (token) OK ; UI smoke Docker → QML OK.
- `00:4x` — nettoyage (daemon standalone, image smoke, db prod).

### Session — Sprint 9 (reconnaissance vocale) [2026-08-31]

- `01:0x` — core : +`infrastructure/voice/{mod,grammar,vosk_engine}.rs`, +`application/voice.rs`, `VoiceAction`/`ParsedUtterance`, `CatalogRepository::search_apps`, IPC `voice.status/simulate/set_enabled` + `run_voice_action`. Cargo `[features] vosk = []`.
- `01:0x` — `cargo fmt` (échec initial : `vosk_engine.rs` inexistant → créé ; puis `cfg(feature="vosk")` inconnu → feature déclarée ; puis import `VoiceService` manquant → ajouté). `cargo test` → 53 OK, `clippy -D warnings` clean.
- `01:0x` — UI : `NativeBridge` +voice*, `Main.qml` Timer voix + prop, `Sidebar.qml` indicateur pulsant, `Dev.qml` panneau voix. `cmake --build` OK.
- `01:0x` — E2E : `voice.simulate` (télé→cec, hub→relay, netflix→launch résolu, inexistant→unresolved, set_enabled token + voice_disabled) ; UI smoke OK.
- `01:0x` — bug : `cec_send` bloque (cec-client sans adaptateur) → `daemon timeout`. Fix : `timeout 6 sh -c` dans `av.c` + timeout DaemonClient 3s→8s. Rebuild daemon → cec ~0,5 s.
- `01:0x` — nettoyage (daemon standalone, image smoke, db prod).

### Session — Sprint 10 (WEB : authentification) [2026-08-31]

- `01:2x` — api : +`infrastructure/security/{passwords,tokens}.py`, +`infrastructure/db/user_repo.py`, +`infrastructure/oauth/providers.py`, +`application/auth_service.py`, +`interface/deps.py`, +`interface/routes/auth.py` ; `main.py` réécrit (routers + `/dev` derrière `require_admin`). requirements : +pyjwt +httpx +email-validator.
- `01:2x` — `pip install -r api/requirements-dev.txt` (venv) ; `flake8 app tests` → clean.
- `01:3x` — `pytest tests/ -q` (db Docker :15432) — 3 échecs : `.test` TLD rejeté par email-validator → emails de test en `@example.com`. Re-run → **15 passed**.
- `01:3x` — `docker compose up -d --build api` ; live : `POST /auth/login admin/admin` → JWT ; `/auth/me` OK ; `/dev/hardware` sans token → 401, admin → 200 ; register OK ; openapi 6 routes `/auth`.

### Session — Sprint 11 (WEB : apps & catégories CRUD) [2026-08-31]

- `01:4x` — api : +`infrastructure/db/catalog_repo.py` (AppRepository, CategoryRepository), +`interface/routes/catalog.py` (2 routers, schémas Pydantic), `main.py` include. +`tests/test_catalog.py` (8).
- `01:4x` — `flake8 app tests` clean ; `pytest` — erreurs DB (db recréé en prod sans :15432) → `docker compose ... native.yml up -d db` → **23 passed**.
- `01:4x` — `sed 's/HTTP_422_UNPROCESSABLE_ENTITY/422/'` (constante Starlette dépréciée).
- `01:4x` — `docker compose up -d --build api` ; live : `GET /apps?mode=tv`→7, `POST /apps` 401 sans token / 201 admin, `GET /categories?mode=dev`→3 ; probe app 21 supprimée ; db restauré prod.

### Session — Sprint 12 (WEB : settings/favoris/logs/admin système) [2026-08-31]

- `02:0x` — api : +`infrastructure/db/user_data_repo.py`, +`infrastructure/docker_client.py` (httpx UDS), +`infrastructure/config_file.py`, +`interface/routes/{user_data,admin}.py` ; `deps.py` (+get_optional_user, +verify_reauth) ; `auth_service.py` (_ensure_settings) ; `catalog.py` (log app_launch) ; `main.py` (+2 routers). +`tests/test_user_data.py` (6), +`tests/test_admin.py` (13).
- `02:0x` — `docker-compose.yml` : `./config` + socket Docker → **rw** (retrait `:ro`) pour l'admin panel.
- `02:1x` — `flake8 app tests` clean ; `pytest` → **36 passed** (config testé sur fichier tmp via `monkeypatch CONFIG_PATH`).
- `02:1x` — `docker compose up -d --build api` ; live : settings auto-créés + PUT, favoris, `GET /apps/{id}`→log, `/admin/logs`/`users`/`services`(4 conteneurs via socket)/`config`. Guards 401/403 OK.
- `02:2x` — db restauré prod.

### Session — Sprint 13 (Frontend React + TS) [2026-08-31]

- `02:5x` — frontend : réécriture complète en React 18 + Vite + TS. +`package.json` (router/vitest/testing-library), `tsconfig.json`, `vite.config.ts`, `src/{api,auth,theme,components,pages,test}/*` (~25 fichiers), `styles.css`, `.dockerignore`.
- `03:0x` — `npm install` sur l'hôte → **échec** (node_modules root-owned/vide, pas de sudo). → build/test **via Docker**.
- `03:0x` — `docker build ./frontend` — `tsc && vite build` OK (51 modules). `frontend/.dockerignore` ajouté (le COPY recopiait le node_modules vide de l'hôte).
- `03:0x` — `docker build ./frontend --target builder` + `docker run ... npx vitest run` — **2/2 tests**.
- `03:0x` — `docker compose up -d --build frontend` ; `curl :4444/` (SPA + assets), `/api/health` + `/api/apps?mode=tv` (proxy nginx OK), `/admin` deep-link → 200.
- `03:0x` — nettoyage images ; db prod.

### Session — Sprint 14 (Frontend : mode Desktop) [2026-08-31]
- `03:1x` — frontend : +`components/GroupedApps.tsx`, +`pages/Favorites.tsx` (+route +sidebar), `AllApps`/`Home` groupés par catégorie en mode desktop, `styles.css` (.grid.desktop, .cat-group).
- `03:1x` — `docker build ./frontend` — `tsc && vite build` OK (53 modules).

### Session — Sprint 15 (Frontend : UX & animations) [2026-08-31]
- `03:1x` — frontend : +`components/LoadingCube.tsx`, `styles.css` (.cube-*, cubeUp, .theme-swatch, prefers-reduced-motion), `App.tsx` (LoadingCube si loading), `Settings.tsx` (swatches de prévisu).
- `03:1x` — `docker build ./frontend --target builder` + `npx tsc --noEmit && npx vitest run` — 54 modules, tsc clean, 2/2.
- `03:1x` — `docker compose up -d --build frontend` — SPA re-déployée, `/` + `/api/health` OK.

### Session — Sprint 16 (CLI dev complète) [2026-08-31]
- `03:2x` — `cli/dev` : +`cmd_native`, +`cmd_backup`, +`cmd_restore`, +`cmd_sprint`, +`cmd_install` ; `cmd_health` (+pg_isready +curl /health) ; `cmd_logs` (+`-l` filtre) ; `cmd_help` réécrit ; dispatch. `.gitignore` +`backups/`.
- `03:2x` — `bash -n cli/dev` OK ; `dev help` / `dev health` / `dev backup` (dump 8 Ko) / `dev restore` (auto-y, \dt OK) / `dev logs -l error` — OK.

### Session — Sprint 17 (sécurité / optim / déploiement) [2026-08-31]
- `03:4x` — api : +`interface/security_mw.py` (SecurityHeaders + RateLimit), `main.py` (add_middleware, CORS resserré), `config.yml` (+rate_limit_per_minute), +`tests/test_security.py` (3). `conftest.py` : `AUTH_RATE_LIMIT_PER_MIN=100000`.
- `03:4x` — GitGuardian : `cli/dev` `dev native start` ne compose plus de DATABASE_URL (passe POSTGRES_HOST/PORT) ; `.gitguardian.yaml` +`cli/**`.
- `03:4x` — frontend : +`nginx.https.conf.template`, `Dockerfile` (EXPOSE 80 443, envsubst http/https), `nginx.conf.template` (+headers).
- `03:4x` — +`docs/deploiement.md`, +`docs/revue-finale.md`, +`scripts/loadtest.sh`.
- `03:5x` — `docker compose up -d --build api frontend` ; headers sécu OK (API+nginx) ; 25× login → 20×401 + 5×429 ; loadtest ~35 req/s 0 échec ; `flake8` + `pytest 39/39` ; `/dev/hardware` admin → 200 (pas de régression).
