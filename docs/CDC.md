# Cahier des Charges — MJQbe v2

## 1. Vision

MJQbe est un hub applicatif embarqué sur Raspberry Pi 4. Il centralise l'accès aux applications, gère le matériel de la maison et fournit une interface adaptée à chaque contexte d'usage (TV, bureau, développement).

**Objectifs :**
- Interface unifiée pour toutes les applications et services de la maison
- Contrôle matériel (TV, PS4, GPIO, voix, télécommande)
- Deux interfaces distinctes : application native (locale) et interface web (réseau)
- Embarqué, léger, sécurisé

---

## 2. Deux interfaces distinctes

MJQbe v2 est composé de **deux interfaces indépendantes** qui partagent la même base de données PostgreSQL et le même daemon C.

### 2.1 Application native (PySide6 / QML)

- Tourne **directement sur le Pi** comme application de bureau (Wayland/X11)
- Lancée au démarrage via systemd
- Accessible **uniquement en local** (écran connecté au Pi)
- Couvre les **3 modes** : TV, Desktop, Dev
- Backend Python direct : SQLAlchemy + socket Unix vers le daemon C
- Authentification locale (pas OAuth) — mode admin par PIN ou mot de passe
- Pas de conteneur Docker — processus systemd natif

### 2.2 Interface web (React + FastAPI)

- Accessible depuis **n'importe quel navigateur** sur le réseau
- Couvre **TV et Desktop uniquement** (pas de Dev)
- Backend FastAPI (API REST, JWT, OAuth Google + GitHub)
- Déployée dans Docker sur le Pi

---

## 3. Modes de fonctionnement

### 3.1 Mode TV
Interface simplifiée orientée consommation de contenu.

- Icônes larges, catégories visibles
- Applications : Netflix, YouTube, Twitch, Crunchyroll, Disney+, Navigateur web
- Navigation optimisée pour télécommande
- Disponible : **app native + web**

### 3.2 Mode Desktop
Interface dense orientée productivité.

- Organisation avancée par catégories
- Accès aux applications + outils
- Navigation souris/clavier
- Disponible : **app native + web**

### 3.3 Mode Dev
Interface orientée administration système.

- Monitoring : CPU, RAM, disque, réseau, température
- Contrôle des processus, conteneurs Docker, serveurs
- Terminal intégré
- Gestion des serveurs hébergés (Minecraft, DHCP, etc.)
- Contrôle GPIO (LEDs, relais, prises, ampoules)
- Sortie vers interface graphique légère du Pi
- Requiert authentification admin avant affichage des éléments sensibles
- Disponible : **app native uniquement**

---

## 4. Interface utilisateur (UI/UX)

### 4.1 Structure générale
- **Sidebar fixe à gauche** dans tous les modes
- Contenu principal à droite (grille d'apps, catégories)
- Identique dans les deux interfaces (native et web)

### 4.2 Sidebar

**Titre dynamique :**
- MJ TV (mode TV)
- MJ Desktop (mode Desktop)
- MJ Dev (mode Dev)

**Menu principal :**
- Home
- All Apps
- Search
- Switch de mode (MJ Desktop ↔ MJ TV)

**Bas de sidebar :**
- Settings
- Heure (affichage temps réel)

### 4.3 Affichage du contenu
- Grilles d'applications
- Cartes : icône + nom
- Catégories visuellement séparées

### 4.4 Animations
- **Icône de chargement** : cube MJQbe animé en rotation 3D
- **Transition de page** : rotation vers le haut type cube (effet PowerPoint)
- Animations GPU-accélérées (OpenGL ES via QML pour le natif, CSS 3D pour le web)

### 4.5 Responsive
L'interface web s'adapte à : TV (1920px), desktop (1280px), tablette (768px), mobile (375px).
L'interface native cible : TV (1920×1080) et desktop (1280×800).

### 4.6 Ouverture des applications
- Intégration directe dans l'interface (iframe / QML WebView)
- Ou ouverture dans un nouvel onglet / fenêtre navigateur

---

## 5. Partie web

### 5.1 Périmètre
- Modes disponibles : **TV** et **Desktop** uniquement
- Mode Dev : app native uniquement

### 5.2 Utilisateurs
- Authentification OAuth 2.0 (Google, GitHub)
- Authentification locale (username + mot de passe hashé)
- Chaque utilisateur possède ses propres : catégories, apps favorites, apps personnelles, paramètres (thèmes, etc.)

### 5.3 Administration
- Panel admin séparé
- Gestion des utilisateurs, des apps, des catégories

### 5.4 Logs
- Séparés du reste de l'application
- Enregistrent : actions utilisateurs, lancements d'apps
- Consultables dans le panel admin

---

## 6. Application native

### 6.1 Stack
| Couche | Tech |
|---|---|
| Interface | PySide6 + QML (Qt 6.x) |
| Logique métier | Python 3.11 |
| Accès données | SQLAlchemy (direct PostgreSQL) |
| Accès daemon | Socket Unix (même daemon C que le web) |
| Démarrage | systemd service |

### 6.2 Authentification
- Pas d'OAuth
- Compte admin local (PIN ou mot de passe)
- Re-authentification obligatoire avant toute action sensible (mode Dev)

### 6.3 Structure des dossiers
```
native/
  app/
    domain/       # Entités, interfaces
    application/  # Use cases
    infrastructure/  # DB, socket daemon
    ui/           # Fichiers QML
      components/
      modes/
        tv/
        desktop/
        dev/
  main.py
  requirements.txt
```

---

## 7. Thèmes

10 thèmes disponibles (partagés entre native et web) :

| Nom | Famille |
|---|---|
| dark | Sombre |
| dark-blue | Sombre |
| dark-purple | Sombre |
| amoled | Sombre |
| dark-green | Sombre |
| light | Clair |
| light-warm | Clair |
| light-blue | Clair |
| light-purple | Clair |
| light-green | Clair |

---

## 8. Sécurité

- Authentification obligatoire pour la partie web
- Mots de passe hashés (bcrypt)
- JWT pour la gestion des sessions web
- Protection de toutes les routes sensibles
- Gestion des rôles : `user` / `admin`
- Vérification d'identité obligatoire avant toute action sensible (mode Dev, panel admin)
- Validation des entrées utilisateur côté serveur
- CORS configuré strictement
- Serveurs non accessibles sans connexion

---

## 9. Fonctionnalités matérielles

- Gestion Wi-Fi / Bluetooth
- Allumage par télécommande IR
- Contrôle TV via HDMI CEC (allumage/extinction)
- Contrôle PS4 via HDMI CEC ou Bluetooth
- Reconnaissance vocale : wake word "OK..." + commandes (ex : "Allume la TV")
- Contrôle GPIO : LEDs, relais, prises connectées, ampoules
- Screen sharing
- Hébergement de serveurs (Minecraft, DHCP, etc.)

---

## 10. Contraintes techniques

### 10.1 Performance
- Embarqué sur Raspberry Pi 4 (4 Go RAM)
- App native : cible < 150 Mo RAM
- Mémoire utilisée minimisée
- Animations GPU (QML OpenGL ES / CSS 3D)

### 10.2 Architecture
- **Clean Architecture** obligatoire pour les deux interfaces (Domain → Application → Infrastructure → Interface)
- **Dockerisation** pour la partie web uniquement
- App native comme processus systemd (accès direct hardware + display)
- IHM totalement séparée de la gestion système
- Point d'entrée unique par couche (API REST pour le web, Python direct pour le natif)
- Dossier `agents/` : fichiers de spécifications pour les agents IA

### 10.3 CLI
- Une CLI nommée `dev` écrite en Bash
- Couvre : démarrage/arrêt des services Docker, commandes système, opérations agents

---

## 11. Modèle de données (résumé)

Voir `docs/data-model.md` pour le schéma SQL complet.

| Table | Rôle |
|---|---|
| users | Comptes utilisateurs web |
| apps | Applications disponibles |
| categories | Catégories d'apps par mode |
| settings | Préférences par utilisateur |
| favorites | Apps favorites par utilisateur |
| logs | Journal des actions web |
