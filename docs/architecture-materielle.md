# Architecture matérielle — MJQbe v2

## 1. Matériel déjà disponible

| Composant | Quantité | Rôle dans MJQbe |
|---|---|---|
| Raspberry Pi 4 Model B (4 Go) | 1 | Cerveau du système |
| HC-05 Bluetooth module | 1 | Communication BT (télécommande, appareils) |
| ISD1820 Voice Recording Module | 1 | Capture vocale (wake word + commandes) |
| Arduino Uno | 1+ | Pont GPIO / capteurs si besoin |
| Arduino Nano | 1+ | Pont GPIO compact |
| Résistances, condensateurs, LEDs | — | Circuits de base |
| Breadboards | — | Prototypage |

---

## 2. Matériel à acheter

| Composant | Utilisation | Lien de référence | Quantité |
|---|---|---|---|
| Récepteur IR TSOP38238 (ou TSOP4838) | Réception télécommande IR | Chercher "TSOP38238" sur Amazon/AliExpress | 1 |
| Télécommande IR universelle | Allumage du hub sans écran | N'importe quelle télécommande IR 38kHz | 1 |
| Relais 5V (module 4 relais) | Contrôle prises/appareils secteur | "Relay module 5V 4ch" | 1 |
| Câble HDMI (CEC compatible) | Contrôle TV et PS4 via HDMI CEC | Câble HDMI standard ≥ 1.4 | 1 |
| Micro USB ou jack 3.5mm microphone | Alternative à ISD1820 pour reconnaissance vocale | Micro USB simple | 1 (optionnel) |
| Dissipateur thermique + ventilateur Pi 4 | Refroidissement sous charge | "Raspberry Pi 4 heatsink fan" | 1 |
| Boîtier Raspberry Pi 4 | Protection + montage | Selon préférence | 1 |

---

## 3. Brochage (pin mapping)

### Raspberry Pi 4 — GPIO utilisés

```
Pin physique | GPIO | Usage
─────────────────────────────────────────────────
8  (TXD)     | GPIO14 | HC-05 RX  (UART TX du Pi)
10 (RXD)     | GPIO15 | HC-05 TX  (UART RX du Pi)
11           | GPIO17 | HC-05 EN  (mode AT si besoin)
12           | GPIO18 | Récepteur IR TSOP38238 (signal)
13           | GPIO27 | ISD1820 REC (déclenche enregistrement)
15           | GPIO22 | ISD1820 PLAYE (lecture)
16           | GPIO23 | Relais canal 1 (prise 1)
18           | GPIO24 | Relais canal 2 (prise 2)
22           | GPIO25 | Relais canal 3 (prise 3 / LED)
29           | GPIO5  | Relais canal 4 (prise 4 / LED)
2            | 5V     | Alimentation HC-05, module relais
4            | 5V     | Alimentation ISD1820
6, 14, 20    | GND    | Masse commune
```

### HC-05 (Bluetooth UART)

```
HC-05 Pin | Raspberry Pi Pin
──────────────────────────────
VCC       | 5V (pin 2)
GND       | GND (pin 6)
TXD       | GPIO15 / RXD (pin 10)
RXD       | GPIO14 / TXD (pin 8)  [via diviseur de tension 3.3V]
EN        | GPIO17 (pin 11)  [optionnel, config AT]
```

> Note : Le HC-05 fonctionne en 3.3V logique. Le TX du Pi (3.3V) est compatible directement. Le TX du HC-05 (5V) vers le RX du Pi (3.3V) nécessite un diviseur de tension (résistances 1kΩ + 2kΩ).

### ISD1820 (Reconnaissance vocale)

```
ISD1820 Pin | Raspberry Pi Pin
──────────────────────────────
VCC         | 5V (pin 4)
GND         | GND (pin 14)
REC         | GPIO27 (pin 13)  [active HIGH]
PLAYE       | GPIO22 (pin 15)  [playback edge trigger]
```

### Récepteur IR TSOP38238

```
TSOP38238 Pin | Raspberry Pi Pin
──────────────────────────────────
VCC (pin 2)   | 3.3V (pin 1)
GND (pin 1)   | GND (pin 20)
OUT (pin 3)   | GPIO18 (pin 12)  [signal démodulé]
```

### Module relais 5V (4 canaux)

```
Relais Pin | Raspberry Pi Pin
──────────────────────────────
VCC        | 5V (pin 2)
GND        | GND (pin 6)
IN1        | GPIO23 (pin 16)
IN2        | GPIO24 (pin 18)
IN3        | GPIO25 (pin 22)
IN4        | GPIO5  (pin 29)
```

---

## 4. HDMI CEC (contrôle TV et PS4)

HDMI CEC est un protocole intégré dans le câble HDMI. Aucun composant supplémentaire n'est nécessaire si la TV et la PS4 sont connectées au Pi via HDMI.

**Logiciel :** `cec-utils` (libCEC) — contrôle l'alimentation et le volume via la ligne CEC du câble HDMI.

```bash
# Allumer la TV
echo "on 0" | cec-client -s -d 1

# Éteindre
echo "standby 0" | cec-client -s -d 1
```

Le daemon C envoie ces commandes quand l'API reçoit une action `tv_on` / `tv_off`.

---

## 5. Schéma de câblage simplifié

```
Raspberry Pi 4
│
├── UART (GPIO14/15) ──────────── HC-05 [Bluetooth]
│                                   └── Télécommande BT / Appareils
│
├── GPIO18 ────────────────────── TSOP38238 [IR Receiver]
│                                   └── Télécommande IR
│
├── GPIO27/22 ──────────────────── ISD1820 [Voice]
│                                   └── Micro intégré
│
├── GPIO23/24/25/5 ─────────────── Module relais x4
│                                   ├── Prise 1
│                                   ├── Prise 2
│                                   ├── Prise 3
│                                   └── LED / Ampoule
│
└── HDMI ───────────────────────── TV / PS4 [CEC]
```

---

## 6. Notes et précautions

- Le Pi 4 délivre max 50mA par GPIO. Le module relais (bobines) consomme plus : toujours utiliser les pins 5V/GND pour l'alimentation du module relais, pas les GPIO.
- Prévoir un dissipateur thermique : le Pi 4 chauffe sous charge Docker.
- Le HC-05 en mode données (pas AT) communique à 9600 baud par défaut.
- L'ISD1820 enregistre ~8-10 secondes. Pour la reconnaissance vocale continue, préférer un micro USB + librairie Python (Vosk ou Whisper).
- La PS4 supporte le CEC de manière limitée selon les modèles. Tester avant d'intégrer.
