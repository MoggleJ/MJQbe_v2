# Sprint 6 — App Native : UX & Animations

**Terminé le** 2026-08-30 (~23:53 CEST) · **Branche** `sprint-06-actions`

---

## 1. Ce qui a été fait

- **`qml/LoadingCube.qml`** — cube MJQbe : 4 faces (`Repeater`) tournant autour de l'axe Y via `transform: Rotation` + fausse perspective (`Scale` piloté par le spin). Aucun Qt3D / `ShaderEffect` → coût GPU minimal. Animation `NumberAnimation on spin` infinie, `running: visible`.
- **Overlay de chargement** (`Main.qml`) — `Rectangle` plein écran `z:100`, `opacity` = `Bridge.connected ? 0 : 1` avec `Behavior`, `LoadingCube` centré, timer de secours 4 s, `MouseArea` qui avale les clics tant qu'il est visible.
- **Transition de page « cube up »** (`Main.qml`, `StackView`) — `replaceExit` : la page sortante remonte (`y → -0.6·h`), rétrécit (`scale → 0.92`), s'efface ; `replaceEnter` : la page entrante monte du bas. `PropertyAction { layer.enabled }` autour de chaque anim (batch GPU) puis remis à `false`.
- **Optimisation rendu** — `layer.enabled` uniquement pendant les transitions ; `clip` conservé seulement sur les `GridView`/`ListView`/`ScrollView` ; bindings gardés simples.
- **Focus / télécommande** — `SidebarButton` : `activeFocusOnTab: true` + anneau de focus (bordure accent, surface au survol/focus). `AppGrid` : `keyNavigationWraps` + `Keys` (déjà en place S3–S4).
- **Release** — `cmake -DCMAKE_BUILD_TYPE=Release` → OK, binaire 700 Ko (`native/ui/build-release/`, gitignoré via `ui/build*/`).
- **`smoketest.sh`** — reporte désormais le `VmRSS` du process.

---

## 2. Vérifications

| Vérif | Résultat |
|---|---|
| `cmake --build` Debug | ✅ |
| `cmake -DCMAKE_BUILD_TYPE=Release` | ✅ (binaire 700 Ko) |
| `smoketest.sh` (Docker offscreen) | ✅ « QML tree loaded (**VmRSS 49 940 kB**, debug build) » — soit ~50 Mo, **< 150 Mo** cible |
| `cargo test` (core, inchangé) | ✅ 37/37 |

---

## 3. Reste à faire sur cet élément

### Différé faute de matériel
- **Fluidité réelle** des transitions sur écran Pi (EGLFS/Wayland) — la mesure offscreen ne dit rien du framerate — issue #137.
- **Profiling `heaptrack` / `valgrind massif`** sur le Pi (RSS offscreen x86 ≠ RSS EGLFS ARM) — issue #137 / Sprint 17.
- **Navigation télécommande complète** : chaîne de focus inter-sections (sidebar ↔ contenu) et `KeyNavigation` explicite sur toutes les pages — partiel ; à finir + tester avec une vraie télécommande.

### Choix assumés
- Le « cube » de chargement et la transition sont des **approximations 2.5D** (`transform: Rotation` + scale), pas un vrai rendu 3D `ShaderEffect`/`Qt3D`. Raison : garder la conso GPU/RAM basse sur Pi 4 et éviter la dépendance Qt3D. Un vrai cube shader reste possible plus tard — **nouvelle issue**.

---

## 4. Comment ça fonctionne

Au lancement, `loadingOverlay` couvre l'app (opacity 1) avec `LoadingCube` qui tourne ; dès `Bridge.connected` (ou après 4 s), `opacity → 0` via `Behavior` (fondu 400 ms) et l'overlay se cache (`visible: opacity > 0`).

À chaque `stack.replace(...)` (navigation sidebar), `StackView` joue `replaceExit` sur l'ancienne page et `replaceEnter` sur la nouvelle : chacune passe en `layer.enabled` (rendue dans une texture GPU unique), est animée en Y + scale + opacity (~220–260 ms), puis `layer.enabled` repasse à `false`.
