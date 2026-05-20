from contextlib import asynccontextmanager
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from alembic.config import Config as AlembicConfig
from alembic import command as alembic_command
import os
import yaml

from app.infrastructure.db.session import SessionLocal
from app.infrastructure.db import seed


def _load_config() -> dict:
    path = os.getenv("CONFIG_PATH", "/app/config/config.yml")
    try:
        with open(path) as f:
            return yaml.safe_load(f) or {}
    except FileNotFoundError:
        return {}


def _run_migrations() -> None:
    alembic_cfg = AlembicConfig(os.path.join(os.path.dirname(__file__), "..", "alembic.ini"))
    alembic_cfg.set_main_option("script_location", os.path.join(os.path.dirname(__file__), "..", "alembic"))
    alembic_command.upgrade(alembic_cfg, "head")


def _run_seed() -> None:
    db = SessionLocal()
    try:
        seed.run(db)
    finally:
        db.close()


@asynccontextmanager
async def lifespan(app: FastAPI):
    _run_migrations()
    _run_seed()
    yield


_config  = _load_config()
_srv     = _config.get("server", {})
_web_port = _srv.get("web_port", 4444)
_domain  = _srv.get("domain", "") or ""
_https   = _srv.get("https", False)

_allowed_origins = [
    f"http://localhost:{_web_port}",
    "http://localhost:5173",
]
if _domain:
    scheme = "https" if _https else "http"
    _allowed_origins.append(f"{scheme}://{_domain}")
    _allowed_origins.append(f"{scheme}://{_domain}:{_web_port}")

app = FastAPI(title="MJQbe API", version="2.0.0", lifespan=lifespan)

app.add_middleware(
    CORSMiddleware,
    allow_origins=_allowed_origins,
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


@app.get("/health")
def health():
    return {"status": "ok"}


@app.get("/config/port")
def config_port():
    return {
        "web_port": _srv.get("web_port", 4444),
        "api_port": _srv.get("api_port", 4848),
    }
