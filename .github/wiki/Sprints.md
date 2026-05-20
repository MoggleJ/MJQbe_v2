# Sprints — Avancement

Chaque sprint correspond à un milestone GitHub.

## Légende
- ✅ Terminé
- 🚧 En cours
- ⏳ À faire

---

## Priorité : App Native d'abord

| Sprint | Titre | Type | Statut |
|---|---|---|---|
| [Sprint 01](https://github.com/MoggleJ/MJQbe_v2/milestone/1) | Scaffolding & DevOps | Infra | ✅ |
| [Sprint 02](https://github.com/MoggleJ/MJQbe_v2/milestone/2) | Base de données | Infra | ✅ |
| [Sprint 03](https://github.com/MoggleJ/MJQbe_v2/milestone/3) | App Native — Scaffold C++/Qt6 + Rust | NATIVE | 🚧 |
| [Sprint 04](https://github.com/MoggleJ/MJQbe_v2/milestone/4) | App Native — Mode TV + Desktop | NATIVE | ⏳ |
| [Sprint 05](https://github.com/MoggleJ/MJQbe_v2/milestone/5) | App Native — Mode Dev | NATIVE | ⏳ |
| [Sprint 06](https://github.com/MoggleJ/MJQbe_v2/milestone/6) | App Native — UX & Animations | NATIVE | ⏳ |
| [Sprint 07](https://github.com/MoggleJ/MJQbe_v2/milestone/7) | Daemon C — GPIO | Daemon | ⏳ |
| [Sprint 08](https://github.com/MoggleJ/MJQbe_v2/milestone/8) | Daemon C — AV (IR + CEC + BT) | Daemon | ⏳ |
| [Sprint 09](https://github.com/MoggleJ/MJQbe_v2/milestone/9) | Reconnaissance vocale | NATIVE | ⏳ |
| [Sprint 10](https://github.com/MoggleJ/MJQbe_v2/milestone/10) | Authentification [WEB] | WEB | ⏳ |
| [Sprint 11](https://github.com/MoggleJ/MJQbe_v2/milestone/11) | API Apps & Catégories [WEB] | WEB | ⏳ |
| [Sprint 12](https://github.com/MoggleJ/MJQbe_v2/milestone/12) | API Settings, Favorites, Logs & Admin [WEB] | WEB | ⏳ |
| [Sprint 13](https://github.com/MoggleJ/MJQbe_v2/milestone/13) | Frontend Web — Layout, TV & Admin [WEB] | WEB | ⏳ |
| [Sprint 14](https://github.com/MoggleJ/MJQbe_v2/milestone/14) | Frontend Web — Mode Desktop [WEB] | WEB | ⏳ |
| [Sprint 15](https://github.com/MoggleJ/MJQbe_v2/milestone/15) | Frontend Web — UX & Animations [WEB] | WEB | ⏳ |
| [Sprint 16](https://github.com/MoggleJ/MJQbe_v2/milestone/16) | CLI dev — Version complète | Infra | ⏳ |
| [Sprint 17](https://github.com/MoggleJ/MJQbe_v2/milestone/17) | Sécurité, optimisation & déploiement | Final | ⏳ |

---

## Workflow de sprint

Voir `agents/sprint-workflow.md` pour la procédure complète :

```
1. INIT         → checkout dev, lire docs
2. DÉVELOPPEMENT → implémenter les tâches, cocher [ ] → [x]
3. COMPARAISON  → vérifier vs CDC.md
4. TESTS        → pytest, npm test, dev health
5. CORRECTION   → réparer les échecs
6. PUSH         → commit dev + branche sprint-XX-actions
7. ISSUES       → créer issues pour les tâches non complétées
```
