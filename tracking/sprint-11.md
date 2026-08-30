# Sprint 11 — API : Apps & Catégories (WEB)

**Terminé le** 2026-08-31 (~01:50 CEST) · **Branche** `sprint-11-actions`
**Issues fermées** : #69, #70, #71, #72, #73, #74, #75, #76, #77, #78

---

## 1. Ce qui a été fait

| Fichier | Rôle |
|---|---|
| `infrastructure/db/catalog_repo.py` | `AppRepository` (get / list(mode, category_id, include_inactive) / create / update / delete) ; `CategoryRepository` (get / list(mode) / find(name, mode) / create / update / delete) |
| `interface/routes/catalog.py` | 2 routers. **Lectures publiques**, **écritures `Depends(require_admin)`**. Schémas Pydantic `AppOut/AppCreate/AppUpdate` + `CategoryOut/CategoryCreate/CategoryUpdate` (type `Mode = Literal["tv","desktop","dev"]`, bornes de longueur). |
| `main.py` | `include_router(apps_router)` + `include_router(categories_router)` |

### Endpoints

| Méthode | Route | Accès | Notes |
|---|---|---|---|
| GET | `/apps?mode&category_id&include_inactive` | public | actifs seuls par défaut, tri `mode, name` |
| GET | `/apps/{id}` | public | 404 si absent |
| POST | `/apps` | admin | 201 ; `category_id` inconnu → 422 |
| PUT | `/apps/{id}` | admin | partiel (`model_dump(exclude_unset=True)`) |
| DELETE | `/apps/{id}` | admin | 204 |
| GET | `/categories?mode` | public | |
| POST | `/categories` | admin | 409 si `name`+`mode` déjà pris |
| PUT / DELETE | `/categories/{id}` | admin | 204 sur delete (apps → `category_id` NULL via FK) |

---

## 2. Vérifications

### `pytest` (contre PostgreSQL Docker) — **23 passed** (+8 vs Sprint 10)
`tests/test_catalog.py` :
- `GET /apps?mode=tv` → ≥ 1, tous `mode=tv`, contient Netflix
- `GET /apps/{id}` OK ; `/apps/99999999` → 404
- `GET /categories?mode=desktop` → tous `mode=desktop`
- `POST /apps` sans token → 401 ; token **user** → 403
- CRUD admin complet : create 201 → update (is_active=false) → **caché** de la liste par défaut, **visible** avec `include_inactive` → delete 204 → get 404
- `POST /apps` `category_id` bidon → 422 ; `mode:"holodeck"` → 422 (Pydantic)
- Catégorie : create 201 → doublon 409 → rename 200 → delete 204

### Live (stack Docker)
- `GET /apps?mode=tv` → 7 apps du seed
- `POST /apps` sans token → 401 ; token admin → 201 (probe supprimée ensuite)
- `GET /categories?mode=dev` → `GPIO / Serveurs / Système`
- flake8 clean

---

## 3. Reste à faire
- Rien de bloquant. `GET /apps` ne pagine pas (catalogue petit — ~20 lignes) ; à ajouter si besoin.
- Le natif (`native/core`) a sa propre couche apps/catégories (sqlx) — les deux interfaces partagent la **base**, pas le code (conforme CDC : point d'entrée unique par couche).

## 4. Comment ça fonctionne
`GET /apps?mode=tv` → `list_apps` → `AppRepository.list` → `SELECT … WHERE mode='tv' AND is_active ORDER BY mode,name` → `list[AppOut]` (`from_attributes`).
`POST /apps` → `Depends(require_admin)` (JWT admin obligatoire) → validation Pydantic (`Mode` Literal) → check `category_id` → `AppRepository.create`.
