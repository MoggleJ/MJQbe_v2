# Sprint 4 — App Native : Mode TV + Desktop

**Terminé le** 2026-08-30 (~23:36 CEST) · **Branche** `sprint-04-actions`

---

## 1. Ce qui a été fait

### 1.1 `native/core` — nouveaux use cases

| Couche | Ajouts |
|---|---|
| domain | `Settings`, `SettingsPatch` ; constantes `THEMES` (10), `LAYOUTS`, `ICON_SIZES`, `USER_MODES` ; traits `FavoritesRepository`, `SettingsRepository` ; `CatalogRepository::recent_apps` ; `AuthRepository::default_admin` |
| application | `FavoritesService` (list / toggle), `SettingsService` (get-or-create + **validation enum** avant écriture), `CatalogService::recent_apps` (clamp 1..50), `AuthService::current_user` |
| infrastructure/db | `PgFavoritesRepository` (toggle = DELETE…RETURNING sinon INSERT), `PgSettingsRepository` (UPSERT + COALESCE), `recent_apps` (JOIN LATERAL sur `logs` action `app_launch`), `default_admin` |
| interface/ipc | `Handler` prend un `Services` ; nouvelles méthodes : `session.current`, `apps.recent`, `favorites.list`, `favorites.toggle`, `settings.get`, `settings.update` |

**Tests** : 27 (26 unit + 1 intégration socket). `cargo clippy -D warnings` clean, `cargo fmt` clean.

### 1.2 `native/ui` — QML

- **`NativeBridge`** : propriétés `userId` / `userName`, `fetchSession()` (auto après connexion), méthodes `listRecent`, `listFavorites`, `toggleFavorite`, `getSettings`, `updateSettings`, `listApps(mode, categoryId)` ; signaux `recentReceived`, `favoritesReceived`, `favoriteToggled`, `settingsReceived`, `sessionChanged`.
- **`AppCard.qml`** : taille d'icône paramétrable (64/80/96), étoile favori (toggle), focus clavier (`Keys` Return/Enter/Space).
- **`AppGrid.qml`** (nouveau) : GridView réutilisable ; `mode` "tv" = cellules larges + `keyNavigationWraps`, "desktop" = denses ; `iconSize` depuis settings ; `favoriteIds`.
- **`CategoryChips.qml`** (nouveau) : chips « Tout » + catégories, filtre.
- **`Main.qml`** : état transverse (session → settings → thème/layout/iconSize appliqués ; `favoriteIds` maintenu sur `favoriteToggled`). Helpers `openApp`, `toggleFav`, `isFav`.
- **`Home.qml`** : sections Favoris + Récents.
- **`AllApps.qml`** : chips catégories + `TextField` filtre + `AppGrid`.
- **`Search.qml`** : filtre live (vide → invite).
- **`Settings.qml`** : composant interne `OptionRow` × 3 (thème / disposition / taille icônes), chaque choix → `Bridge.updateSettings`.

### 1.3 Outillage de test

- `native/ui/Dockerfile.smoketest` — image Qt6 (debian:bookworm) pour valider le chargement QML offscreen (≠ artefact de déploiement).
- `native/ui/smoketest.sh` — build image + run offscreen + verdict, se connecte au socket core si fourni.

---

## 2. Vérifications

| Vérif | Résultat |
|---|---|
| `cargo test` (core) | ✅ 27/27, clippy `-D warnings` clean |
| `cmake --build native/ui` | ✅ qmlcachegen OK sur tous les .qml (dont AppGrid, CategoryChips) |
| E2E core ↔ PostgreSQL (seed) | ✅ `session.current`→admin ; `settings.get` crée la ligne ; `settings.update` persiste + relecture OK ; `theme:"neon"` rejeté ; `favorites.toggle 1` on→`[1]`→off→`[]` ; `apps.recent`→`[]` (pas de logs, attendu) |
| `native/ui/smoketest.sh /tmp/mjqbe-dev.sock` (Docker offscreen) | ✅ « VERDICT: OK — QML tree loaded » |

---

## 3. Reste à faire sur cet élément

### Différé faute de matériel / de sprint
- Navigation **télécommande** réelle (KeyNavigation posé, à éprouver sur écran + télécommande — issue #137).
- **Mode Desktop « catégories groupées »** : actuellement chips + filtre (pas de sections empilées par catégorie). À enrichir si besoin visuel.
- Ouverture **embarquée** des apps (`is_web === false`) via `QWebEngineView` — nécessite `qt6-webengine`, lourd sur Pi. Repoussé (Sprint 6 UX ou dédié).
- `apps.recent` ne renverra des données qu'une fois que des `logs` `app_launch` existent (peuplés par le middleware web, Sprint 12) — ou ajouter l'écriture de logs côté natif (non prévu par le CDC pour le natif).
- Réorg QML `components/` + `modes/` — issue #138 (toujours ouverte).

### Dette
- `Search`/`AllApps` filtrent en JS côté UI sur la liste complète du mode (OK pour ~20 apps ; si le catalogue grossit, pousser le filtre côté core).

---

## 4. Comment ça fonctionne

Au démarrage : `Bridge` se connecte → `session.current` → `sessionChanged` → `Main` appelle `getSettings()` + `listFavorites()` → `settingsReceived` applique thème/layout/iconSize, `favoritesReceived` remplit `favoriteIds`.

Une page (ex. `AllApps`) : `Component.onCompleted` → `Bridge.listApps(mode)` + `listCategories(mode)` → signaux → `page.allApps` / `page.categories` → `filtered` (catégorie + texte, calculé en QML) → `AppGrid` (model = JS array, delegates via `modelData`).

Toggle favori : `AppCard` étoile → `AppGrid.favoriteToggled(id)` → `window.toggleFav` → `Bridge.toggleFavorite` → core `favorites.toggle` (DELETE/INSERT) → `favoriteToggled(id, bool)` → `Main` met à jour `favoriteIds` → toutes les grilles se rafraîchissent (binding).

Changer un réglage : `Settings` chip → `Bridge.updateSettings({theme:"..."})` → core valide l'enum puis `UPDATE … COALESCE` → `settingsReceived` → `ThemeManager.setTheme` / `window.layout` / `window.iconSize`.
