"""Admin panel endpoints — every route requires an admin JWT.

System operations (config write, service restart, reboot) additionally require
re-authentication (the admin's own password in the request body).
"""
import threading

from fastapi import APIRouter, Depends, HTTPException, Query, status
from pydantic import BaseModel
from sqlalchemy.orm import Session

from app.domain.entities import User
from app.infrastructure.config_file import ConfigError, read_config, write_config
from app.infrastructure.db.user_data_repo import LogRepository, list_users
from app.infrastructure.docker_client import DockerUnavailable, list_services, service_action
from app.interface.deps import get_current_user, get_db, require_admin, verify_reauth

router = APIRouter(prefix="/admin", tags=["admin"], dependencies=[Depends(require_admin)])


class ReauthBody(BaseModel):
    password: str


class ConfigWriteBody(BaseModel):
    config: dict
    password: str


# --- logs / users --------------------------------------------------------------

@router.get("/logs")
def get_logs(
    db: Session = Depends(get_db),
    limit: int = Query(default=50, ge=1, le=500),
    offset: int = Query(default=0, ge=0),
):
    repo = LogRepository(db)
    rows = repo.list(limit=limit, offset=offset)
    return {
        "total": repo.count(),
        "limit": limit,
        "offset": offset,
        "items": [
            {
                "id": r.id,
                "user_id": r.user_id,
                "action": r.action,
                "metadata": r.meta,
                "created_at": r.created_at.isoformat(),
            }
            for r in rows
        ],
    }


@router.get("/users")
def get_users(db: Session = Depends(get_db)):
    return [
        {
            "id": u.id,
            "username": u.username,
            "email": u.email,
            "role": u.role,
            "oauth_provider": u.oauth_provider,
            "created_at": u.created_at.isoformat() if u.created_at else None,
            "last_login": u.last_login.isoformat() if u.last_login else None,
        }
        for u in list_users(db)
    ]


# --- config ------------------------------------------------------------------

@router.get("/config")
def get_config_file():
    try:
        return read_config()
    except ConfigError as exc:
        raise HTTPException(status.HTTP_404_NOT_FOUND, str(exc))


@router.put("/config")
def put_config_file(body: ConfigWriteBody, user: User = Depends(get_current_user)):
    verify_reauth(user, body.password)
    try:
        return write_config(body.config)
    except ConfigError as exc:
        raise HTTPException(422, str(exc))


# --- docker services --------------------------------------------------------

@router.get("/services")
def get_services():
    try:
        return list_services()
    except DockerUnavailable as exc:
        raise HTTPException(status.HTTP_503_SERVICE_UNAVAILABLE, str(exc))


@router.post("/services/{name}/restart", status_code=status.HTTP_202_ACCEPTED)
def restart_service(name: str):
    return _do_service(name, "restart")


@router.post("/services/{name}/stop", status_code=status.HTTP_202_ACCEPTED)
def stop_service(name: str):
    return _do_service(name, "stop")


def _do_service(name: str, action: str):
    try:
        service_action(name, action)
    except DockerUnavailable as exc:
        raise HTTPException(status.HTTP_503_SERVICE_UNAVAILABLE, str(exc))
    return {"service": name, "action": action, "accepted": True}


@router.post("/reboot", status_code=status.HTTP_202_ACCEPTED)
def reboot_all(reauth: ReauthBody, user: User = Depends(get_current_user)):
    verify_reauth(user, reauth.password)
    try:
        services = [s["service"] for s in list_services()]
    except DockerUnavailable as exc:
        raise HTTPException(status.HTTP_503_SERVICE_UNAVAILABLE, str(exc))

    others = [s for s in services if s != "api"]
    for svc in others:
        try:
            service_action(svc, "restart")
        except DockerUnavailable:
            pass
    # Restart api last, off the request thread, so this response can flush first.
    if "api" in services:
        threading.Timer(1.0, lambda: _safe_restart("api")).start()
    return {"restarting": services}


def _safe_restart(svc: str) -> None:
    try:
        service_action(svc, "restart")
    except Exception:  # noqa: BLE001
        pass
