# Suivi d'avancement — MJQbe v2

Journal de tracking : tâches en cours, fichiers impactés, erreurs + tentatives, notes.
Chaque entrée est horodatée.

---

## État global (au 2026-08-30 22:40 CEST)

| Sprint | Statut | Note |
|---|---|---|
| 1 — Scaffolding & DevOps | ✅ terminé | branche `sprint-01-actions` |
| 2 — Base de données | ✅ terminé | branche `sprint-02-actions` ; tests pytest **manquants** (2 issues ouvertes) |
| 3 — Native scaffolding C++/Qt6 + Rust | ⏳ à démarrer | dossier `native/` inexistant |
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
- **Prochaine étape** : commit des modifs non commitées, puis clarification (accès Pi + ordre native/web) avant démarrage Sprint 3.
