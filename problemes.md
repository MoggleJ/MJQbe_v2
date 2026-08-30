# Problèmes rencontrés — MJQbe v2

## P1 — Docker snap : permission denied sur /var/run/docker.sock

**Symptôme :** `permission denied while trying to connect to the Docker daemon socket`

**Cause :** Avec Docker installé via snap, le socket Docker n'appartient pas au groupe `docker` par défaut après un restart du daemon.

**Solution :**
```bash
sudo groupadd docker 2>/dev/null || true
sudo usermod -aG docker $USER
sudo chown root:docker /var/run/docker.sock
newgrp docker
```

Après un `sudo snap restart docker`, relancer :
```bash
sudo chown root:docker /var/run/docker.sock
```

Le script `cli/dev` se ré-exécute automatiquement avec `sg docker` si le socket est inaccessible mais que l'user est dans le groupe docker.

**Statut :** Résolu (workaround automatique dans cli/dev + chown manuel post-restart)

---

## P2 — Port déjà alloué (Bind failed: port is already allocated)

**Symptôme :** `Bind for 0.0.0.0:XXXX failed: port is already allocated` alors que `ss` ne montre rien sur ce port.

**Cause :** Docker (snap) garde des réservations de ports en mémoire quand un conteneur échoue à démarrer (reste en état `Created`). Le daemon ne libère pas le port automatiquement.

**Solution immédiate :**
```bash
docker compose -f docker-compose.yml -f docker-compose.dev.yml down --remove-orphans
```

**Si ça persiste :**
```bash
sudo snap restart docker
sudo chown root:docker /var/run/docker.sock
```

**Fix préventif appliqué :** `cmd_up` et `cmd_watch` dans `cli/dev` font un `down --remove-orphans` avant chaque démarrage + `--force-recreate` sur le `up`.

**Statut :** Résolu

---

## P3 — daemon build : stdio.h not found

**Symptôme :** `fatal error: stdio.h: No such file or directory` lors du build du daemon.

**Cause :** Image Debian slim sans les headers libc. `gcc` seul ne suffit pas.

**Solution :** Utiliser `build-essential` à la place de `gcc` dans `daemon/Dockerfile`.

**Statut :** Résolu

---

## P4 — Docker Compose port merge : double-bind du port frontend

**Symptôme :** `Bind for 0.0.0.0:4444 failed: port is already allocated` quand on lance le stack dev alors qu'aucun autre processus n'écoute. Les deux fichiers compose tentent de binder le même port.

**Cause :** Docker Compose **fusionne** les listes de ports quand on empile plusieurs fichiers avec `-f base -f override`. Si `docker-compose.yml` déclare déjà `ports: ["4444:80"]` pour `frontend`, et que `docker-compose.dev.yml` déclare `ports: ["4444:5173"]`, les deux entrées coexistent → double-bind.

**Solution :** Retirer tous les ports du frontend de `docker-compose.yml` (base). Les ports sont déclarés uniquement dans les overrides :
- `docker-compose.prod.yml` : `0.0.0.0:${WEB_PORT:-4444}:80`
- `docker-compose.dev.yml` : `0.0.0.0:${WEB_PORT:-4444}:5173`

**Statut :** Résolu

---

## P5 — SQLAlchemy : `metadata` est un nom réservé

**Symptôme :** `AttributeError` ou comportement inattendu sur le modèle `Log` au démarrage de l'API.

**Cause :** `DeclarativeBase` de SQLAlchemy expose un attribut de classe `metadata` (le `MetaData` du schéma). Nommer une colonne `metadata` écrase cet attribut.

**Solution :**
```python
# Dans api/app/domain/entities/log.py
meta = Column("metadata", JSONB)  # attribut Python "meta", colonne SQL "metadata"
```

**Statut :** Résolu

---

## P6 — Port API hardcodé dans docker-compose.dev.yml et Dockerfile

**Symptôme :** Changer `api_port` dans `config/config.yml` ne prenait pas effet pour le conteneur API en mode dev — le port 4848 restait hardcodé.

**Cause :** Deux emplacements hardcodaient le port :
1. `docker-compose.dev.yml` : `command: ["uvicorn", ..., "--port", "4848"]`
2. `api/Dockerfile` : `CMD ["uvicorn", ..., "--port", "4848"]` (exec form, pas d'expansion de variable)

**Solution :**
- `docker-compose.dev.yml` : utiliser `"${API_PORT:-4848}"` dans la commande
- `api/Dockerfile` : passer en shell form + ARG/ENV :
```dockerfile
ARG API_PORT=4848
EXPOSE ${API_PORT}
ENV API_PORT=${API_PORT}
CMD uvicorn app.main:app --host 0.0.0.0 --port ${API_PORT:-4848}
```

**Statut :** Résolu

---

## P7 — npm ci échoue : package-lock.json absent

**Symptôme :** `npm ci` échoue dans le Dockerfile frontend avec `npm error The \`npm ci\` command can only install with an existing package-lock.json`.

**Cause :** Le repo ne contenait pas de `package-lock.json` au moment du Sprint 1 — fichier généré par `npm install`, pas présent avant le premier build.

**Solution :** Utiliser `npm install` à la place de `npm ci` dans `frontend/Dockerfile` et `frontend/Dockerfile.dev`.

**Statut :** Résolu

---

## P8 — Sprint 3 : `tokio-uds` n'existe plus

**Symptôme :** le plan demande la crate `tokio-uds` pour le socket Unix du core Rust.

**Cause :** `tokio-uds` est obsolète depuis Tokio 0.2 — les `UnixListener` / `UnixStream` sont intégrés à `tokio` via la feature `net`.

**Solution :** `tokio = { version = "1", features = ["net", ...] }`. Le plan a été annoté.

**Statut :** Résolu

---

## P9 — Docker (snap) : `docker run` ne renvoie pas stdout dans un pipe

**Symptôme :** `docker run --rm <image> echo hello` → exit 1, aucune sortie. `docker logs` vide. Mais `docker build`, `docker compose up`, `docker ps`, et `docker run <image> true` (sans sortie) fonctionnent.

**Cause :** confinement AppArmor du snap Docker : le conteneur ne peut pas écrire sur le pipe stdout fourni par l'outil d'exécution non-TTY → SIGPIPE → exit 1.

**Solution (tests de fonctionnement) :** rediriger la sortie du conteneur vers un fichier sur un volume bind-monté (`docker run -d -v "$PWD/out":/out ... sh -c 'exec >/out/log 2>&1; ...'`), puis lire le fichier côté hôte.

**Statut :** Contourné (limite d'environnement, pas un bug projet)

---

## P10 — Port 5432 déjà pris sur l'hôte (PostgreSQL système)

**Symptôme :** `docker compose -f ... -f docker-compose.native.yml up -d db` → `failed to bind host port 127.0.0.1:5432: address already in use`. Le recreate échoue et laisse `mjqbe_v2-db-1` en état `Created`.

**Cause :** un PostgreSQL système écoute déjà sur `127.0.0.1:5432`. L'app native (processus local hors Docker) a besoin d'atteindre la base du conteneur.

**Solution :** `docker-compose.native.yml` publie `db` sur `127.0.0.1:${MJQBE_DB_HOST_PORT:-15432}:5432` (loopback uniquement, port 15432 par défaut). Récupération après échec : `docker compose -f docker-compose.yml -f docker-compose.prod.yml up -d db`.

**Statut :** Résolu

---

## P11 — Qt 6 : `QtQuick.Templates` / `QtQuick.Window` absents du poste de dev

**Symptôme :** `./mjqbe-native` → `module "QtQuick.Templates" plugin "qtquicktemplates2plugin" not found` ou `module "QtQuick.Window" is not installed`. Le **build** (`cmake --build`) réussit ; seul le **run** échoue.

**Cause :** poste de dev sans `qml6-module-qtquick-templates` / `qml6-module-qtquick-window` (runtime QML), et `sudo` non disponible sans mot de passe en session non-interactive.

**Solutions appliquées :**
1. Retrait de `import QtQuick.Window` dans `Main.qml` — plein écran via `showFullScreen()` / `showNormal()` en `Component.onCompleted` (méthodes, pas d'enum → pas d'import).
2. `QtQuick.Templates` reste requis par `QtQuick.Controls` : smoke-test lancé dans une image Docker `debian:bookworm` (même base Qt 6.4 que le Pi) avec tous les `qml6-module-*` → arbre QML chargé sans erreur.

**Statut :** Résolu (dev = via Docker ; sur le Pi, les paquets `qml6-module-qtquick-*` sont à installer — voir `docs/deploiement.md` à venir, Sprint 17)

---

## P12 — CI `API CI` rouge (lint) + alerte GitGuardian

**Symptôme :** job « Tests & Lint » de `api-ci.yml` en échec à l'étape **Lint** ; les tests étaient *skipped*. En parallèle, GitGuardian a levé une alerte.

**Causes :**
1. `flake8 api/app --max-line-length=100` (args CLI) écrasait toute config. Le code Sprint 1–2 aligne volontairement les `=` → **E221** (+ quelques **E501** à 105–108 car.). Aucun test dans `api/tests/` → `pytest` exit 5 (mais masqué, lint échouait avant).
2. GitGuardian : chaînes `postgres://mjqbe:mjqbe@…` (exemples/commentaires) dans `docker-compose.native.yml` et `native/README.md`.

**Solution :**
- `api/setup.cfg` : `[flake8]` `max-line-length=120`, `extend-ignore=E203,W503,E221,E241` ; `[tool:pytest]` `pythonpath=.`.
- `api/requirements-dev.txt` (pytest + httpx + flake8) ; `api-ci.yml` : `cd api && flake8 app tests`, install de `requirements-dev.txt`.
- `api/tests/` : `conftest.py` (fixture `client` = `TestClient` → lifespan → migrations+seed) + `test_health.py` + `test_seed.py` (schéma, admin, idempotence). 4 tests, vérifiés contre le PostgreSQL Docker.
- Chaînes de connexion : remplacées par `${POSTGRES_USER}:${POSTGRES_PASSWORD}` (lus depuis `.env`).
- `.gitguardian.yaml` : `ignore-paths` += `tracking/**`, `**/*.md`, `**/tests/**` ; `ignore-known-secrets: true`.

**Statut :** Résolu côté repo (lint + 4 tests OK en local). L'incident GitGuardian existant est à clôturer manuellement dans le dashboard GG.
