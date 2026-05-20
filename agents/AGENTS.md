# AGENTS.md — Coordination des agents IA

## Agents actifs

| Agent | Rôle | Outil |
|---|---|---|
| Claude Code | Développement principal, implémentation des sprints | Claude Code CLI |
| GitHub Agents | Review de PR, gestion des issues, CI checks | GitHub Actions / Copilot |

---

## Règles communes à tous les agents

1. **Répertoire de travail :** toujours `MJQbe_v2/`. Ne jamais toucher aux répertoires voisins.
2. **Lecture obligatoire au démarrage :** `docs/CDC.md`, `docs/plan-implementation.md`, `problemes.md`
3. **Problèmes :** tout blocage ou comportement inattendu → entrée dans `problemes.md` avant de continuer
4. **Tests :** chaque sprint doit passer ses tests avant le push
5. **Pas de secrets dans le code :** toutes les credentials passent par `.env` (jamais hardcodées)
6. **Branches :** développer sur `dev`, merger vers `sprint-XX-actions` en fin de sprint

---

## Workflow inter-agents

### Claude Code (agent principal)
- Lit les specs au début de chaque session
- Implémente les tâches du sprint en cours
- Exécute les tests
- Compare le résultat avec `docs/CDC.md`
- Corrige les écarts
- Pousse sur la branche de sprint
- Crée les GitHub Issues pour les tâches déférées

### GitHub Agents
- Review automatique des PR (style, sécurité, performance)
- Vérification des tests CI
- Mise à jour automatique des labels d'issues
- Notification si les tests échouent

---

## Contexte à transmettre entre sessions

Au début d'une nouvelle session, lire dans cet ordre :
1. `CLAUDE.md` — règles générales
2. `docs/plan-implementation.md` — identifier le sprint en cours (dernière tâche cochée)
3. `problemes.md` — prendre en compte les problèmes connus
4. `agents/sprint-workflow.md` — procédure à suivre

---

## Repository GitHub
URL : `https://github.com/MoggleJ/MJQbe_v2.git`

Branches :
- `main` — stable, releases uniquement
- `dev` — développement continu
- `sprint-01-actions` … `sprint-15-actions` — archives de chaque sprint terminé

Issues : utiliser les labels `bug`, `feature`, `blocked`, `sprint-XX`.
