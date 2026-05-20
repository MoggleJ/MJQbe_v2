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
