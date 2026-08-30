"""JWT access + refresh tokens (HS256).

Secret comes from ``SECRET_KEY`` (env). Lifetimes come from ``config.yml``
(``auth.access_token_expire_minutes`` / ``auth.refresh_token_expire_days``).
"""
import os
from datetime import datetime, timedelta, timezone

import jwt

ALGORITHM = "HS256"


class TokenError(Exception):
    pass


def _secret() -> str:
    secret = os.getenv("SECRET_KEY")
    if not secret:
        raise RuntimeError("SECRET_KEY is not set")
    return secret


def _cfg() -> dict:
    from app.interface.deps import get_config

    return get_config().get("auth", {})


def _make(sub: str, role: str, token_type: str, expires: timedelta) -> str:
    now = datetime.now(timezone.utc)
    payload = {
        "sub": str(sub),
        "role": role,
        "type": token_type,
        "iat": now,
        "exp": now + expires,
    }
    return jwt.encode(payload, _secret(), algorithm=ALGORITHM)


def create_access_token(sub: str, role: str) -> str:
    minutes = int(_cfg().get("access_token_expire_minutes", 60))
    return _make(sub, role, "access", timedelta(minutes=minutes))


def create_refresh_token(sub: str, role: str) -> str:
    days = int(_cfg().get("refresh_token_expire_days", 30))
    return _make(sub, role, "refresh", timedelta(days=days))


def decode_token(token: str, expected_type: str = "access") -> dict:
    try:
        payload = jwt.decode(token, _secret(), algorithms=[ALGORITHM])
    except jwt.ExpiredSignatureError as exc:
        raise TokenError("token expired") from exc
    except jwt.PyJWTError as exc:
        raise TokenError("invalid token") from exc
    if payload.get("type") != expected_type:
        raise TokenError(f"expected a {expected_type} token")
    return payload
