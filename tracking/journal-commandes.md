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
