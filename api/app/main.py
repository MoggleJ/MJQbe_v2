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
_srv = _config.get("server", {})
_web_port = _srv.get("web_port", 4443)
_domain = _srv.get("domain", "") or ""
_https = _srv.get("https", False)

_allowed_origins = [
    f"http://localhost:{_web_port}",
    "http://localhost:5173",
]
if _domain:
    scheme = "https" if _https else "http"
    _allowed_origins.append(f"{scheme}://{_domain}")
    _allowed_origins.append(f"{scheme}://{_domain}:{_web_port}")

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
        "web_port": _srv.get("web_port", 4443),
        "api_port": _srv.get("api_port", 4848),
    }
