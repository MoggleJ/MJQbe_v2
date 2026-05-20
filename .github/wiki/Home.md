# MJQbe v2 — Wiki

Hub applicatif embarqué sur Raspberry Pi 4. Interface unifiée pour applications, contrôle matériel et monitoring.

## Navigation

| Page | Contenu |
|---|---|
| [Architecture](Architecture) | Stack technique, services, flux de données |
| [Installation](Installation) | Prérequis, setup, premier lancement |
| [App Native](App-Native) | C++/Qt6/QML + Rust — build, IPC, modes |
| [Interface Web](Interface-Web) | React + FastAPI — Docker, OAuth, API |
| [Daemon C](Daemon-C) | GPIO, IR, CEC, Bluetooth |
| [Sprints](Sprints) | Avancement par sprint, issues associées |
| [Problèmes connus](Problemes) | Bugs résolus et workarounds |
| [Déploiement Pi](Deploiement-Pi) | Mise en production sur Raspberry Pi 4 |

---

## Résumé du projet

```
Raspberry Pi 4
├── App native (C++/Qt6 + Rust)     ← processus systemd, local uniquement
│   ├── Mode TV                      ← grille apps, navigation télécommande
│   ├── Mode Desktop                 ← layout dense, catégories
│   └── Mode Dev                     ← monitoring, terminal, GPIO
├── Interface web (React + FastAPI)  ← Docker, accès réseau
│   ├── Mode TV
│   └── Mode Desktop
└── Daemon C                         ← GPIO, IR, CEC, Bluetooth
```

## Ports

| Service | Port |
|---|---|
| Interface web | 4444 |
| API REST | 4848 |
| PostgreSQL | 5432 (interne Docker) |

## Commandes rapides

```bash
dev up          # Démarre la stack web (prod)
dev watch       # Hot-reload (dev)
dev down        # Arrête tout
dev health      # Vérifie l'état des services
dev db          # Shell psql
dev logs api    # Logs de l'API
```
