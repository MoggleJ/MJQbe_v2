# daemon/ — mjqbe-daemon (contrôle matériel, C)

Processus C qui possède le matériel bas niveau. Écoute un socket Unix
(`DAEMON_SOCKET`, défaut `/run/mjqbe/daemon.sock`), un objet JSON par ligne.

Clients : `native/core` (Rust) et l'API FastAPI (Python) — même protocole.

## Protocole

```
→ {"id":"7","cmd":"gpio_set","pin":23,"value":1}
← {"id":"7","ok":true,"data":{"pin":23,"value":1}}
← {"id":"7","ok":false,"error":"bad pin/value"}
```

| cmd | params | data |
|---|---|---|
| `ping` | — | `{"pong":true}` |
| `info` | — | `{"backend":"sysfs\|stub","pi":bool,"relays":4}` |
| `gpio_set` | `pin` (0–53), `value` (0/1) | `{"pin","value"}` |
| `gpio_get` | `pin` | `{"pin","value"}` |
| `relay_set` | `relay` (1–4), `state` (0/1) | `{"relay","state","pin"}` |
| `led_set` | `r`, `g`, `b` (0/1, ou >0) | `{"r","g","b"}` |

## GPIO

- Backend **sysfs** (`/sys/class/gpio`). Pas de dépendance libgpiod.
- **Mode stub** automatique si `/sys/class/gpio` absent ou `MJQBE_GPIO_STUB=1` :
  les requêtes sont validées et renvoyées sans toucher au matériel (dev hors-Pi).

### Câblage (BCM)

| Relais | GPIO |
|---|---|
| 1 | 23 |
| 2 | 24 |
| 3 | 25 |
| 4 | 12 |

LED RGB : `MJQBE_LED_R` / `_G` / `_B` (défaut 5 / 6 / 13).
Cartes relais généralement **actives à l'état bas** → `state:1` écrit `0` sur la pin.
Forcer l'inverse : `MJQBE_RELAY_ACTIVE_HIGH=1`.

## Build / test

```bash
# via Docker (recommandé — libcjson fournie par l'image)
docker build ./daemon -t mjqbe-daemon
docker run --rm -e MJQBE_GPIO_STUB=1 -v /tmp/d:/run/mjqbe mjqbe-daemon &
printf '{"cmd":"ping"}\n{"cmd":"gpio_set","pin":23,"value":1}\n' | nc -U /tmp/d/daemon.sock
```

Sprints suivants : IR + CEC + Bluetooth (Sprint 8), déclenchement vocal (Sprint 9).
