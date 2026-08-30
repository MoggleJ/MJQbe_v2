# Plan d'implémentation — MJQbe v2

Sprints basés sur les tâches à accomplir, pas sur le temps.

Workflow de chaque sprint : voir `agents/sprint-workflow.md`.

**Priorité : l'app native d'abord.**

**Deux interfaces :**
- **[NATIVE]** C++/Qt6/QML (UI) + Rust tokio+sqlx (logique/DB) — TV + Desktop + Dev, local Pi, processus systemd
- **[WEB]** React + FastAPI — TV + Desktop uniquement, accès réseau, OAuth

---

## Sprint 1 — Scaffolding & DevOps ✓

**Objectif :** Poser les fondations du projet. Après ce sprint, tout agent peut cloner le repo et lancer l'environnement.

### Tâches
- [x] Initialiser le repo Git, créer la branche `dev`
- [x] Créer la structure de dossiers complète (`api/`, `frontend/`, `daemon/`, `cli/`, `docs/`, `agents/`)
- [x] Écrire `docker-compose.yml` avec les 4 services : `db`, `api`, `frontend`, `daemon`
- [x] Écrire les `Dockerfile` de base pour chaque service
- [x] Configurer le réseau Docker `mjqbe-network` et les volumes
- [x] Écrire le fichier `.env.example` avec toutes les variables requises
- [x] Ajouter `.gitignore`, `.dockerignore`
- [x] Créer le script CLI `dev` (Bash) avec les commandes : `up`, `down`, `logs`, `restart`, `status`, `db`, `shell`, `update`
- [x] Vérifier que `docker compose up` démarre sans erreur (voir `problemes.md`)

### Livrable de vérification
`dev up` → tous les conteneurs passent en `healthy` ou `running`.

---

## Sprint 2 — Base de données ✓

**Objectif :** Schéma PostgreSQL opérationnel avec données initiales.

### Tâches
- [x] Implémenter le schéma SQL complet (voir `docs/data-model.md`)
- [x] Créer le système de migrations (Alembic)
- [x] Écrire les fichiers de migration initiaux (6 tables, 7 index)
- [x] Écrire des données de seed : catégories, apps par défaut, utilisateur admin (idempotent)
- [x] Ajouter les index recommandés
- [x] Ajouter `dev db` au CLI pour accéder à psql
- [x] Documenter les variables d'environnement PostgreSQL dans `.env.example`

### Livrable de vérification
`dev db` → connexion psql → `\dt` affiche toutes les tables → les données seed sont présentes.

---

## Sprint 3 — App Native — Scaffolding C++/Qt6 + Rust [NATIVE] ✓

**Objectif :** Poser la structure de l'application native. Après ce sprint, la fenêtre s'ouvre sur le Pi.

### Tâches
- [x] Créer `native/ui/` : projet Qt6/QML avec `CMakeLists.txt`
- [x] Créer `native/core/` : crate Rust avec `Cargo.toml` (`sqlx`, `tokio`, `serde`, ~~`tokio-uds`~~ → `tokio` feature `net`, tokio-uds obsolète)
- [x] Définir le protocole IPC : C++ client ↔ Rust serveur via socket Unix JSON (`/run/mjqbe/native.sock`) — JSON lignes, méthodes `ping`/`health`/`apps.list`/`categories.list`/`auth.login`
- [x] Fenêtre principale QML : plein écran (`--windowed` en dev), sidebar 250px + `StackView`
- [x] Implémenter `ThemeManager.qml` (singleton, 10 thèmes, palette amoled par défaut)
- [x] Implémenter `Sidebar.qml` : titre dynamique, menu, heure (`Clock.qml`, `Timer` 1 s)
- [x] Navigation entre pages QML (`StackView`)
- [x] Couche Rust `db/` : connexion PostgreSQL via `sqlx` (requêtes runtime), apps/catégories — vérifié E2E sur le seed
- [x] Authentification locale admin (bcrypt Rust, hash en base) — vérifié E2E (`admin`/`admin` → role admin)
- [x] Stub mode hors Pi : `Platform::detect` (`MJQBE_STUB=1` / device-tree), façade GPIO → `HardwareUnavailable`, DB locale
- [x] Créer `mjqbe-native.service` (+ `mjqbe-core.service`, l'IPC est un process séparé)

### Livrable de vérification
`cmake --build && ./mjqbe-native` → fenêtre s'ouvre, sidebar visible, navigation fonctionne.
**Statut :** build OK (Rust + Qt) ; smoke-test Docker `QT_QPA_PLATFORM=offscreen` → arbre QML chargé sans erreur, app stable ; core testé E2E contre PostgreSQL du seed (18 tests). Vérification sur écran réel / Pi : différée (pas de Pi). Voir `tracking/sprint-3.md`.

---

## Sprint 4 — App Native — Mode TV + Desktop [NATIVE] ✓

**Objectif :** Les deux modes de consommation fonctionnels dans l'app native.

### Tâches
- [x] Implémenter `AppCard.qml` (icône arrondie, taille depuis settings, nom tronqué centré, étoile favori, focus clavier)
- [x] Implémenter `Home.qml` (favoris + récents via IPC `favorites.list` / `apps.recent`)
- [x] Implémenter `AllApps.qml` : `AppGrid` (GridView), `CategoryChips`, filtre texte temps réel
- [x] Mode TV : cellules larges, `keyNavigationWraps` + `Keys` (télécommande)
- [x] Mode Desktop : cellules denses (`AppGrid.mode`)
- [x] Implémenter `Search.qml` : filtre live via IPC
- [x] Favoris : `favorites.toggle` côté Rust, persisté PostgreSQL — vérifié E2E
- [x] `Settings.qml` : thème (10) + layout (grid/list) + icon_size (small/medium/large), persistés (`settings.update`)
- [x] Ouvrir les apps : `Qt.openUrlExternally()` (embedded `QWebEngineView` → sprint ultérieur)
- [x] Couche Rust : use cases `FavoritesService`, `SettingsService`, recent + validation enum

### Livrable de vérification
Mode TV : grille apps visible, navigation clavier/télécommande. Mode Desktop : layout dense. Favoris persistés.
**Statut :** build Rust+Qt OK ; **27 tests** core (clippy clean) ; E2E contre PostgreSQL : `session.current`, `settings.get/update` (+ validation), `favorites.toggle/list` OK ; smoke-test Docker offscreen → arbre QML chargé sans erreur. Nav télécommande sur écran réel + `QWebEngineView` : différés. Voir `tracking/sprint-4.md`.

---

## Sprint 5 — App Native — Mode Dev [NATIVE] ✓

**Objectif :** Dashboard de monitoring et contrôle système dans l'app native.

### Tâches
- [x] `Dev.qml` : gate re-auth admin (`auth.verify`) avant affichage
- [x] Widgets monitoring QML : `Gauge` CPU / RAM / disque / température + débit réseau + load + uptime
- [x] Couche Rust `infrastructure/system/` : `/proc/stat` (CPU %), `/proc/meminfo`, `/proc/net/dev`, `statvfs`, `sysfs` thermal
- [x] Rust : liste process (`/proc/<pid>/status` + `stat`), `kill` / `setpriority` via `libc` (refus pid ≤ 1, mapping EPERM/ESRCH)
- [x] Rust : liste conteneurs Docker via CLI `docker ps -a` (validation d'id anti-injection)
- [x] Terminal QML : `TerminalController` (`QProcess` → `bash -i`, `MergedChannels`) → `TextArea`
- [x] Gestion serveurs : `docker.start` / `docker.stop` (Rust → `docker`)
- [x] Lien interface graphique Pi : bouton → `Qt.openUrlExternally("vnc://…")`
- [x] Re-auth via `Dialog` QML avant toute action destructive (token à usage unique, TTL 120 s)

### Livrable de vérification
Sur Pi : mode Dev accessible après auth admin → stats CPU/RAM en temps réel → terminal fonctionnel.
**Statut :** build Rust+Qt OK ; **37 tests** core (clippy clean) ; E2E : `system.snapshot` (CPU/mem/disk/**temp 46 °C**/net réels), `process.list` (tri RSS réel), `docker.list` (14 conteneurs), `process.kill` sans token → `reauth_required`, `auth.verify` → token usage unique, injection d'id Docker bloquée ; smoke-test Docker offscreen OK. Vérif sur Pi + terminal PTY : différés. Voir `tracking/sprint-5.md`.

---

## Sprint 6 — App Native — UX & Animations [NATIVE] ✓

**Objectif :** Animations GPU et polish de l'interface native.

### Tâches
- [x] Animation chargement : `LoadingCube.qml` (4 faces animées via `transform: Rotation` axe Y — pas de Qt3D, coût minimal) en overlay tant que le core n'est pas connecté
- [x] Transition de page : « cube up » (`replaceEnter`/`replaceExit` du `StackView` — slide Y + scale + opacity, `layer.enabled` le temps de l'anim)
- [x] Optimiser rendu QML : `layer.enabled` transitoire sur les transitions, `clip` seulement sur les vues défilantes, bindings simples
- [x] Profiler mémoire native : mesuré via smoke-test Docker offscreen → **VmRSS ≈ 50 Mo** (debug) — bien sous les 150 Mo. `heaptrack` sur Pi : différé
- [x] Navigation télécommande : `SidebarButton` focusable (`activeFocusOnTab`, anneau de focus), `AppGrid` `keyNavigationWraps` + `Keys` (posé aux sprints 3–4)
- [x] Compiler en release (`cmake -DCMAKE_BUILD_TYPE=Release`) → OK (binaire 700 Ko). Mesure perf sur Pi : différée

### Livrable de vérification
Transitions fluides sur Pi. `htop` montre < 150 Mo RAM pour le process natif.
**Statut :** build Debug + Release OK ; smoke-test Docker offscreen → QML chargé, **VmRSS ≈ 50 Mo**. Fluidité + `heaptrack` + nav télécommande complète sur écran/Pi : différés. Voir `tracking/sprint-6.md`.

---

## Sprint 7 — Daemon C — GPIO [NATIVE] ✓

**Objectif :** Daemon C opérationnel pour contrôle GPIO.

### Tâches
- [x] Daemon C (socket Unix, JSON par ligne, **cJSON** `libcjson`) — `ping`, `info`, dispatch
- [x] `gpio_set`, `gpio_get`, `relay_set` (map relais 1–4 → GPIO 23/24/25/12, actif-bas)
- [x] `led_set` (RGB sur 3 GPIO, pins via `MJQBE_LED_R/_G/_B`)
- [x] Client Rust `native/core/infrastructure/hardware/daemon_client.rs` + `application/hardware.rs` + IPC `hardware.info` / `gpio.get` (ouverts), `gpio.set` / `relay.set` / `led.set` (token de ré-auth)
- [x] Client Python `api/app/infrastructure/hardware/daemon_client.py` — même protocole
- [x] Stub hors-Pi : détection device-tree « raspberry pi » (sinon stub) + `MJQBE_GPIO_STUB` / `MJQBE_GPIO_FORCE`
- [x] Endpoints `POST /dev/gpio`, `POST /dev/relay` (+ `GET /dev/hardware`, `GET /dev/gpio/{pin}`, `POST /dev/led`) — Pydantic
- [x] CLI `dev gpio <pin> <0|1>` + `dev relay <1-4> <0|1>` (→ API → daemon)

### Livrable de vérification
Sur Pi : `dev gpio 23 1` allume le relais 1.
**Statut :** vérifié **via Docker** (daemon en stub) : `dev gpio 17 1` → `{"pin":17,"value":1}` ; `dev relay 3 0` → `{"relay":3,"state":0,"pin":25}` ; chaîne complète Rust core → daemon (`gpio.set` sans token → `reauth_required`, avec token → OK) ; 39 tests core, clippy clean ; api-ci vert. Vérif GPIO réelle sur Pi : différée. Voir `tracking/sprint-7.md`.

---

## Sprint 8 — Daemon C — AV (IR + CEC + Bluetooth) [NATIVE] ✓

**Objectif :** Contrôle TV, PS4 et télécommande IR.

### Tâches
- [x] Réception IR — `daemon/av.c` thread écoutant le socket LIRC (`LIRC_SOCKET`), `sscanf` code/repeat/button
- [x] Mapper les codes IR → actions — `daemon/ir-map.json` (`KEY_POWER→hub_on`, nav…) + fallback intégré ; `dispatch_action` (cec / relais / nav)
- [x] HDMI CEC — via **`cec-client`** (paquet `cec-utils`, pas de linkage libCEC) : `tv_on/tv_off/tv_toggle/ps4_on/ps4_off`
- [x] HC-05 — thread UART (`BT_SERIAL`, 9600 8N1), lignes `TV_ON`/`HUB_ON`/… → mêmes actions
- [x] Boutons AV dans `Dev.qml` (Allume/Éteins TV, PS4 on/off) + statut `cec/ir/bt` — via `av.send` (token) / `av.status`
- [x] Endpoint `POST /dev/av {action}` (+ `GET /dev/av` statut) — Pydantic `Literal`
- [x] Hooks de test hors-Pi : `ir_inject {name}` / `bt_inject {line}` (exécutent le mapping sans matériel)

### Livrable de vérification
Sur Pi : bouton "Allume TV" dans l'app native → TV s'allume via CEC. Télécommande IR → allume le hub.
**Statut :** vérifié **via Docker** (daemon stub) : `ir_inject KEY_POWER` → `hub_on` ; `bt_inject TV_ON` → action `tv_on` ; `POST /dev/av tv_on` → `cec-client failed` (pas d'adaptateur HDMI-CEC sur x86 — attendu) ; chaîne Rust core → daemon (`av.send` sans token → `reauth_required`) ; 40 tests core, clippy clean ; UI smoke OK. IR/CEC/BT réels sur Pi : différés (#137). Voir `tracking/sprint-8.md`.

---

## Sprint 9 — Reconnaissance vocale [NATIVE] ✓

**Objectif :** Wake word + commandes vocales.

### Tâches
- [x] Intégrer Vosk — `infrastructure/voice/vosk_engine.rs` derrière la feature cargo `vosk` (off par défaut : `libvosk` + modèle = uniquement sur le Pi) ; `engine_name()` (`stub`/`vosk`)
- [x] Détection du wake word — `grammar.rs` (`ok hub` / `okay hub` / `ok qube` en tête ou en milieu de phrase)
- [x] Parser les commandes — `map_command` : `allume/éteins la tv|télé`, `démarre la ps4`, `allume le hub`, `lance <app>` — normalisation accents, **13 tests unitaires**
- [x] Connecter aux actions — `Handler::run_voice_action` : `Cec→HardwareService::av_cec`, `Relay→relay_set`, `LaunchApp→CatalogService::find_app` (ILIKE) → URL
- [~] ISD1820 / micro USB continu — capture audio réelle **différée** (nouvelle issue) ; hook `voice.simulate` pour tester le pipeline sans micro
- [x] Indicateur visuel — point dans la `Sidebar` (pulse sur wake récent, `last_wake_secs ≤ 3`), poll `voice.status` (Timer 2 s)

### IPC
`voice.status` (ouvert), `voice.simulate {text}` (ouvert, dev), `voice.set_enabled {token,enabled}` (token de ré-auth).

### Livrable de vérification
Sur Pi : dire "OK hub allume la TV" → TV s'allume.
**Statut :** vérifié **via Docker** (daemon stub) : `voice.simulate "ok hub allume la télé"` → action `cec/tv_on` → daemon ; `"…allume le hub"` → `relay:1=1` ; `"…lance netflix"` → `launch:https://netflix.com` (résolu en base) ; `"…truc inexistant"` → `launch_unresolved` ; `voice.set_enabled` sans token → `reauth_required` ; après désactivation → `voice_disabled` ; 53 tests core, clippy clean ; UI smoke OK. Reconnaissance audio réelle sur Pi : différée. Voir `tracking/sprint-9.md`.

---

## Sprint 10 — Authentification [WEB] ✓

**Objectif :** Système d'auth complet (local + OAuth) avec protection des routes.

### Tâches
- [x] `UserRepository` (`infrastructure/db/user_repo.py`) — get/by_username/by_email/by_oauth/create/touch_last_login/list_all
- [x] `POST /auth/login` — bcrypt (`security/passwords.py`) → paire de tokens JWT HS256 (`security/tokens.py`)
- [x] `POST /auth/register` — compte local (username ≥ 3, password ≥ 8, unicité)
- [x] `POST /auth/refresh` — token `type:refresh` → nouveau `access` (rejette un access token)
- [x] Middleware JWT — `deps.get_current_user` (`HTTPBearer`, `decode_token`), `GET /auth/me`
- [x] OAuth Google — `GET /auth/oauth/google` → redirect ; `/callback` → échange code + userinfo
- [x] OAuth GitHub — idem (`api.github.com/user` + `/user/emails`)
- [x] Rôles `user`/`admin` — `deps.require_admin`
- [x] Protéger `/dev/*` — `include_router(dev, dependencies=[Depends(require_admin)])` ; `/admin/*` : routes créées + protégées au Sprint 12 (#67)
- [x] Tests d'intégration — `tests/test_auth.py` : register/login/me/refresh/guards/OAuth (11 tests)

### Livrable de vérification
Tests passent. `POST /auth/login` avec credentials valides retourne un JWT. Route protégée sans JWT retourne 401.
**Statut :** **15 tests api** verts (4 santé/seed + 11 auth), flake8 clean. Live : `POST /auth/login admin/admin` → JWT ; `GET /dev/hardware` sans token → **401**, avec token admin → **200** ; user non-admin → **403** ; `oauth/github` configuré → redirect vers github. Flux OAuth réel (avec vrais client id/secret) : non testé — creds absents. Voir `tracking/sprint-10.md`.

---

## Sprint 11 — API — Apps & Catégories [WEB] ✓

**Objectif :** CRUD complet pour apps et catégories, filtrage par mode.

### Tâches
- [x] `AppRepository` + `CategoryRepository` (`infrastructure/db/catalog_repo.py`)
- [x] `GET /apps?mode&category_id&include_inactive` — filtré, actifs seuls par défaut
- [x] `GET /apps/:id` — 404 si absent
- [x] `POST /apps` (admin) — 201 ; vérifie `category_id`
- [x] `PUT /apps/:id` (admin) — partiel (`exclude_unset`)
- [x] `DELETE /apps/:id` (admin) — 204
- [x] `GET /categories?mode`
- [x] `POST /categories` (admin, 409 si doublon name+mode), `PUT`/`DELETE /categories/:id` (admin)
- [x] Schémas Pydantic `AppCreate`/`AppUpdate`/`CategoryCreate`/`CategoryUpdate` (`Mode` Literal, longueurs)
- [x] Tests — `tests/test_catalog.py` (8) → **23 api total**

### Livrable de vérification
Tests passent. `GET /apps?mode=tv` retourne les apps TV du seed. `POST /apps` sans token admin retourne 403.
**Statut :** **23 tests api** verts, flake8 clean. Live : `GET /apps?mode=tv` → 7 apps du seed ; `POST /apps` sans token → 401, token user → 403, token admin → 201 ; `GET /categories?mode=dev` → GPIO/Serveurs/Système. Voir `tracking/sprint-11.md`.

---

## Sprint 12 — API — Settings, Favorites, Logs & Admin Système [WEB] ✓

**Objectif :** Données per-user (settings, favoris), logs, et administration système.

### Tâches
- [x] `GET /settings` / `PUT /settings` (thème/layout/icon_size/default_mode, validation enum → 422)
- [x] Création auto des settings à l'inscription — `AuthService._ensure_settings` (register + oauth_upsert)
- [x] `GET /favorites` / `POST /favorites/:app_id` (201, idempotent, 404 app inconnue) / `DELETE /favorites/:app_id`
- [x] Logging `app_launch` — `GET /apps/:id` par un user authentifié → `LogRepository.record("app_launch", uid, {app_id})` (dépendance `get_optional_user`)
- [x] `GET /admin/logs` — pagination `limit`/`offset` + `total`
- [x] `GET /admin/users`
- [x] `GET /admin/config` — lit `config/config.yml`
- [x] `PUT /admin/config` — écrit + valide (`server.web_port`/`api_port` requis) + **re-auth** (mot de passe dans le body)
- [x] `GET /admin/services` — API Docker via socket Unix (`httpx` UDS), filtré sur le projet compose
- [x] `POST /admin/services/:name/restart` / `/stop` — 202
- [x] `POST /admin/reboot` — **re-auth** ; redémarre les autres services puis `api` en différé (hors thread requête)
- [x] Tests — `test_user_data.py` (6) + `test_admin.py` (13, config sur fichier tmp) → **36 api total**

### Livrable de vérification
Tests passent. `PUT /admin/config` modifie `config.yml`. `POST /admin/services/api/restart` redémarre le conteneur.
**Statut :** **36 tests api** verts, flake8 clean. Live : settings auto-créés (theme `dark`) + `PUT` → `light-blue` ; favoris `[4]` ; `GET /apps/{id}` user → log `app_launch` visible dans `/admin/logs` ; `/admin/users` → 27 ; **`/admin/services` liste les 4 conteneurs via le socket Docker** ; `/admin/config` → `web_port 4444`. Guards : non-admin → 403, sans token → 401. `docker-compose.yml` : mount `./config` passé en **rw** + socket Docker (retrait `:ro`). Voir `tracking/sprint-12.md`.
---

## Sprint 13 — Frontend Web — Layout, Mode TV & Admin Panel [WEB] ✓

**Objectif :** Shell React fonctionnel avec sidebar et mode TV complet.

### Tâches
- [x] React 18 + Vite + **TypeScript** (tsconfig strict, `tsc && vite build`)
- [x] Nginx : sert le build + proxy `/api/` → `api:4848/` (déjà dans `nginx.conf.template`) ; `frontend/.dockerignore` ajouté
- [x] 10 thèmes en **variables CSS** (`theme/themes.ts` → `applyTheme` sur `:root`), `UiContext`
- [x] `Sidebar` : titre dynamique (MJ TV/Desktop), menu, switch de mode, `Clock` temps réel, Paramètres, connexion/déconnexion
- [x] Layout `.layout` (sidebar fixe 250px + `.content`)
- [x] Pages `Home` / `AllApps` / `Search` — React Router (`BrowserRouter`)
- [x] Mode TV : `.grid.tv` (colonnes larges), chips catégories
- [x] `AppCard` (icône lettre + nom tronqué + étoile favori)
- [x] Service API `src/api/{client,endpoints}.ts` (fetch, base `/api`)
- [x] Page `Login` : formulaire login/register + boutons OAuth (Google/GitHub)
- [x] JWT client : `localStorage` + **refresh automatique transparent** sur 401 (`client.ts`)
- [x] Responsive : media queries 900 / 640 px (sidebar horizontale en mobile)
- [x] Page `Admin` (onglets) : users, catalogue (CRUD apps/catégories), logs
- [x] `Admin > Système` : `config.yml` éditable (textarea JSON), état services Docker, restart/stop
- [x] Modal de confirmation reboot + re-auth (`Modal` + mot de passe)
- [x] Vitest : `src/test/basic.test.tsx` (thèmes + AppCard) — `npm run test` vert

### Livrable de vérification
`dev up` → `http://localhost:4444` → login → mode TV affiche les apps → admin panel accessible → modifier config.yml via l'UI persiste le fichier.
**Statut :** `tsc && vite build` OK (51 modules) ; **vitest 2/2** (dans Docker) ; **live** : SPA servie sur `:4444` avec assets buildés, `/api/health` et `/api/apps?mode=tv` proxifiés OK, deep-link `/admin` → 200 (fallback nginx). Parcours navigateur (clic login → grille → admin) non cliqué mais toutes les API sous-jacentes sont vérifiées live (Sprints 10-12). Voir `tracking/sprint-13.md`.
---

## Sprint 14 — Frontend Web — Mode Desktop [WEB] ✓

**Objectif :** Mode Desktop avec organisation avancée et recherche.

### Tâches
- [x] Layout Desktop — `GroupedApps` : une section par catégorie (Home + AllApps quand `mode=desktop`, filtre catégorie « Tout », pas de recherche) ; `.grid.desktop` dense (84px) ; `.cat-group`
- [x] Switch TV ↔ Desktop dans la sidebar (`toggleMode`, persisté via `PUT /settings` `default_mode`)
- [x] Page `Search` — filtre temps réel (`useApps(mode)` + `includes`), invite si vide
- [x] Favoris — section dédiée sur `Home` **+ page `/favorites`** ; étoile toggle sur `AppCard` (déjà S13)
- [x] `AllApps` — chips par catégorie (déjà S13)
- [x] Connecter les favoris à l'API — `useFavorites` → `GET/POST/DELETE /favorites` (déjà S13)

### Livrable de vérification
Switch TV/Desktop change le layout. La recherche filtre les apps en temps réel. Les favoris persistent.
**Statut :** `tsc && vite build` OK (53 modules). Desktop = sections par catégorie (`GroupedApps`) ; TV = grille large. Route `/favorites` + entrée sidebar. Favoris persistés en base (vérifié Sprint 12). Voir `tracking/sprint-14.md`.
---

## Sprint 15 — Frontend Web — UX & Animations [WEB] ✓

**Objectif :** Animations et page Settings.

### Tâches
- [x] Animation de chargement — `LoadingCube` : cube CSS 3D (6 faces, `transform-style: preserve-3d`, `@keyframes cubeSpin`), overlay plein écran tant que `useAuth().loading`
- [x] Transition de page — « cube up » : `.page-enter` → `@keyframes cubeUp` (`rotateX(-32deg)` + translate + scale, `perspective: 1400px` sur `.content`), rejouée à chaque changement de route
- [x] Page `Settings` — sélecteur de thème avec **prévisualisation** (`.theme-swatch` : barres bg/surface/accent/text par thème)
- [x] Connecter les settings à l'API — `GET /settings` au login, `PUT /settings` à chaque changement (`UiContext`, déjà S13)
- [x] Appliquer le thème en temps réel — `applyTheme` écrit les variables CSS sur `:root` dans un `useEffect([theme])` (déjà S13)
- [x] `prefers-reduced-motion` respecté (cube ralenti, transitions coupées)

### Livrable de vérification
Naviguer entre pages → animation cube visible. Changer le thème → appliqué instantanément.
**Statut :** `tsc && vite build` OK (54 modules), vitest 2/2, live SPA re-déployée sur `:4444`. Cube CSS 3D + transition `cubeUp` + swatches de prévisu. Thème temps réel via variables CSS. Voir `tracking/sprint-15.md`.
---

## Sprint 16 — CLI `dev` — Version complète ✓

**Objectif :** CLI Bash complète avec toutes les fonctionnalités.

### Tâches
- [x] `dev native <build|start|stop>` — compile (cargo release + cmake release) / lance mjqbe-core (+ mjqbe-native) / arrête
- [x] `dev sprint [--push]` — workflow : `cargo test` + `pytest` (conteneur api) + build/test frontend (Docker) + `dev health` ; push si `--push` et tout vert
- [x] `dev health` — enrichi : + `pg_isready` (connectivité DB) + `curl /health` (API)
- [x] `dev backup` — `pg_dump | gzip` → `backups/mjqbe-<date>.sql.gz`
- [x] `dev restore <file>` — `.sql` ou `.sql.gz` → `psql` (confirmation y/N)
- [x] `dev logs [service] [-l error|warning|info]` — filtre par sévérité (grep -Ei)
- [x] `dev gpio <pin> <val>` — déjà Sprint 7
- [x] `dev install` — `ln -sf` vers `/usr/local/bin/dev` (sudo si besoin)
- [x] `dev help` — toutes les commandes documentées, regroupées par thème

### Livrable de vérification
`dev help` liste toutes les commandes. `dev health` affiche l'état de chaque service.
**Statut :** `bash -n` OK. Vérifié : `dev help` (groupé), `dev health` (services + `PostgreSQL accepte les connexions` + `API /health`), `dev backup` (dump 8 Ko créé), `dev restore` (restauration + `\dt` OK), `dev logs -l error` (filtre). `backups/` gitignoré. Voir `tracking/sprint-16.md`.
---

## Sprint 17 — Sécurité, optimisation & déploiement final

**Objectif :** Audit de sécurité, optimisation mémoire, déploiement production.

### Tâches
- [ ] Audit de sécurité web : CORS, headers HTTP (HSTS, CSP), injection SQL (Pydantic)
- [ ] Rate limiting sur les endpoints d'authentification
- [ ] Profiling mémoire sur Pi 4 : web stack + app native
- [ ] Optimiser les images Docker (multi-stage builds, alpine)
- [ ] Configurer HTTPS (certificat auto-signé ou Let's Encrypt si domaine)
- [ ] Tests de charge légers
- [ ] Documenter la procédure de déploiement production dans `docs/deploiement.md`
- [ ] Créer les GitHub Actions pour CI (tests automatiques sur push)
- [ ] Revue finale vs `docs/CDC.md` (checklist complète)

### Livrable de vérification
Tous les tests passent. Audit OWASP Top 10 coché. `docker stats` + `htop` sur Pi montrent une consommation mémoire acceptable.
