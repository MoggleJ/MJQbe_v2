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
