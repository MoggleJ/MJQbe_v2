# Sprint 17 — Sécurité, optimisation & déploiement final

**Terminé le** 2026-08-31 (~03:55 CEST) · **Branche** `sprint-17-actions`
**Issues fermées** : #128, #129, #131–#136 · **reste** : #130 (profiling mémoire sur Pi réel)

## Fait

### Sécurité web (`api/app/interface/security_mw.py`)
- **`SecurityHeadersMiddleware`** : `X-Content-Type-Options: nosniff`, `X-Frame-Options: DENY`, `Referrer-Policy: no-referrer`, `Cross-Origin-Opener-Policy`, `Content-Security-Policy: default-src 'none'; frame-ancestors 'none'`, `Permissions-Policy` ; `Strict-Transport-Security` si `server.https`.
- **`RateLimitMiddleware`** : fenêtre glissante 60 s par `IP|path` sur `/auth/login|register|refresh` ; dépasse `auth.rate_limit_per_minute` (défaut 20) → **429** + `Retry-After`. Override test `AUTH_RATE_LIMIT_PER_MIN`.
- **CORS** resserré : `allow_methods=[GET,POST,PUT,DELETE,OPTIONS]`, `allow_headers=[Authorization,Content-Type]`, `max_age=600`.
- **Injection SQL** : SQLAlchemy ORM + paramètres liés + validation Pydantic (`Literal`, bornes) partout. Test `test_security.py::test_sql_injection_attempt_is_harmless`.
- `config/config.yml` : `auth.rate_limit_per_minute: 20`.

### HTTPS
- `frontend/nginx.conf.template` (HTTP + headers) inchangé pour le défaut ; **`frontend/nginx.https.conf.template`** (redirection 80→443, `ssl_protocols TLSv1.2/1.3`, HSTS, proxy `/api` en `X-Forwarded-Proto https`).
- `frontend/Dockerfile` : `EXPOSE 80 443`, envsubst choisit `http`/`https` selon `HTTPS` (env), variables `CERT_PATH`/`CERT_KEY_PATH`.

### Optimisation images
- `frontend` : builder `node:20-alpine` → `nginx:alpine`. `daemon` : builder `debian-slim` → `debian-slim` (+`libcjson1`, `cec-utils`). `api` : `python:3.11-slim` mono-stage — pas de toolchain à retirer, `--no-cache-dir`, acceptable.

### Docs & outillage
- **`docs/deploiement.md`** : procédure Pi complète (Docker + systemd + HTTPS + backups + sécurité + MAJ + profiling).
- **`docs/revue-finale.md`** : checklist CDC §2–§10 (✅/🟡/⏳ + issues de suivi).
- **`scripts/loadtest.sh`** : `hey`/`wrk` sinon fallback curl séquentiel sur `/health`, `/apps`, `/categories`.

### CI
`api-ci` + `docker-build` + `native-build` verts sur chaque push (mis en place Sprint 1, réparé au commit `fix(ci)` — P12).

## Vérifications (live, stack Docker)
- En-têtes sécu présents sur `GET /health` (API) **et** `GET /` (nginx).
- 25× `POST /auth/login` → `401 ×20` puis `429 ×5` (limite = 20/min).
- `scripts/loadtest.sh` : ~33–38 req/s (fallback curl séquentiel), 0 échec.
- `flake8 app tests` clean ; **`pytest` 39/39** (+`test_security.py` : headers, rate-limit isolé, injection).
- Pas de régression : `/dev/hardware` (Bearer admin) → 200 à travers la nouvelle pile de middlewares.

## Reste
- **#130** — `docker stats` / `heaptrack` / `htop` sur le **Pi 4** : impossible sans matériel. Méthode dans `docs/deploiement.md §6`.
- HTTPS : template + Dockerfile + doc prêts ; test avec un **vrai certificat** (Let's Encrypt) à faire au déploiement.
- Tests de charge : outil léger ; pour du vrai, installer `hey`/`k6`.
- `api` mono-stage : passage multi-stage possible mais gain marginal (`slim` sans build-deps).

## GitGuardian
Alerte « internal secret » levée pendant le sprint sur `cli/dev` (chaîne `postgres://…:${POSTGRES_PASSWORD:-mjqbe}@…` introduite au Sprint 16). **Corrigé** : `dev native start` ne compose plus d'URL — il passe `POSTGRES_HOST`/`POSTGRES_PORT` et laisse le core lire `POSTGRES_*` depuis `.env`. `.gitguardian.yaml` : +`cli/**`.
