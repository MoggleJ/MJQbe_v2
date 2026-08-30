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
