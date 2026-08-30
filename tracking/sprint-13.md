# Sprint 13 — Frontend Web : Layout, Mode TV & Admin Panel

**Terminé le** 2026-08-31 (~03:05 CEST) · **Branche** `sprint-13-actions`
**Issues fermées** : #93–#107 · **également fermées ce tour** : #1, #4–#41 (tâches détaillées des Sprints 3–7, oubliées jusqu'ici)

---

## 1. Ce qui a été fait — `frontend/` (React 18 + Vite + TypeScript)

| Zone | Fichiers |
|---|---|
| Config | `package.json` (react-router-dom, typescript, vitest, testing-library), `tsconfig.json` (strict), `vite.config.ts` (proxy `/api` + config vitest jsdom), `index.html` → `main.tsx`, `.dockerignore` |
| API | `src/api/client.ts` (fetch, `localStorage` tokens, **refresh auto sur 401** avec dédup), `src/api/endpoints.ts` (auth / catalog / user / admin), `src/api/types.ts` |
| État | `src/auth/AuthContext.tsx` (`me`, `login`, `logout`, `isAdmin`), `src/theme/UiContext.tsx` (thème/mode/layout/iconSize, persistés `localStorage` + `PUT /settings` si connecté), `src/theme/themes.ts` (10 thèmes → variables CSS) |
| Composants | `Sidebar` (titre dynamique, menu, switch mode, `Clock`, Paramètres, login/logout), `AppCard`, `AppGrid` (mode/layout/iconSize + favoris), `Clock`, `Modal`, `CategoryChips`(inline dans AllApps) |
| Pages | `Home` (favoris + toutes apps), `AllApps` (chips catégories + filtre texte), `Search` (filtre live), `Settings` (thème/layout/taille), `Login` (login+register+OAuth), `Admin` (onglets users / catalogue CRUD / logs / système) |
| Routing | `App.tsx` (`BrowserRouter`, `RequireAdmin` garde `/admin`), `main.tsx` (providers) |
| Style | `src/styles.css` — variables CSS thèmes, layout, grid (`.grid.tv` large), responsive @900/@640, `.page-enter` (transition cube-up CSS, affinée au Sprint 15) |
| Tests | `src/test/{setup.ts,basic.test.tsx}` — thèmes (10) + `AppCard` (rendu + onOpen) |
| Nginx / Docker | `nginx.conf.template` (déjà OK : `/api/` → `api:4848/`, `try_files … /index.html`) ; `frontend/.dockerignore` (le build `docker build ./frontend` recopiait un `node_modules` vide root de l'hôte) |

---

## 2. Vérifications

| Vérif | Résultat |
|---|---|
| `docker build ./frontend` (`tsc && vite build`) | ✅ 51 modules, build 1.4 s, image nginx |
| `npm run test` (vitest, dans l'image builder) | ✅ 2/2 |
| Live — `docker compose up -d --build frontend` | ✅ conteneur `healthy` |
| `GET http://localhost:4444/` | ✅ SPA + `/assets/index-*.js` + `.css` |
| `GET /api/health` (proxy nginx) | ✅ `{"status":"ok"}` |
| `GET /api/apps?mode=tv` (proxy, public) | ✅ 7 apps |
| `GET /admin` (deep link) | ✅ 200 (fallback `index.html`) |

---

## 3. Reste à faire

- **Parcours navigateur cliqué** (login → grille TV → admin → édition config) : non exécuté (pas de navigateur piloté ici). Toutes les API appelées par le front sont vérifiées en live aux Sprints 10-12 ; le proxy `/api` est vérifié ci-dessus.
- Sprint 14 : mode Desktop dense + sections par catégorie, section Favoris dédiée, switch déjà en place.
- Sprint 15 : animation cube 3D CSS + page Settings avec **prévisualisation** des thèmes + application temps réel (déjà temps réel via `UiContext`, la prévisu reste à faire).
- **OAuth front** : les boutons pointent vers `/api/auth/oauth/{provider}` ; le callback renvoie actuellement un JSON `{access_token,…}` — il faudra une page `/oauth/callback` côté front qui stocke les tokens (ou faire renvoyer une redirection avec fragment par l'API). À traiter avec les vrais identifiants OAuth.
- `noUnusedParameters`/`noUnusedLocals` actifs — code nettoyé en conséquence.

## 4. Comment ça fonctionne

`main.tsx` monte `BrowserRouter > AuthProvider > UiProvider > App`. `AuthProvider` appelle `GET /auth/me` si un token est présent. `UiProvider` applique le thème (variables CSS sur `:root`) et, si connecté, charge `GET /settings` puis pousse chaque changement via `PUT /settings`.

`client.ts` : tout appel `api()` ajoute `Authorization: Bearer` ; sur `401`, un `POST /auth/refresh` (dédupliqué) est tenté puis la requête est rejouée ; échec → tokens effacés.

Nginx sert `dist/` et proxifie `location /api/ → http://api:4848/` (le préfixe `/api` est retiré). En dev, `vite.config.ts` fait le même proxy vers `http://api:4848`.
