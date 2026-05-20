# CLAUDE.md — Instructions for Claude Code agents

## Working directory
All work happens exclusively in `/MJQbe_v2/`. Never touch sibling directories (`MJQbe_app`, `MJQbe_web`).

## Project summary
MJQbe is an embedded hub application running on Raspberry Pi 4.
- 3 modes: TV, Desktop, Dev
- Web interface (TV + Desktop only) with OAuth users and admin panel
- Hardware control (GPIO, IR remote, Bluetooth, voice recognition)
- Full Docker deployment

## Tech stack
| Layer | Tech |
|---|---|
| Backend API | Python 3.11 + FastAPI |
| Hardware daemon | C |
| Frontend web | React 18 + Vite |
| Database | PostgreSQL 15 |
| CLI | Bash (`dev`) |
| Containers | Docker + Docker Compose |
| Auth | JWT + OAuth 2.0 (Google + GitHub) |

## Key files to read at session start
1. `docs/CDC.md` — full specifications
2. `docs/plan-implementation.md` — current sprint and tasks
3. `problemes.md` — known problems and their solutions
4. `agents/AGENTS.md` — multi-agent coordination rules

## Git conventions
- Main development branch: `dev`
- Sprint branches: `sprint-01-actions`, `sprint-02-actions`, etc.
- Never commit directly to `main`
- See `agents/sprint-workflow.md` for the full sprint loop

## Problem tracking
When you encounter a blocker or unexpected behavior:
1. Add an entry to `problemes.md` immediately
2. Include the problem description, your solution, and status
3. Keep it in context for the rest of the session

## Architecture rules
- Clean Architecture: Domain → Application → Infrastructure → Interface
- Frontend never accesses the database directly (always through API)
- Each Docker service has a single responsibility
- Admin-level operations require explicit re-authentication

## Sprint workflow (summary)
See `agents/sprint-workflow.md` for the full procedure.
Short version: implement → compare with specs → test → fix → push to `sprint-XX-actions`.
