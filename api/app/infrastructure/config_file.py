"""Read / write ``config/config.yml`` for the admin panel."""
import os

import yaml

_REQUIRED_TOP_KEYS = {"server"}


def _path() -> str:
    return os.getenv("CONFIG_PATH", "/app/config/config.yml")


class ConfigError(RuntimeError):
    pass


def read_config() -> dict:
    try:
        with open(_path()) as f:
            return yaml.safe_load(f) or {}
    except FileNotFoundError as exc:
        raise ConfigError(f"config file not found: {_path()}") from exc


def write_config(new: dict) -> dict:
    if not isinstance(new, dict):
        raise ConfigError("config must be a mapping")
    missing = _REQUIRED_TOP_KEYS - new.keys()
    if missing:
        raise ConfigError(f"missing required top-level keys: {sorted(missing)}")
    server = new.get("server")
    if not isinstance(server, dict) or "web_port" not in server or "api_port" not in server:
        raise ConfigError("server.web_port and server.api_port are required")

    path = _path()
    try:
        with open(path, "w") as f:
            yaml.safe_dump(new, f, sort_keys=False, allow_unicode=True)
    except OSError as exc:
        raise ConfigError(f"cannot write {path}: {exc}") from exc
    return read_config()
