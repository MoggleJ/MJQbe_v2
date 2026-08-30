# tracking/ — Journalisation de l'avancement MJQbe v2

Ce dossier contient les journaux tenus par l'agent Claude Code tout au long du projet.
**Règle absolue : chaque entrée est horodatée** (`YYYY-MM-DD HH:MM:SS CEST`).

| Fichier | Contenu |
|---|---|
| [`journal-commandes.md`](journal-commandes.md) | Historique de **toutes** les commandes exécutées sur le PC (cd, docker, sudo, git, build…), style `history`. |
| [`suivi-avancement.md`](suivi-avancement.md) | Suivi d'avancement : ce qui est en cours, fichiers impactés, erreurs rencontrées + nombre de tentatives, notes de tracking. |
| [`actions-decisives.md`](actions-decisives.md) | Actions décisives : interfaces/environnements créés, fichiers modifiés (lignes + raison), config système touchée. |
| `sprint-XX.md` | Créé **à la fin de chaque sprint** : ce qui a été fait, ce qui reste sur cet élément, comment ça fonctionne. |

## Conventions

- Horodatage obtenu via `date '+%Y-%m-%d %H:%M:%S %Z'`.
- `journal-commandes.md` : ordre chronologique, ajout en fin de fichier, une section `## AAAA-MM-JJ` par jour.
- Les commandes de lecture seule triviales et répétées (`cat`, `ls`, `grep` d'inspection) peuvent être regroupées ; toute commande qui **modifie un état** (fichier, conteneur, réseau, paquet, git) est journalisée individuellement.
- `actions-decisives.md` ne consigne que ce qui change l'état du projet ou de la machine de façon non triviale.
