# Problèmes connus et solutions

Source complète : `problemes.md` dans le repo.

---

## P1 — Docker snap : permission denied sur /var/run/docker.sock

**Fix :**
```bash
sudo chown root:docker /var/run/docker.sock
```
Le script `cli/dev` gère ça automatiquement avec `sg docker`.

---

## P2 — Port déjà alloué (Bind failed)

**Fix :**
```bash
docker compose -f docker-compose.yml -f docker-compose.dev.yml down --remove-orphans
# Si ça persiste :
sudo snap restart docker && sudo chown root:docker /var/run/docker.sock
```

---

## P3 — Daemon build : stdio.h not found

**Fix :** Utiliser `build-essential` au lieu de `gcc` dans `daemon/Dockerfile`.

---

## P4 — Docker Compose port merge : double-bind

**Cause :** Docker Compose fusionne les listes de ports des fichiers `-f base -f override`.

**Fix :** Ports frontend uniquement dans `docker-compose.prod.yml` et `docker-compose.dev.yml`, jamais dans `docker-compose.yml`.

---

## P5 — SQLAlchemy : `metadata` est un nom réservé

**Fix :**
```python
meta = Column("metadata", JSONB)  # attribut Python ≠ colonne SQL
```

---

## P6 — Port API hardcodé dans Dockerfile

**Fix :** Shell form CMD + ARG/ENV dans `api/Dockerfile` :
```dockerfile
ARG API_PORT=4848
ENV API_PORT=${API_PORT}
CMD uvicorn app.main:app --host 0.0.0.0 --port ${API_PORT:-4848}
```

---

## P7 — npm ci échoue sans package-lock.json

**Fix :** Utiliser `npm install` au lieu de `npm ci` dans les Dockerfiles frontend.
