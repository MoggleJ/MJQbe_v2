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

## Sprint 4 — App Native — Mode TV + Desktop [NATIVE]

**Objectif :** Les deux modes de consommation fonctionnels dans l'app native.

### Tâches
- [ ] Implémenter `AppCard.qml` (icône arrondie 80×80px + nom tronqué centré)
- [ ] Implémenter `Home.qml` (apps récentes / favorites via Rust IPC)
- [ ] Implémenter `AllApps.qml` : `GridView` QML (cellWidth 106, cellHeight 120), chips catégories, filtre temps réel
- [ ] Mode TV : colonnes larges, navigation `KeyNavigation` QML (télécommande)
- [ ] Mode Desktop : layout dense, catégories groupées
- [ ] Implémenter `Search.qml` : filtre live via Rust IPC
- [ ] Favoris : toggle côté Rust, persisté PostgreSQL
- [ ] `Settings.qml` : sélecteur thème (10 options), layout, icon_size
- [ ] Ouvrir les apps : `Qt.openUrlExternally()` ou `QWebEngineView`
- [ ] Couche Rust : use cases favorites, settings, search

### Livrable de vérification
Mode TV : grille apps visible, navigation clavier/télécommande. Mode Desktop : layout dense. Favoris persistés.

---

## Sprint 5 — App Native — Mode Dev [NATIVE]

**Objectif :** Dashboard de monitoring et contrôle système dans l'app native.

### Tâches
- [ ] `Dev.qml` : gate re-auth admin avant affichage
- [ ] Widgets monitoring QML : CPU, RAM, disque, réseau, température
- [ ] Couche Rust `system/` : lecture `/proc/stat`, `/proc/meminfo`, `sysfs` temp
- [ ] Rust : liste des processus (`/proc/<pid>/status`), kill/nice via `libc`
- [ ] Rust : liste des conteneurs Docker (appel CLI `docker ps` ou API Unix socket)
- [ ] Terminal QML : `QProcess` → bash, texte dans `TextArea` scrollable
- [ ] Gestion serveurs : démarrer/arrêter conteneurs (Rust → `docker start/stop`)
- [ ] Lien interface graphique Pi : `Qt.openUrlExternally` vers VNC ou `startx`
- [ ] Re-auth via dialog QML avant toute action destructive

### Livrable de vérification
Sur Pi : mode Dev accessible après auth admin → stats CPU/RAM en temps réel → terminal fonctionnel.

---

## Sprint 6 — App Native — UX & Animations [NATIVE]

**Objectif :** Animations GPU et polish de l'interface native.

### Tâches
- [ ] Animation chargement : cube MJQbe 3D (`ShaderEffect` QML + OpenGL ES ou `Qt3D`)
- [ ] Transition de page : rotation cube vers le haut (`SequentialAnimation` + `RotationAnimation`)
- [ ] Optimiser rendu QML : `layer.enabled`, éviter bindings inutiles, `clip: false` là où possible
- [ ] Profiler mémoire native : `valgrind` / `heaptrack` → cible < 150 Mo RAM
- [ ] Navigation télécommande complète (`KeyNavigation` QML sur toutes les pages)
- [ ] Compiler en release (`cmake -DCMAKE_BUILD_TYPE=Release`) et mesurer perf sur Pi

### Livrable de vérification
Transitions fluides sur Pi. `htop` montre < 150 Mo RAM pour le process natif.

---

## Sprint 7 — Daemon C — GPIO [NATIVE]

**Objectif :** Daemon C opérationnel pour contrôle GPIO.

### Tâches
- [ ] Écrire le daemon C (lecture socket Unix, JSON parsing avec cJSON)
- [ ] Implémenter : `gpio_set`, `gpio_get`, `relay_set` (contrôle relais)
- [ ] Implémenter : `led_set` (RGB si applicable)
- [ ] Client Rust dans `native/core/infrastructure/hardware/` pour communiquer avec le daemon
- [ ] Client Python dans l'API (`infrastructure/hardware/`) — même interface
- [ ] Stub pour développement hors Pi
- [ ] Endpoints API : `POST /dev/gpio`, `POST /dev/relay`
- [ ] Ajouter `dev gpio <pin> <val>` au CLI

### Livrable de vérification
Sur Pi : `dev gpio 23 1` allume le relais 1.

---

## Sprint 8 — Daemon C — AV (IR + CEC + Bluetooth) [NATIVE]

**Objectif :** Contrôle TV, PS4 et télécommande IR.

### Tâches
- [ ] Implémenter réception IR (LIRC ou lecture directe GPIO18)
- [ ] Mapper les codes IR aux actions (allumage hub, navigation)
- [ ] Implémenter HDMI CEC via libCEC : `tv_on`, `tv_off`, `ps4_on`, `ps4_off`
- [ ] Implémenter communication HC-05 (UART, parse commandes BT)
- [ ] Boutons de contrôle AV dans le mode Dev natif
- [ ] Endpoints API web : `POST /dev/av`

### Livrable de vérification
Sur Pi : bouton "Allume TV" dans l'app native → TV s'allume via CEC. Télécommande IR → allume le hub.

---

## Sprint 9 — Reconnaissance vocale [NATIVE]

**Objectif :** Wake word + commandes vocales.

### Tâches
- [ ] Intégrer Vosk (offline, léger) pour reconnaissance vocale
- [ ] Implémenter détection du wake word (ex: "OK hub")
- [ ] Parser les commandes : "allume la TV", "éteins la TV", "lance Netflix"
- [ ] Connecter les commandes aux actions existantes (CEC, GPIO)
- [ ] Lier avec ISD1820 (déclenchement GPIO) ou micro USB en continu
- [ ] Indicateur visuel dans l'app native quand le wake word est détecté

### Livrable de vérification
Sur Pi : dire "OK hub allume la TV" → TV s'allume.

---

## Sprint 10 — Authentification [WEB]

**Objectif :** Système d'auth complet (local + OAuth) avec protection des routes.

### Tâches
- [ ] Implémenter les entités `User` et le repository dans la couche domain/infrastructure
- [ ] Implémenter l'endpoint `POST /auth/login` (username + password, bcrypt, retourne JWT)
- [ ] Implémenter `POST /auth/register` (création de compte local)
- [ ] Implémenter le refresh token (`POST /auth/refresh`)
- [ ] Implémenter le middleware JWT (vérification sur toutes les routes protégées)
- [ ] Implémenter OAuth Google : `GET /auth/oauth/google` + callback
- [ ] Implémenter OAuth GitHub : `GET /auth/oauth/github` + callback
- [ ] Implémenter la gestion des rôles (`user` / `admin`)
- [ ] Protéger les routes admin (`/admin/*`, `/dev/*`)
- [ ] Écrire les tests d'intégration pour login, register, OAuth callback, JWT validation

### Livrable de vérification
Tests passent. `POST /auth/login` avec credentials valides retourne un JWT. Route protégée sans JWT retourne 401.

---

## Sprint 11 — API — Apps & Catégories [WEB]

**Objectif :** CRUD complet pour apps et catégories, filtrage par mode.

### Tâches
- [ ] Implémenter les entités `App` et `Category` (domain + infrastructure)
- [ ] `GET /apps` — liste filtrée par mode, catégorie (query params)
- [ ] `GET /apps/:id` — détail d'une app
- [ ] `POST /apps` — créer une app (admin)
- [ ] `PUT /apps/:id` — modifier (admin)
- [ ] `DELETE /apps/:id` — supprimer (admin)
- [ ] `GET /categories` — liste par mode
- [ ] `POST /categories`, `PUT /categories/:id`, `DELETE /categories/:id` (admin)
- [ ] Validation des inputs (Pydantic schemas)
- [ ] Écrire les tests pour chaque endpoint

### Livrable de vérification
Tests passent. `GET /apps?mode=tv` retourne les apps TV du seed. `POST /apps` sans token admin retourne 403.

---

## Sprint 12 — API — Settings, Favorites, Logs & Admin Système [WEB]

**Objectif :** Données per-user (settings, favoris), logs, et administration système.

### Tâches
- [ ] `GET /settings` — settings de l'utilisateur connecté
- [ ] `PUT /settings` — mise à jour (thème, layout, icon_size, default_mode)
- [ ] Création automatique des settings à l'inscription (hook post-register)
- [ ] `GET /favorites` — favoris de l'utilisateur connecté
- [ ] `POST /favorites/:app_id` — ajouter un favori
- [ ] `DELETE /favorites/:app_id` — retirer un favori
- [ ] Middleware de logging : enregistrer `app_launch` à chaque `GET /apps/:id` par un user connecté
- [ ] `GET /admin/logs` — liste des logs (admin, avec pagination)
- [ ] `GET /admin/users` — liste des users (admin)
- [ ] `GET /admin/config` — lire `config/config.yml` (admin)
- [ ] `PUT /admin/config` — écrire `config/config.yml` + valider la structure (admin)
- [ ] `GET /admin/services` — état de chaque service Docker (via socket Docker)
- [ ] `POST /admin/services/:name/restart` — redémarrer un service Docker (admin)
- [ ] `POST /admin/services/:name/stop` — arrêter un service (admin)
- [ ] `POST /admin/reboot` — redémarrer tous les services (admin, re-auth obligatoire)
- [ ] Écrire les tests

### Livrable de vérification
Tests passent. `PUT /admin/config` modifie `config.yml`. `POST /admin/services/api/restart` redémarre le conteneur.

---

## Sprint 13 — Frontend Web — Layout, Mode TV & Admin Panel [WEB]

**Objectif :** Shell React fonctionnel avec sidebar et mode TV complet.

### Tâches
- [ ] Initialiser le projet React 18 + Vite
- [ ] Configurer Nginx pour servir le build et proxifier `/api` → `api:4848`
- [ ] Implémenter le système de thèmes (10 thèmes CSS via variables)
- [ ] Créer le composant `Sidebar` (titre dynamique, menu, heure temps réel, settings)
- [ ] Créer le layout principal (sidebar fixe + contenu)
- [ ] Implémenter les pages : `Home`, `AllApps`, `Search`
- [ ] Implémenter le mode TV : grille d'apps larges, catégories visibles
- [ ] Créer le composant `AppCard` (icône + nom)
- [ ] Connecter les appels API (service `apps`, `categories`)
- [ ] Implémenter la page de login (formulaire + OAuth buttons)
- [ ] Gérer le JWT côté client (stockage, refresh automatique)
- [ ] Responsive : TV (1920px), desktop (1280px), tablette (768px), mobile (375px)
- [ ] Page `Admin` : liste des users, gestion apps/catégories
- [ ] Page `Admin > Système` : affichage config.yml éditable (formulaire), état des services Docker
- [ ] Boutons restart par service + bouton "Reboot tout" (avec modal de confirmation + re-auth)

### Livrable de vérification
`dev up` → `http://localhost:4444` → login → mode TV affiche les apps → admin panel accessible → modifier config.yml via l'UI persiste le fichier.

---

## Sprint 14 — Frontend Web — Mode Desktop [WEB]

**Objectif :** Mode Desktop avec organisation avancée et recherche.

### Tâches
- [ ] Implémenter le layout Desktop (organisation dense, catégories groupées)
- [ ] Implémenter le switch de mode TV ↔ Desktop dans la sidebar
- [ ] Implémenter la recherche (`/search`) : filtre temps réel sur les apps
- [ ] Implémenter les favoris : affichage section dédiée, bouton toggle sur AppCard
- [ ] Implémenter `AllApps` avec filtres par catégorie
- [ ] Connecter les favoris à l'API

### Livrable de vérification
Switch TV/Desktop change le layout. La recherche filtre les apps en temps réel. Les favoris persistent.

---

## Sprint 15 — Frontend Web — UX & Animations [WEB]

**Objectif :** Animations et page Settings.

### Tâches
- [ ] Implémenter l'animation de chargement : cube MJQbe 3D en rotation (CSS 3D)
- [ ] Implémenter la transition de page : rotation cube vers le haut (CSS 3D transform)
- [ ] Implémenter la page `Settings` : sélecteur de thème (10 options avec prévisualisation), layout, icon_size
- [ ] Connecter les settings à l'API (`GET /settings`, `PUT /settings`)
- [ ] Appliquer le thème en temps réel sans rechargement

### Livrable de vérification
Naviguer entre pages → animation cube visible. Changer le thème → appliqué instantanément.

---

## Sprint 16 — CLI `dev` — Version complète

**Objectif :** CLI Bash complète avec toutes les fonctionnalités.

### Tâches
- [ ] Ajouter `dev native` : démarrer/arrêter l'app native
- [ ] `dev sprint` : exécute le workflow complet de sprint
- [ ] `dev health` : vérifie l'état de tous les services + connectivité DB
- [ ] `dev backup` : backup PostgreSQL vers fichier daté
- [ ] `dev restore <file>` : restaure un backup
- [ ] `dev logs` avec filtre par niveau (error, warning, info)
- [ ] `dev gpio <pin> <val>` : contrôle GPIO direct
- [ ] Installer le script en tant que commande système (`/usr/local/bin/dev`)
- [ ] Documenter chaque commande (`dev help`)

### Livrable de vérification
`dev help` liste toutes les commandes. `dev health` affiche l'état de chaque service.

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
