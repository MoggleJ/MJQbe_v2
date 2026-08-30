# Sprint 15 — Frontend Web : UX & Animations

**Terminé le** 2026-08-31 (~03:20 CEST) · **Branche** `sprint-15-actions`
**Issues fermées** : #114–#118

## Fait
- **`components/LoadingCube.tsx`** : cube CSS 3D — 6 `.cube-face` positionnées via `rotate*/translateZ(42px)`, parent `transform-style: preserve-3d` + `animation: cubeSpin 2.6s linear infinite`. Overlay `.cube-overlay` plein écran + label « MJQbe ». Affiché par `App.tsx` tant que `useAuth().loading`.
- **Transition de page** : `.content { perspective: 1400px }` ; `.page-enter` → `@keyframes cubeUp` (`rotateX(-32deg) translateY(28px) scale(0.97)` → identité, 320 ms). Chaque page a `className="page-enter"` sur son div racine → l'anim se rejoue à chaque navigation (le composant de page est remonté).
- **`pages/Settings.tsx`** : les chips de thème deviennent des `.theme-swatch` — fond = `theme.bg`, 3 barres = `surface` / `accent` / `text`, bordure accent si actif. Clic → `setTheme` (temps réel + `PUT /settings`).
- **`styles.css`** : `.cube-*`, `.theme-swatch`, `@media (prefers-reduced-motion: reduce)` (cube ralenti à 8 s, `.page-enter` sans anim).

## Déjà fait (S13, coché)
Application du thème en temps réel (variables CSS sur `:root`), settings ↔ API (`GET`/`PUT /settings`).

## Vérif
`docker build ./frontend` → build 54 modules ; `tsc --noEmit` clean ; `vitest` 2/2 ; SPA re-déployée et servie sur `:4444` (assets rebuildés), `/api/health` OK.

## Reste
- L'anim de transition est un « flip » d'entrée (pas de rotation sortante synchronisée façon PowerPoint plein cube) — suffisant et léger ; un vrai cube à 2 faces demanderait de garder l'ancienne page montée (peut faire l'objet d'une issue si souhaité).
