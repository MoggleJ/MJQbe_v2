"""Talk to the Docker Engine API over the mounted ``/var/run/docker.sock``.

Used by the admin "system" endpoints to inspect and restart the compose
services. Every call degrades to a clear error when the socket is absent.
"""
import os

import httpx

_SOCK = os.getenv("DOCKER_SOCKET", "/var/run/docker.sock")
_BASE = "http://docker"


class DockerUnavailable(RuntimeError):
    pass


def _client() -> httpx.Client:
    if not os.path.exists(_SOCK):
        raise DockerUnavailable(f"docker socket not found at {_SOCK}")
    return httpx.Client(transport=httpx.HTTPTransport(uds=_SOCK), base_url=_BASE, timeout=15)


def _project() -> str:
    # compose sets com.docker.compose.project; default matches the repo folder.
    return os.getenv("COMPOSE_PROJECT_NAME", "mjqbe_v2")


def list_services() -> list[dict]:
    with _client() as c:
        r = c.get("/containers/json", params={"all": "true"})
        r.raise_for_status()
        out = []
        for ct in r.json():
            labels = ct.get("Labels", {})
            if labels.get("com.docker.compose.project") != _project():
                continue
            out.append(
                {
                    "service": labels.get("com.docker.compose.service", ct["Names"][0].lstrip("/")),
                    "name": ct["Names"][0].lstrip("/"),
                    "id": ct["Id"][:12],
                    "image": ct["Image"],
                    "state": ct["State"],
                    "status": ct["Status"],
                }
            )
        return sorted(out, key=lambda s: s["service"])


def _resolve(service: str) -> str:
    for s in list_services():
        if s["service"] == service or s["name"] == service:
            return s["id"]
    raise DockerUnavailable(f"no compose service '{service}'")


def service_action(service: str, action: str) -> None:
    if action not in {"restart", "stop", "start"}:
        raise ValueError(f"unsupported action {action}")
    cid = _resolve(service)
    with _client() as c:
        r = c.post(f"/containers/{cid}/{action}", params={"t": "10"})
        if r.status_code not in (204, 304):
            r.raise_for_status()
