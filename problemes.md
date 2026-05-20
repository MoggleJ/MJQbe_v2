# problemes.md — Journal des problèmes

Ce fichier est maintenu par les agents IA. Chaque problème bloquant ou inattendu doit être enregistré ici avec sa solution.

---

## Format

```
## [SPRINT-XX] Titre du problème

### Problème
Description précise du problème rencontré.

### Contexte
Fichier(s) concerné(s), commande lancée, message d'erreur exact.

### Solution
Ce qui a résolu le problème.

### Statut
- [ ] En cours
- [x] Résolu
```

---

<!-- Les agents ajoutent leurs entrées ci-dessous -->

## [SPRINT-01] Accès Docker sans sudo

### Problème
`docker compose up` échoue avec `permission denied while trying to connect to the Docker daemon socket at unix:///var/run/docker.sock`. L'utilisateur `mogglej` n'est pas dans le groupe `docker`.

### Contexte
Commande : `docker compose -f docker-compose.yml up -d --build`
Erreur : `unable to get image 'postgres:15-alpine': permission denied while trying to connect to the docker API at unix:///var/run/docker.sock`

### Solution
```bash
sudo usermod -aG docker $USER
# Se déconnecter puis se reconnecter (ou newgrp docker)
newgrp docker
# Relancer
dev up
```

### Statut
- [ ] En cours (action manuelle requise de l'utilisateur)
