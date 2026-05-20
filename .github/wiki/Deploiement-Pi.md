# Déploiement sur Raspberry Pi 4

## Prérequis Pi

```bash
# OS recommandé : Raspberry Pi OS Lite 64-bit (Bookworm)
# RAM : 4 Go minimum

# Docker
curl -fsSL https://get.docker.com | sh
sudo usermod -aG docker $USER
sudo chown root:docker /var/run/docker.sock

# Rust (pour compiler native/core)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Qt6 (pour compiler native/ui)
sudo apt-get install -y qt6-base-dev qt6-declarative-dev cmake build-essential
```

## Déploiement interface web

```bash
git clone https://github.com/MoggleJ/MJQbe_v2.git
cd MJQbe_v2
cp .env.example .env
# Éditer .env

cli/dev up
# → http://<IP-du-pi>:4444
```

## Build app native sur Pi

```bash
# Rust core
cd native/core
cargo build --release

# Qt6 UI
mkdir native/ui/build && cd native/ui/build
cmake .. -DCMAKE_BUILD_TYPE=Release
cmake --build . --parallel $(nproc)

# Installer
sudo make install
# → /usr/local/bin/mjqbe-native

# Systemd
sudo systemctl enable mjqbe-native
sudo systemctl start mjqbe-native
```

## Objectifs performance

| Métrique | Cible |
|---|---|
| RAM app native | < 150 Mo |
| Temps démarrage native | < 3s |
| FPS animations | > 30 fps |
| RAM stack web | < 512 Mo total |

## Mise à jour

```bash
git pull origin main
cli/dev update    # rebuild + restart containers
# Pour le natif : rebuild + systemctl restart mjqbe-native
```

## Troubleshooting Pi

```bash
# Logs app native
journalctl -u mjqbe-native -f

# Logs Docker
dev logs

# Température Pi
cat /sys/class/thermal/thermal_zone0/temp
# (diviser par 1000 pour °C)

# Utilisation mémoire
free -h && docker stats --no-stream
```
