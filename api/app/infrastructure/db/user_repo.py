"""User persistence — the only place that touches the ``users`` table."""
from datetime import datetime, timezone

from sqlalchemy import select
from sqlalchemy.orm import Session

from app.domain.entities import User


class UserRepository:
    def __init__(self, db: Session):
        self.db = db

    def get(self, user_id: int) -> User | None:
        return self.db.get(User, user_id)

    def get_by_username(self, username: str) -> User | None:
        return self.db.execute(
            select(User).where(User.username == username)
        ).scalar_one_or_none()

    def get_by_email(self, email: str) -> User | None:
        return self.db.execute(
            select(User).where(User.email == email)
        ).scalar_one_or_none()

    def get_by_oauth(self, provider: str, oauth_id: str) -> User | None:
        return self.db.execute(
            select(User).where(
                User.oauth_provider == provider, User.oauth_id == oauth_id
            )
        ).scalar_one_or_none()

    def create(
        self,
        *,
        username: str,
        email: str | None = None,
        password_hash: str | None = None,
        role: str = "user",
        oauth_provider: str | None = None,
        oauth_id: str | None = None,
    ) -> User:
        user = User(
            username=username,
            email=email,
            password_hash=password_hash,
            role=role,
            oauth_provider=oauth_provider,
            oauth_id=oauth_id,
        )
        self.db.add(user)
        self.db.commit()
        self.db.refresh(user)
        return user

    def touch_last_login(self, user: User) -> None:
        user.last_login = datetime.now(timezone.utc)
        self.db.commit()

    def list_all(self, limit: int = 100, offset: int = 0) -> list[User]:
        return list(
            self.db.execute(
                select(User).order_by(User.id).limit(limit).offset(offset)
            ).scalars()
        )
