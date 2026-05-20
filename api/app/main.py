from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
import os
import yaml

app = FastAPI(title="MJQbe API", version="2.0.0")


def _load_config() -> dict:
    path = os.getenv("CONFIG_PATH", "/app/config/config.yml")
    try:
        with open(path) as f:
            return yaml.safe_load(f) or {}
    except FileNotFoundError:
        return {}


_config = _load_config()
_allowed_origins = [
    f"http://localhost:{_config.get('server', {}).get('web_port', 8484)}",
    "http://localhost:5173",  # vite dev server
]

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
        "web_port": _config.get("server", {}).get("web_port", 8484),
        "api_port": _config.get("server", {}).get("api_port", 4848),
    }
