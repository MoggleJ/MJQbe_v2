# Revue finale — MJQbe v2 vs `docs/CDC.md`

État au terme du Sprint 17. ✅ = fait & vérifié · 🟡 = fait, vérif matériel Pi en attente · ⏳ = suivi en issue.

## 2. Deux interfaces
| CDC | État |
|---|---|
| App native C++/Qt6/QML + Rust, systemd, local, 3 modes | 🟡 build + tests + smoke Docker ; écran/Pi → #137 |
| Interface web React + FastAPI, réseau, TV+Desktop, OAuth | ✅ |
| Base PostgreSQL partagée + daemon C partagé | ✅ (schéma commun ; core Rust et API ont chacun leur couche d'accès) |

## 3. Modes
| Mode | Natif | Web |
|---|---|---|
| TV | ✅ (grille large, KeyNavigation) | ✅ |
| Desktop | ✅ (dense) | ✅ (sections par catégorie) |
| Dev | ✅ (monitoring, process/Docker, terminal, re-auth) | — (natif only, conforme) |

## 4. UI/UX
- Sidebar fixe gauche, titre dynamique, menu, heure temps réel, Settings, switch de mode : ✅ (natif + web).
- Cartes icône+nom, catégories séparées : ✅.
- Animations : cube de chargement + transition « cube up » — ✅ web (CSS 3D), 🟡 natif (2.5D `transform`, vrai shader → #144).
- Responsive web 1920/1280/768/375 : ✅ (media queries).
- Ouverture apps : `Qt.openUrlExternally` / `window.open` ✅ ; iframe/WebView intégrée ⏳ #140.

## 5. Web
- OAuth Google + GitHub : ✅ (routes + échange + upsert) ; aller-retour réseau réel non testé (pas d'identifiants).
- Auth locale bcrypt + JWT : ✅.
- Données par utilisateur (catégories/apps/favoris/settings) : ✅ favoris + settings ; « apps personnelles » par user : non demandé explicitement au plan, catalogue global admin.
- Panel admin séparé (users, apps, catégories) : ✅.
- Logs séparés (actions, `app_launch`) consultables admin : ✅.

## 6. App native
- Stack C++/Qt6 + Rust `sqlx`/`tokio` + socket Unix vers daemon C + systemd : ✅.
- Auth admin locale (pas OAuth), re-auth avant action sensible : ✅ (token usage unique, TTL 120 s).

## 7. Thèmes
10 thèmes partagés (5 sombres + 5 clairs), amoled par défaut : ✅ (natif `ThemeManager`, web variables CSS).

## 8. Sécurité
| CDC | État |
|---|---|
| Auth obligatoire web | ✅ (routes protégées ; lectures catalogue publiques par choix) |
| Mots de passe bcrypt | ✅ |
| JWT sessions web | ✅ (access + refresh) |
| Protection routes sensibles | ✅ `/admin/*` + `/dev/*` admin-only |
| Rôles user/admin | ✅ |
| Vérif identité avant action sensible | ✅ (re-auth config/reboot ; token natif Dev) |
| Validation entrées serveur | ✅ Pydantic + enums |
| CORS strict | ✅ (méthodes + headers restreints, origines depuis config) |
| Headers HTTP (HSTS/CSP…) | ✅ `SecurityHeadersMiddleware` + nginx |
| Rate limiting auth | ✅ 20/min/IP (config) |

## 9. Matériel — 🟡 (code + stub, vérif Pi #137)
Wi-Fi/BT : gestion BT via HC-05 (daemon) ; IR allumage : ✅ mapping `ir-map.json` ; HDMI-CEC TV/PS4 : ✅ via `cec-client` ; voix wake word + commandes : ✅ grammaire + dispatch (capture audio réelle → #145) ; GPIO LED/relais : ✅ ; screen sharing / hébergement serveurs : partiel (bouton VNC #143 ; gestion conteneurs Docker en Dev).

## 10. Contraintes techniques
- Clean Architecture (Domain→Application→Infrastructure→Interface) : ✅ natif (Rust) **et** web (FastAPI).
- Dockerisation web only, natif = systemd : ✅.
- IHM séparée de la gestion système : ✅ (IPC / API).
- Point d'entrée unique par couche : ✅ (API REST ; IPC socket).
- App native < 150 Mo RAM : 🟡 ~50 Mo mesuré (debug, offscreen, x86) — mesure EGLFS/Pi #137.
- Images Docker : frontend & daemon multi-stage + alpine/slim ✅ ; api `python:3.11-slim` mono-stage (acceptable).
- CI GitHub Actions (api / docker / native) sur push : ✅.

## Issues de suivi ouvertes
#2 (migrations/seed sur Pi ARM), #137 (vérif native sur Pi), #138 (réorg QML), #140 (WebView), #141 (Desktop groupé natif), #142 (terminal PTY), #143 (URL GUI Pi), #144 (cube shader), #145 (capture audio Vosk), #128–136 (tâches Sprint 17 : profiling Pi, HTTPS réel, charge, etc.).
