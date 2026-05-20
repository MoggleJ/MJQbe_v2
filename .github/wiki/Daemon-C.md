# Daemon C — Contrôle matériel

Le daemon C est le seul service qui accède directement au hardware du Pi.  
Il est partagé entre l'app native (client Rust) et l'interface web (client Python).

## Socket

```
/run/mjqbe/daemon.sock
```

## Protocole JSON

Requête :
```json
{ "action": "gpio_set", "payload": { "pin": 23, "value": 1 } }
```
Réponse :
```json
{ "ok": true }
```

## Actions

| Action | Payload | Description |
|---|---|---|
| `gpio_set` | `{pin, value}` | Écrire une pin GPIO |
| `gpio_get` | `{pin}` | Lire une pin GPIO |
| `relay_set` | `{relay_id, state}` | Contrôler un relais |
| `led_set` | `{r, g, b}` | LED RGB |
| `ir_send` | `{code}` | Envoyer code IR |
| `cec_command` | `{command}` | Commande HDMI CEC |
| `tv_on` / `tv_off` | — | Allumer/éteindre la TV via CEC |
| `ps4_on` / `ps4_off` | — | Allumer/éteindre la PS4 via CEC |

## Build

```bash
cd daemon
make
```

## Docker

Le daemon tourne en mode `privileged` pour accéder au hardware :
```yaml
daemon:
  privileged: true
  volumes:
    - daemon-socket:/run/mjqbe
```

## Mapping GPIO (config/config.yml)

```yaml
hardware:
  relay_pins: [23, 24, 25]   # GPIO des relais
  led_pin: 18                # GPIO LED RGB
  ir_pin: 17                 # GPIO réception IR
```

## Compatibilité

| Matériel | Interface | Status |
|---|---|---|
| Relais 5V | GPIO sysfs | Sprint 07 |
| Télécommande IR | GPIO18 / LIRC | Sprint 08 |
| TV HDMI | CEC via libCEC | Sprint 08 |
| PS4 | CEC ou Bluetooth | Sprint 08 |
| HC-05 Bluetooth | UART /dev/ttyAMA0 | Sprint 08 |
| Micro ISD1820 | GPIO | Sprint 09 |
