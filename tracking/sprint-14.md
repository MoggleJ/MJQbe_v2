# Sprint 14 — Frontend Web : Mode Desktop

**Terminé le** 2026-08-31 (~03:15 CEST) · **Branche** `sprint-14-actions`
**Issues fermées** : #108–#113

## Fait
- **`components/GroupedApps.tsx`** : regroupe les apps par catégorie (Map category_id→name, bucket « Autres »), une `<section class="cat-group">` + `AppGrid` par catégorie, tri alpha.
- **`pages/AllApps.tsx`** : si `mode === 'desktop'` **et** filtre = « Tout » **et** recherche vide → `GroupedApps`, sinon `AppGrid` unique (TV, ou filtre actif).
- **`pages/Home.tsx`** : desktop → `GroupedApps` pour « Toutes les applications » ; section Favoris en tête.
- **`pages/Favorites.tsx`** (nouveau) + route `/favorites` + entrée sidebar « Favoris ».
- **`styles.css`** : `.grid.desktop` (colonnes 84px, gap 12px), `.cat-group`.

## Déjà fait au Sprint 13 (coché ici)
Switch de mode (sidebar), recherche live (`Search`), chips catégories (`AllApps`), toggle favori sur `AppCard`, `useFavorites` branché sur `/favorites`.

## Vérif
`docker build ./frontend` → `tsc && vite build` OK (53 modules). Persistance favoris : `POST/DELETE /favorites` vérifiés en live au Sprint 12.

## Reste
- Section Favoris = filtre côté client sur la liste du mode courant ; OK pour un petit catalogue.
- Parcours navigateur cliqué : non exécuté (idem S13).
