# Sprint 16 — CLI `dev` complète

**Terminé le** 2026-08-31 (~03:30 CEST) · **Branche** `sprint-16-actions`
**Issues fermées** : #119–#127

## Fait (`cli/dev`)
| Commande | Détail |
|---|---|
| `dev native build` | `cargo build --release` (native/core) + `cmake -DCMAKE_BUILD_TYPE=Release` + `cmake --build` (native/ui) |
| `dev native start` | lance `mjqbe-core` (socket `/tmp/mjqbe-native.sock`, `DATABASE_URL` depuis `.env` + port 15432) puis `mjqbe-native` si le binaire existe ; pid dans `/tmp/*.pid`, logs `/tmp/mjqbe-*.log` |
| `dev native stop` | kill par pid + `pkill -x` |
| `dev backup` | `_compose exec -T db pg_dump --clean --if-exists \| gzip` → `backups/mjqbe-<horodatage>.sql.gz` |
| `dev restore <f>` | confirmation `y/N` ; `.gz` → `gunzip -c \| psql`, sinon `psql < f` |
| `dev logs [svc] [-l lvl]` | `error` = `error\|exception\|traceback\|critical\|fatal\|panic`, `warning` ajoute `warn`, `info` = `error\|warn\|info` ; `grep -Ei --line-buffered` |
| `dev health` (enrichi) | table des conteneurs + `pg_isready` + `curl :API_PORT/health` |
| `dev sprint [--push]` | 1) `cargo test` 2) `pytest` dans le conteneur `api` 3) `docker build frontend --target builder` + `tsc --noEmit` + `vitest` 4) `dev health` ; `git push origin dev` si `--push` et tout OK |
| `dev install` | `ln -sf <script> /usr/local/bin/dev` (tente `sudo` si non inscriptible) |
| `dev help` | réécrit, regroupé : Services Docker / Base de données / App native / Matériel / Agents |

`.gitignore` : `backups/`.

## Vérif
- `bash -n cli/dev` OK.
- `dev help` → liste complète groupée.
- `dev health` → `✓` api/db/frontend + `✓ PostgreSQL accepte les connexions` + `✓ API /health`.
- `dev backup` → `backups/mjqbe-20260831-011536.sql.gz` (8 Ko).
- `dev restore <ce fichier>` (auto-`y`) → `Restauration terminée`, `dev db \dt` → 7 tables.
- `dev logs api -l error` → suit + filtre (arrêté au timeout).

## Reste
- `dev native start` : jamais lancé pour de vrai ici (pas d'affichage) ; testable sur le Pi (#137).
- `dev install` : non exécuté (pas de sudo en session non-interactive) — la commande imprime l'instruction `sudo ln -sf …`.
- `dev sprint` : dispatch vérifié, run complet non relancé (les 3 suites de tests ont déjà tourné indépendamment ce jour).
