"""FastAPI dependencies: DB session, config, current user, admin guard."""
import os
from functools import lru_cache

import yaml
from fastapi import Depends, HTTPException, status
from fastapi.security import HTTPAuthorizationCredentials, HTTPBearer
from sqlalchemy.orm import Session

from app.domain.entities import User
from app.infrastructure.db.session import SessionLocal
from app.infrastructure.db.user_repo import UserRepository
from app.infrastructure.security.tokens import TokenError, decode_token

_bearer = HTTPBearer(auto_error=False)


@lru_cache
def get_config() -> dict:
    path = os.getenv("CONFIG_PATH", "/app/config/config.yml")
    try:
        with open(path) as f:
            return yaml.safe_load(f) or {}
    except FileNotFoundError:
        return {}


def get_db():
    db = SessionLocal()
    try:
        yield db
    finally:
        db.close()


def get_users(db: Session = Depends(get_db)) -> UserRepository:
    return UserRepository(db)


def get_current_user(
    creds: HTTPAuthorizationCredentials | None = Depends(_bearer),
    users: UserRepository = Depends(get_users),
) -> User:
    if creds is None:
        raise HTTPException(status.HTTP_401_UNAUTHORIZED, "missing bearer token")
    try:
        payload = decode_token(creds.credentials, expected_type="access")
    except TokenError as exc:
        raise HTTPException(status.HTTP_401_UNAUTHORIZED, str(exc))
    user = users.get(int(payload["sub"]))
    if user is None:
        raise HTTPException(status.HTTP_401_UNAUTHORIZED, "unknown user")
    return user


def require_admin(user: User = Depends(get_current_user)) -> User:
    if user.role != "admin":
        raise HTTPException(status.HTTP_403_FORBIDDEN, "admin role required")
    return user


def get_optional_user(
    creds: HTTPAuthorizationCredentials | None = Depends(_bearer),
    users: UserRepository = Depends(get_users),
) -> User | None:
    """Like get_current_user but returns None instead of raising (for logging)."""
    if creds is None:
        return None
    try:
        payload = decode_token(creds.credentials, expected_type="access")
    except TokenError:
        return None
    return users.get(int(payload["sub"]))


def verify_reauth(user: User, password: str) -> None:
    """Re-check the caller's own password (for destructive admin actions)."""
    from app.infrastructure.security.passwords import verify_password

    if not verify_password(password, user.password_hash):
        raise HTTPException(status.HTTP_401_UNAUTHORIZED, "re-authentication failed")
