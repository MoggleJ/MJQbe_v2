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
