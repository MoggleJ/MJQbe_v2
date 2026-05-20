# Sprint Workflow — Procédure d'exécution

Ce fichier décrit la boucle exacte à suivre pour chaque sprint. Les agents doivent la suivre sans dévier.

---

## Boucle de sprint

```
1. INIT
2. DÉVELOPPEMENT
3. COMPARAISON SPECS
4. TESTS
5. CORRECTION
6. PUSH
7. ISSUES
```

---

## Étape 1 — INIT

```bash
# Se placer dans le répertoire de travail
cd /path/to/MJQbe_v2

# Lire les fichiers de contexte
# (obligatoire à chaque début de session ou sprint)
cat CLAUDE.md
cat docs/plan-implementation.md      # identifier le sprint en cours
cat problemes.md                     # prendre en compte les problèmes connus

# S'assurer d'être sur la bonne branche
git checkout dev
git pull origin dev
```

---

## Étape 2 — DÉVELOPPEMENT

- Implémenter toutes les tâches du sprint en cours (cases `[ ]` dans `docs/plan-implementation.md`)
- Cocher chaque tâche dans `docs/plan-implementation.md` au fur et à mesure : `[ ]` → `[x]`
- Si un problème est rencontré : l'ajouter à `problemes.md` immédiatement
- Ne pas passer à l'étape suivante si des tâches critiques sont incomplètes

---

## Étape 3 — COMPARAISON AVEC LES SPECS

Comparer l'implémentation avec `docs/CDC.md` :

```
Pour chaque fonctionnalité du sprint :
  ✓ La fonctionnalité est implémentée ?
  ✓ Elle correspond exactement à la spec ?
  ✓ Pas de dérive (over-engineering, feature manquante) ?
```

Si un écart est trouvé : le corriger avant de continuer ou créer une issue GitHub si c'est hors scope du sprint actuel.

---

## Étape 4 — TESTS

```bash
# Backend
cd api/
pytest tests/ -v

# Frontend
cd frontend/
npm run test

# Docker (vérification globale)
dev up
dev health
```

Tous les tests doivent passer. Si un test échoue → retour à l'étape 2 (correction).

---

## Étape 5 — CORRECTION

- Corriger tous les tests qui échouent
- Si la correction révèle un nouveau problème : l'ajouter à `problemes.md`
- Re-lancer les tests après correction
- Répéter jusqu'à ce que tous les tests passent

---

## Étape 6 — PUSH

```bash
# Identifier le numéro du sprint (ex: 03)
SPRINT="03"

# Commit sur dev
git add .
git commit -m "sprint-${SPRINT}: implement all tasks"
git push origin dev

# Créer et pousser la branche de sprint
git checkout -b sprint-${SPRINT}-actions
git push origin sprint-${SPRINT}-actions

# Retourner sur dev pour la suite
git checkout dev
```

---

## Étape 7 — ISSUES GITHUB

Pour chaque tâche qui n'a pas pu être complétée ou qui a révélé un travail futur :

```bash
# Créer une issue GitHub
gh issue create \
  --title "[Sprint-XX] Titre de la tâche déférée" \
  --body "Description du problème ou de la tâche." \
  --label "sprint-XX,blocked"
```

Mettre à jour `problemes.md` si l'issue est liée à un problème connu.

---

## Commande `dev sprint`

Le CLI intègre une commande `dev sprint` qui guide l'agent à travers ce workflow :

```bash
dev sprint        # Lance le workflow interactif
dev sprint --push # Lance + push automatique si tous les tests passent
```

---

## Règles absolues

- Ne jamais pousser si les tests échouent
- Ne jamais committer de secrets (`.env`, tokens, passwords)
- Toujours mettre à jour `docs/plan-implementation.md` avec les tâches cochées
- Toujours finir sur la branche `dev` après le workflow
