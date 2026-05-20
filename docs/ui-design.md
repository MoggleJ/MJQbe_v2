# UI Design — MJQbe v2

Référence visuelle pour les sprints frontend (web) et native (QML).
Source : screenshot de référence fourni par l'utilisateur.

---

## Palette de couleurs (thème `amoled` par défaut)

| Variable | Valeur | Usage |
|---|---|---|
| `--bg-primary` | `#0f0f0f` | Fond principal (zone contenu) |
| `--bg-sidebar` | `#141414` | Fond sidebar |
| `--bg-hover` | `#1e1e1e` | Item sidebar au survol / sélectionné |
| `--accent` | `#00bcd4` | Cyan — titres mode, item actif, chips sélectionnés |
| `--text-primary` | `#ffffff` | Texte principal |
| `--text-muted` | `#888888` | Texte secondaire, sous-titres |
| `--border` | `#2a2a2a` | Séparateurs |

---

## Sidebar

### Dimensions
- Largeur : `250px` fixe
- Hauteur : 100vh
- Fond : `--bg-sidebar`

### Structure (de haut en bas)

```
[icône moniteur]        ← petit, centré à gauche, ~16px, couleur --text-muted
MJ Desktop              ← "MJ" en --text-primary bold, "Desktop" en --accent, font ~28px
─────────────────────
Home                    ← icône maison + label
All Apps                ← icône grille + label  [ITEM ACTIF]
Search                  ← icône loupe + label
Admin                   ← icône bouclier + label
─────────────────────   ← séparateur horizontal --border
MJ TV                   ← icône TV + label, tout en --accent (switch de mode)
─────────────────────   ← push vers le bas (flex-grow)
Settings                ← icône engrenage + label
20:11:42                ← icône horloge + heure temps réel
```

### Item actif (ex: All Apps)
- Fond : `--bg-hover` (`#1e1e1e`)
- Bordure gauche : `3px solid --accent`
- Pas de fond coloré sur toute la largeur — juste la bordure

### Switch de mode (MJ TV)
- Texte et icône en `--accent`
- Même taille que les autres items
- Clic → bascule vers le mode TV (et le titre passe à "MJ TV")

---

## Zone contenu

### En-tête de page
```
All Apps                ← titre h1, blanc, ~32px, bold
Mode: MJ Desktop        ← sous-titre, "Mode: " en --text-muted, "MJ Desktop" en --accent
```

### Filtres catégories (chips)
- Pills arrondies (`border-radius: 999px`)
- Non sélectionné : fond `#1e1e1e`, texte blanc, pas de bordure visible
- Sélectionné (`All`) : fond transparent, bordure `1px solid --accent`, texte --accent
- Gap entre chips : `8px`
- Height : ~36px, padding : `0 16px`

### Grille d'AppCards
- Colonnes : 8 (Desktop), adaptatif selon résolution
- Gap : `16px`
- Pas de fond de carte visible — juste l'icône + le nom

### AppCard
```
┌─────────────┐
│             │
│    [ICON]   │  ← icône 80×80px, border-radius: 16px
│             │
└─────────────┘
   Nom app     ← texte centré, blanc, ~13px, tronqué après ~8 chars avec "..."
```
- Hover : légère mise en avant (scale 1.05 ou fond derrière)
- Largeur card : ~90px

---

## Titre dynamique sidebar

| Mode | Affichage |
|---|---|
| Desktop | `MJ` blanc + `Desktop` cyan |
| TV | `MJ` blanc + `TV` cyan |
| Dev | `MJ` blanc + `Dev` cyan |

---

## Icônes sidebar

Utiliser une librairie d'icônes cohérente (ex: Lucide, Phosphor, ou Material Symbols) :

| Item | Icône suggérée |
|---|---|
| Titre | `monitor` / `tv` |
| Home | `home` |
| All Apps | `grid-2x2` / `apps` |
| Search | `search` |
| Admin | `shield` |
| MJ TV | `tv` |
| Settings | `settings` / `gear` |
| Heure | `clock` |

---

## Responsive (web uniquement)

| Breakpoint | Comportement |
|---|---|
| ≥ 1920px (TV) | 8 colonnes apps, sidebar visible |
| ≥ 1280px (Desktop) | 6 colonnes, sidebar visible |
| ≥ 768px (Tablette) | 4 colonnes, sidebar réduite (icônes seules) |
| < 768px (Mobile) | 2 colonnes, sidebar hamburger |

---

## Thèmes (10 total)

Chaque thème redéfinit les variables CSS. Seul `amoled` est documenté ici (défaut).
Les autres thèmes suivent le même schéma de variables.

| Nom | `--bg-primary` | `--bg-sidebar` | `--accent` |
|---|---|---|---|
| amoled | `#0f0f0f` | `#141414` | `#00bcd4` |
| dark | `#1a1a2e` | `#16213e` | `#00bcd4` |
| dark-blue | `#0d1117` | `#161b22` | `#58a6ff` |
| dark-purple | `#13111c` | `#1a1625` | `#c084fc` |
| dark-green | `#0d1a0d` | `#111f11` | `#4ade80` |
| light | `#f5f5f5` | `#ffffff` | `#0284c7` |
| light-warm | `#faf7f2` | `#ffffff` | `#d97706` |
| light-blue | `#f0f7ff` | `#ffffff` | `#2563eb` |
| light-purple | `#f5f0ff` | `#ffffff` | `#7c3aed` |
| light-green | `#f0fdf4` | `#ffffff` | `#16a34a` |

---

## Notes pour l'implémentation native (C++ / Qt6 / QML)

- **Build** : CMake + Qt6, compilé en C++ Release pour le Pi
- **IPC** : C++ (Qt) ↔ Rust via socket Unix JSON — Qt envoie requêtes, Rust répond
- **Thèmes** : objet QML singleton `Theme {}` avec propriétés color bindées aux variables
- **AppCard** : `Image` + `Text` (pas de fond de card visible), `layer.enabled: true` pour GPU
- **GridView** : `cellWidth: 106`, `cellHeight: 120`, modèle fourni par Rust via IPC
- **Sidebar** : `Column` dans `Rectangle { width: 250 }`, fond `Theme.bgSidebar`
- **Heure** : `Timer { interval: 1000; onTriggered: timeLabel.text = Qt.formatTime(new Date(), "hh:mm:ss") }`
- **Animations** : `SequentialAnimation` + `RotationAnimation` pour la transition cube
- **Terminal Dev** : `QProcess` côté C++, exposé à QML via `Q_PROPERTY`
