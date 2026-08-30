"""Authentication use cases (local + OAuth). No FastAPI here."""
from dataclasses import dataclass

from app.domain.entities import User
from app.infrastructure.db.user_repo import UserRepository
from app.infrastructure.oauth.providers import OAuthUser
from app.infrastructure.security.passwords import hash_password, verify_password
from app.infrastructure.security.tokens import (
    create_access_token,
    create_refresh_token,
    decode_token,
)


class AuthError(Exception):
    def __init__(self, message: str, status: int = 400):
        super().__init__(message)
        self.status = status


@dataclass
class TokenPair:
    access_token: str
    refresh_token: str
    token_type: str = "bearer"


class AuthService:
    def __init__(self, users: UserRepository, settings=None):
        self.users = users
        self.settings = settings  # optional SettingsRepository

    def _ensure_settings(self, user: User) -> None:
        if self.settings is not None:
            self.settings.get_or_create(user.id)

    # --- local -----------------------------------------------------------
    def register(self, username: str, password: str, email: str | None) -> User:
        if len(username) < 3 or len(password) < 8:
            raise AuthError("username >= 3 and password >= 8 chars required", 422)
        if self.users.get_by_username(username):
            raise AuthError("username already taken", 409)
        if email and self.users.get_by_email(email):
            raise AuthError("email already registered", 409)
        user = self.users.create(
            username=username, email=email, password_hash=hash_password(password)
        )
        self._ensure_settings(user)
        return user

    def login(self, username: str, password: str) -> tuple[User, TokenPair]:
        user = self.users.get_by_username(username)
        if not user or not verify_password(password, user.password_hash):
            raise AuthError("invalid credentials", 401)
        self.users.touch_last_login(user)
        return user, self._issue(user)

    def refresh(self, refresh_token: str) -> TokenPair:
        try:
            payload = decode_token(refresh_token, expected_type="refresh")
        except Exception as exc:  # noqa: BLE001
            raise AuthError("invalid refresh token", 401) from exc
        user = self.users.get(int(payload["sub"]))
        if not user:
            raise AuthError("user no longer exists", 401)
        return self._issue(user)

    # --- oauth ---------------------------------------------------------------
    def oauth_upsert(self, info: OAuthUser) -> tuple[User, TokenPair]:
        user = self.users.get_by_oauth(info.provider, info.oauth_id)
        if not user and info.email:
            user = self.users.get_by_email(info.email)
            if user and not user.oauth_provider:
                user.oauth_provider = info.provider
                user.oauth_id = info.oauth_id
                self.users.db.commit()
        if not user:
            username = self._unique_username(info.username)
            user = self.users.create(
                username=username,
                email=info.email,
                oauth_provider=info.provider,
                oauth_id=info.oauth_id,
            )
            self._ensure_settings(user)
        self.users.touch_last_login(user)
        return user, self._issue(user)

    def _unique_username(self, base: str) -> str:
        base = (base or "user").strip()[:56] or "user"
        candidate = base
        i = 1
        while self.users.get_by_username(candidate):
            i += 1
            candidate = f"{base}{i}"
        return candidate

    def _issue(self, user: User) -> TokenPair:
        return TokenPair(
            access_token=create_access_token(user.id, user.role),
            refresh_token=create_refresh_token(user.id, user.role),
        )
