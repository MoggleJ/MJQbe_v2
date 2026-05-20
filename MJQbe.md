# MJQbe

* Un hub d'applications.
3 catégories : 
  * Desktop
  * TV
  * Dev
  
Il s'agit d'une application avec pour langages majoritaire C / Rust / C++ / Python / bash (au choix, privilégier le proche du matériel).
Le choix de bdd est libre, pouquoi pas postgresql.
Chaque level de l'application doit être proprement séparé des autres, avec un unique point d'entrée à chaque fois (type mapping, api ou un truc du style)
La gestion de la mémoire et des accès est tres importante. L'app est embarquée, donc la mémoire utilisée doit être la plus efficace, adaptée possible.

L'icone de chargement doit être un cube MJQbe qui tourne. 
Il doit y avoir un effet de transition lors de chargement de page via une rotation vers le haut comme un cube (comme transition powerpoint).

Plusieurs agents vont travailler (claude code et github agents)

L'application possède une partie web, dans laquelle seuls Desktop et TV sont dispos. La versions WEB utilise des users et utilise OAuth. Il y a une partie admin aussi. Les logs ne concernent que la partie web, cest un truc separées. Les categories, les apps favorites, les apps personnelles, les paramètres themes etc sont propres a chaque user.



## Interface utilisateur (UI/UX)

### Structure générale
L'interface est organisée autour d'une **sidebar fixe située à gauche**.

#### Sidebar
* Titre dynamique :
  * **MJ TV** (mode TV)
  * **MJ Desktop** (mode Desktop)
  * **MJ Dev** (mode Dev)

* Menu principal :
  * Home
  * All Apps
  * Search
  * MJ Desktop / MJ TV (switch de mode)

* Bas de la sidebar :
  * Settings
  * Heure

### Organisation du contenu
Le contenu principal est affiché sous forme de :
* Grilles d'applications
* Cartes avec icônes + noms
* Catégories visuellement séparées

L'interface doit être :
* Responsive (TV, desktop, tablette, mobile)
* Fluide
* Minimaliste
* Optimisée pour souris/clavier/telecommande
* **Note technique** : Les applications s'ouvrent soit en intégration directe, soit en **onglet google**.

---

## Fonctionnalités

### Fonctionnalités communes 
* Affichage des applications disponibles
* Organisation par catégories
* Recherche d'applications
* Navigation entre modes (TV / Desktop)
* Interface personnalisable (thème)

### Mode TV
Le mode TV propose une interface simplifiée orientée consommation de contenu.
Fonctionnalités :
* Accès à des applications: Netflix, YouTube, Twitch, Crunchyroll, Disney+, Navigateur web.
* Affichage simplifié : icônes larges, catégories visibles.

### Mode Desktop
Le mode Desktop offre une organisation plus dense et orientée productivité.
Fonctionnalités :
* Accès à des applications (avec en plus navigateur web) ou outils.
* Organisation avancée par catégories.

### Mode Dev
Le mode Dev offre un organisation plutot orientée logiciel :
C'est ici que sont definis tous les paramètres logiciels et de la raspberry portant l'app.
Fonctionnalités : 
* Monitoring paramétrage.
* Demande un login admin avant d'afficher les trucs importants / dangereux
* contrôle matériel, gestion système.
* mémoire, gpus, ...
* terminal, gestion des processus, des serveurs, des conteneurs
* Une sortie vers l'interface graphique (light) de la raspberry pour une programmation basique


---


## Sécurité
* Authentification utilisateur
* Hash des mots de passe
* Protection des routes
* Validation des entrées utilisateur
* Gestion des rôles (admin / utilisateur)
* Vérification d'identité obligatoire avant tout changements sensibles (comme root finalement).



## Modèle de données

### Table Apps
* id
* name
* icon
* url/app
* category_id
* mode (TV / Desktop)
* **is_web** (Boolean pour gérer l'ouverture sur internet)

### Table Categories
* id
* name
* mode

### Table Settings
* id
* theme (10 valeurs : `dark`, `dark-blue`, `dark-purple`, `amoled`, `dark-green`, `light`, `light-warm`, `light-blue`, `light-purple`, `light-green`)
* layout
* icon_size
* selected_apps

### Table Favorites
* id
* app_id

### Table Logs
* id
* user_id (nullable)
* action (`command`, `app_launch`)
* metadata
* created_at

---

Cette version prend en charge :
* Gestion du Wi-Fi / Bluetooth / Installation locale.
* Screen sharing / Terminal / Processus système.
* Hébergement de serveurs (Minecraft, DHCP, etc.) / Interaction matérielle.
* Allumage grace à une télécommande.
* Reconnaissance vocale (OK ... Allume la TV)
* capable d'allumer la TV ou la PS4 via reconnaissance vocale ou telecommande seulement
* futur : pilotage led , prises , ampoules, autres systemes

## Architecture matérielle
J'ai besoin que tu me fasse une architecture matérielle aussi. Pour cela, il faut que tu m'indique quel matos acheter. 
Je dispose maintenant: 
* Raspberry pi 4 model b 
* HC-05 bluetooth module
* isd1820 recording module voice module
* arduino unos et nano
* resistances, condensateurs, leds,...
* breadboards

dis moi tous les composants necessaires dans les docs, et les pins associées


## Architecture logicielle
Le système est composé de plusieurs modules indépendants :
* **Dockerisation** : Chaque module tourne dans son propre container pour faciliter la maintenance.
* **Clean Architecture** obligatoire
* **Dossier `agents/`** : Fichiers de spécifications techniques pour l'agent IA
* l'IHM doit etre completement séparée de la gestion système etc.
* L'architecture doit être sécurisée
* les serveurs ne doivent pas etre accédé sans connexion ou droits particuliers. Chacune demandera un login.
* je veux que tu me cree une cli avec comme mot appelant dev. on en a a deja fait une avec beaucoup de docker. Reprend la et ajoute des possibilités
* 

Lien du repository : https://github.com/MoggleJ/MJQbe_v2.git

* je veux que tu t'organise avec les agents. vous devez tourner en boucle. A la fin de chaque sprints, je veux que vous pushiez sur mon git et que vous metiez toutes les issues etc. Je veux ensuite que tu compare avec les specications et que tu vérifie que tout marche bien. Ainsi, chaque sprint comporte push sur git dev, lecture spec, dev, comparaison spec, correction, tests, correction, push sur git sprint-XX-Actions.
* Des qu'un agent rencontre un problème, il l'ajoute au fichier problemes.md avec sa solution et le prend en contexte

## Prompt Claude 
Hey Claude Plan, j'ai besoin de toi. Voici ci-joint un résumé de mon prochain projet, que nous ferons ensemble. Je veux que tu me redige toute les documentation nécéssaire et optimisées pour toi et autres agents (github), et pour moi aussi, incluant les specifications (cdc et tout), le plan d'implémentation qui doit etre decoupé assez finement, mais pas trop, pour obtimiser la quantité de dev-tests-correction.
Des que tu as une question, pose la moi. 
Je ne suis pas tres douée en technologies, donc si tu as des questions liées, explique moi ce que tu propose, pourquoi et ou se renseigner. 
