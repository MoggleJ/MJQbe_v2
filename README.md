# MJQbe v2

Hub applicatif embarqué sur Raspberry Pi 4.

## Modes
- **TV** — Interface simplifiée, icônes larges, contenu streamé (Netflix, YouTube, Twitch…)
- **Desktop** — Interface dense, organisation avancée, outils de productivité
- **Dev** — Monitoring système, contrôle matériel, terminal, gestion des serveurs

## Lancement rapide

```bash
docker compose up -d
```

L'interface web est disponible sur `http://localhost:3000`.

## Documentation

| Fichier | Contenu |
|---|---|
| `docs/CDC.md` | Cahier des charges complet |
| `docs/architecture-logicielle.md` | Architecture Docker + Clean Architecture |
| `docs/architecture-materielle.md` | Composants hardware, câblage, pins |
| `docs/data-model.md` | Schéma base de données |
| `docs/plan-implementation.md` | Plan de sprints |
| `agents/AGENTS.md` | Règles de coordination des agents IA |
| `agents/sprint-workflow.md` | Procédure d'exécution de chaque sprint |
| `problemes.md` | Journal des problèmes rencontrés |

## Repository
`https://github.com/MoggleJ/MJQbe_v2.git`
