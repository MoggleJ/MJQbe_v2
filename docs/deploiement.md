# Déploiement production — MJQbe v2

Cible : **Raspberry Pi 4 (4 Go)** sous Raspberry Pi OS / Debian Bookworm (arm64).
Deux briques indépendantes : la **stack web Docker** et l'**app native** (systemd).

---

## 1. Prérequis Pi

```bash
sudo apt update && sudo apt install -y docker.io docker-compose-plugin git
sudo usermod -aG docker "$USER" && newgrp docker

# App native
sudo apt install -y \
  qt6-base-dev qt6-declarative-dev qt6-declarative-dev-tools libgl1-mesa-dev \
  qml6-module-qtquick qml6-module-qtquick-controls qml6-module-qtquick-templates \
  qml6-module-qtquick-layouts qml6-module-qtquick-window \
  cmake build-essential libcjson-dev cec-utils
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
```

---

## 2. Stack web (Docker)

```bash
git clone https://github.com/MoggleJ/MJQbe_v2.git && cd MJQbe_v2
cp .env.example .env
#  éditer .env :
#   POSTGRES_PASSWORD    = openssl rand -hex 16
#   SECRET_KEY           = openssl rand -hex 32
#   MJQBE_ADMIN_PASSWORD = mot de passe admin initial (sinon "admin")
#   GOOGLE_/GITHUB_CLIENT_ID/SECRET si OAuth
#  éditer config/config.yml : server.domain, server.https, ports

./cli/dev install       # → /usr/local/bin/dev
dev up                  # build + démarre db / api / frontend / daemon
dev health              # doit être tout ✓
```

- Web : `http://<pi>:4444` (ou `server.web_port`).
- Compte admin initial : `admin` / `$MJQBE_ADMIN_PASSWORD` (défaut `admin`) — **le changer immédiatement** via l'UI si le défaut a été utilisé.

### HTTPS

1. `config/config.yml` → `server.https: true`, renseigner `cert_path` / `cert_key_path`.
2. Fournir les certificats (Let's Encrypt via `certbot`, ou auto-signé) et les **monter** dans le conteneur `frontend` :
   ```yaml
   # docker-compose.prod.yml, service frontend
   environment: { HTTPS: "1", CERT_PATH: /etc/nginx/certs/cert.pem, CERT_KEY_PATH: /etc/nginx/certs/key.pem }
   volumes: [ "/etc/letsencrypt/live/<domaine>:/etc/nginx/certs:ro" ]
   ports: [ "443:443", "80:80" ]
   ```
3. `dev up`. Le conteneur choisit `nginx.https.conf.template` (redirection 80→443 + HSTS).

### Sauvegardes

```bash
dev backup                       # → backups/mjqbe-<date>.sql.gz
# cron quotidien :
0 3 * * * cd /home/pi/MJQbe_v2 && /usr/local/bin/dev backup
dev restore backups/<fichier>    # restauration
```

---

## 3. App native (systemd)

```bash
dev native build     # cargo release + cmake release

sudo install -Dm755 native/core/target/release/mjqbe-core   /opt/mjqbe/bin/mjqbe-core
sudo install -Dm755 native/ui/build/mjqbe-native            /opt/mjqbe/bin/mjqbe-native
sudo install -Dm644 native/mjqbe-core.service   /etc/systemd/system/
sudo install -Dm644 native/mjqbe-native.service /etc/systemd/system/

sudo useradd -r -s /usr/sbin/nologin mjqbe 2>/dev/null || true
sudo usermod -aG gpio,video,input,docker mjqbe
sudo mkdir -p /etc/mjqbe
printf 'POSTGRES_HOST=127.0.0.1\nPOSTGRES_PORT=5432\nPOSTGRES_USER=mjqbe\nPOSTGRES_PASSWORD=...\nPOSTGRES_DB=mjqbe\n' \
  | sudo tee /etc/mjqbe/core.env >/dev/null
sudo chmod 600 /etc/mjqbe/core.env

sudo systemctl daemon-reload
sudo systemctl enable --now mjqbe-core.service mjqbe-native.service
```

> Le core parle à PostgreSQL. Soit publier le port du conteneur `db` sur `127.0.0.1:5432`
> (override), soit lancer un PostgreSQL système et pointer `DATABASE_URL` dessus.

### Daemon matériel

Le conteneur `daemon` détecte le Pi via `/proc/device-tree/model` et passe en mode **sysfs**
(sinon `MJQBE_GPIO_FORCE=1`). Câblage : relais 1–4 → BCM 23/24/25/12 ; LED RGB → 5/6/13 ;
IR sur le socket LIRC (`LIRC_SOCKET`) ; HC-05 sur `BT_SERIAL` (`/dev/serial0`).
`cec-client` (paquet `cec-utils`) pour le HDMI-CEC.

---

## 4. Sécurité (rappels)

- `.env` `chmod 600`, jamais commité. `SECRET_KEY` / `POSTGRES_PASSWORD` uniques par déploiement.
- API : en-têtes `X-Content-Type-Options`, `X-Frame-Options`, `CSP`, `Referrer-Policy` (+ HSTS si `https`).
- Rate limiting `auth.rate_limit_per_minute` (défaut 20/min/IP sur `/auth/login|register|refresh`).
- `/admin/*` + `/dev/*` : JWT **admin** ; `PUT /admin/config` et `/admin/reboot` : re-auth mot de passe.
- Le socket Docker est monté rw dans `api` (admin panel) — envisager `docker-socket-proxy` si exposition réseau.
- Requêtes SQL : SQLAlchemy ORM + paramètres liés + validation Pydantic partout → pas d'injection.

---

## 5. Mise à jour

```bash
cd MJQbe_v2 && git pull
dev backup
dev update            # pull images + rebuild + recreate
dev native build && sudo systemctl restart mjqbe-core mjqbe-native
dev health
```

---

## 6. Profiling mémoire (à faire sur le Pi)

```bash
docker stats --no-stream          # RAM par conteneur web
systemctl status mjqbe-native     # RSS du process natif ; cible < 150 Mo
htop                              # vue d'ensemble
# détail natif :
heaptrack /opt/mjqbe/bin/mjqbe-native   # ou valgrind --tool=massif
```
