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
