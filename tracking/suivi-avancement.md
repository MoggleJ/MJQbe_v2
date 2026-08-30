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
| 4 → 9 — Native (TV/Desktop/Dev/UX/daemon/AV/voix) | ⛔ non démarré | dépendances matériel Pi pour 5→9 |
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
