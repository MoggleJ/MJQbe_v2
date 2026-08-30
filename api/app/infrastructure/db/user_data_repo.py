"""Per-user data: settings, favourites, logs."""
from sqlalchemy import delete, insert, select
from sqlalchemy.orm import Session

from app.domain.entities import App, Favorite, Log, Settings, User

_THEMES = {
    "dark", "dark-blue", "dark-purple", "amoled", "dark-green",
    "light", "light-warm", "light-blue", "light-purple", "light-green",
}
_LAYOUTS = {"grid", "list"}
_ICON_SIZES = {"small", "medium", "large"}
_DEFAULT_MODES = {"tv", "desktop"}

SETTINGS_ENUMS = {
    "theme": _THEMES,
    "layout": _LAYOUTS,
    "icon_size": _ICON_SIZES,
    "default_mode": _DEFAULT_MODES,
}


class SettingsRepository:
    def __init__(self, db: Session):
        self.db = db

    def get_or_create(self, user_id: int) -> Settings:
        row = self.db.execute(
            select(Settings).where(Settings.user_id == user_id)
        ).scalar_one_or_none()
        if row is None:
            row = Settings(user_id=user_id)
            self.db.add(row)
            self.db.commit()
            self.db.refresh(row)
        return row

    def update(self, user_id: int, patch: dict) -> Settings:
        row = self.get_or_create(user_id)
        for key, value in patch.items():
            setattr(row, key, value)
        self.db.commit()
        self.db.refresh(row)
        return row


class FavoritesRepository:
    def __init__(self, db: Session):
        self.db = db

    def list_app_ids(self, user_id: int) -> list[int]:
        return list(
            self.db.execute(
                select(Favorite.app_id)
                .where(Favorite.user_id == user_id)
                .order_by(Favorite.app_id)
            ).scalars()
        )

    def add(self, user_id: int, app_id: int) -> bool:
        """Returns False if the app does not exist."""
        if not self.db.get(App, app_id):
            return False
        exists = self.db.execute(
            select(Favorite.id).where(
                Favorite.user_id == user_id, Favorite.app_id == app_id
            )
        ).scalar_one_or_none()
        if exists is None:
            self.db.execute(
                insert(Favorite).values(user_id=user_id, app_id=app_id)
            )
            self.db.commit()
        return True

    def remove(self, user_id: int, app_id: int) -> None:
        self.db.execute(
            delete(Favorite).where(
                Favorite.user_id == user_id, Favorite.app_id == app_id
            )
        )
        self.db.commit()


class LogRepository:
    def __init__(self, db: Session):
        self.db = db

    def record(self, action: str, user_id: int | None, meta: dict | None = None) -> None:
        self.db.add(Log(action=action, user_id=user_id, meta=meta or {}))
        self.db.commit()

    def list(self, limit: int = 50, offset: int = 0) -> list[Log]:
        return list(
            self.db.execute(
                select(Log).order_by(Log.created_at.desc()).limit(limit).offset(offset)
            ).scalars()
        )

    def count(self) -> int:
        from sqlalchemy import func

        return self.db.execute(select(func.count(Log.id))).scalar_one()


def list_users(db: Session, limit: int = 100, offset: int = 0) -> list[User]:
    return list(
        db.execute(
            select(User).order_by(User.id).limit(limit).offset(offset)
        ).scalars()
    )
