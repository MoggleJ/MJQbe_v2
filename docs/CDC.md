# Cahier des Charges — MJQbe v2

## 1. Vision

MJQbe est un hub applicatif embarqué sur Raspberry Pi 4. Il centralise l'accès aux applications, gère le matériel de la maison et fournit une interface adaptée à chaque contexte d'usage (TV, bureau, développement).

**Objectifs :**
- Interface unifiée pour toutes les applications et services de la maison
- Contrôle matériel (TV, PS4, GPIO, voix, télécommande)
- Accessible depuis le navigateur web (TV + Desktop uniquement) avec comptes utilisateurs
- Embarqué, léger, sécurisé

---

## 2. Modes de fonctionnement

### 2.1 Mode TV
Interface simplifiée orientée consommation de contenu.

- Icônes larges, catégories visibles
- Applications disponibles : Netflix, YouTube, Twitch, Crunchyroll, Disney+, Navigateur web
- Navigation optimisée pour télécommande
- Accessible en version web

### 2.2 Mode Desktop
Interface dense orientée productivité.

- Organisation avancée par catégories
- Accès aux applications + outils
- Navigation souris/clavier
- Accessible en version web

### 2.3 Mode Dev
Interface orientée administration système. Disponible uniquement en local sur le Raspberry Pi.

- Monitoring : CPU, RAM, disque, réseau, température
- Contrôle des processus, conteneurs Docker, serveurs
- Terminal intégré
- Gestion des serveurs hébergés (Minecraft, DHCP, etc.)
- Contrôle GPIO (LEDs, relais, prises, ampoules)
- Sortie vers l'interface graphique légère de la Raspberry (programmation basique)
- Requiert authentification admin avant affichage des éléments sensibles

---

## 3. Interface utilisateur (UI/UX)

### 3.1 Structure générale
- **Sidebar fixe à gauche** dans tous les modes
- Contenu principal à droite (grille d'apps, catégories)

### 3.2 Sidebar

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

### 3.3 Affichage du contenu
- Grilles d'applications
- Cartes : icône + nom
- Catégories visuellement séparées

### 3.4 Animations
- **Icône de chargement** : cube MJQbe animé en rotation 3D
- **Transition de page** : rotation vers le haut type cube (effet PowerPoint)

### 3.5 Responsive
L'interface s'adapte à : TV, desktop, tablette, mobile.

### 3.6 Ouverture des applications
- Intégration directe dans l'interface (iframe)
- Ou ouverture dans un nouvel onglet navigateur

---

## 4. Partie web

### 4.1 Périmètre
- Modes disponibles en web : **TV** et **Desktop** uniquement
- Mode Dev : local uniquement

### 4.2 Utilisateurs
- Authentification OAuth 2.0 (Google, GitHub)
- Authentification locale (username + mot de passe hashé)
- Chaque utilisateur possède ses propres : catégories, apps favorites, apps personnelles, paramètres (thèmes, etc.)

### 4.3 Administration
- Panel admin séparé
- Gestion des utilisateurs, des apps, des catégories

### 4.4 Logs
- Séparés du reste de l'application
- Enregistrent : actions utilisateurs, lancements d'apps
- Consultables dans le panel admin

---

## 5. Thèmes

10 thèmes disponibles :

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

## 6. Sécurité

- Authentification obligatoire pour la partie web
- Mots de passe hashés (bcrypt)
- JWT pour la gestion des sessions
- Protection de toutes les routes sensibles
- Gestion des rôles : `user` / `admin`
- Vérification d'identité obligatoire avant toute action sensible (mode Dev, panel admin)
- Validation des entrées utilisateur côté serveur
- CORS configuré strictement
- Serveurs non accessibles sans connexion

---

## 7. Fonctionnalités matérielles

- Gestion Wi-Fi / Bluetooth
- Allumage par télécommande IR
- Contrôle TV via HDMI CEC (allumage/extinction)
- Contrôle PS4 via HDMI CEC ou Bluetooth
- Reconnaissance vocale : wake word "OK..." + commandes (ex : "Allume la TV")
- Contrôle GPIO : LEDs, relais, prises connectées, ampoules
- Screen sharing
- Hébergement de serveurs (Minecraft, DHCP, etc.)

---

## 8. Contraintes techniques

### 8.1 Performance
- Embarqué sur Raspberry Pi 4 (4 Go RAM)
- Mémoire utilisée minimisée
- Pas de dépendances lourdes inutiles

### 8.2 Architecture
- **Clean Architecture** obligatoire (Domain → Application → Infrastructure → Interface)
- **Dockerisation** : chaque module dans son propre conteneur
- IHM totalement séparée de la gestion système
- Point d'entrée unique par couche (API REST)
- Dossier `agents/` : fichiers de spécifications pour les agents IA

### 8.3 CLI
- Une CLI nommée `dev` écrite en **Bash**
- Couvre : démarrage/arrêt des services Docker, commandes système, opérations agents

---

## 9. Modèle de données (résumé)

Voir `docs/data-model.md` pour le schéma SQL complet.

| Table | Rôle |
|---|---|
| users | Comptes utilisateurs web |
| apps | Applications disponibles |
| categories | Catégories d'apps par mode |
| settings | Préférences par utilisateur |
| favorites | Apps favorites par utilisateur |
| logs | Journal des actions web |
