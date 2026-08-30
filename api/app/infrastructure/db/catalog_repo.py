"""Apps + categories persistence."""
from sqlalchemy import select
from sqlalchemy.orm import Session

from app.domain.entities import App, Category

MODES = ("tv", "desktop", "dev")


class AppRepository:
    def __init__(self, db: Session):
        self.db = db

    def get(self, app_id: int) -> App | None:
        return self.db.get(App, app_id)

    def list(
        self,
        *,
        mode: str | None = None,
        category_id: int | None = None,
        include_inactive: bool = False,
    ) -> list[App]:
        stmt = select(App)
        if mode:
            stmt = stmt.where(App.mode == mode)
        if category_id is not None:
            stmt = stmt.where(App.category_id == category_id)
        if not include_inactive:
            stmt = stmt.where(App.is_active.is_(True))
        stmt = stmt.order_by(App.mode, App.name)
        return list(self.db.execute(stmt).scalars())

    def create(self, data: dict) -> App:
        app = App(**data)
        self.db.add(app)
        self.db.commit()
        self.db.refresh(app)
        return app

    def update(self, app: App, data: dict) -> App:
        for key, value in data.items():
            setattr(app, key, value)
        self.db.commit()
        self.db.refresh(app)
        return app

    def delete(self, app: App) -> None:
        self.db.delete(app)
        self.db.commit()


class CategoryRepository:
    def __init__(self, db: Session):
        self.db = db

    def get(self, category_id: int) -> Category | None:
        return self.db.get(Category, category_id)

    def list(self, mode: str | None = None) -> list[Category]:
        stmt = select(Category)
        if mode:
            stmt = stmt.where(Category.mode == mode)
        return list(self.db.execute(stmt.order_by(Category.mode, Category.name)).scalars())

    def find(self, name: str, mode: str) -> Category | None:
        return self.db.execute(
            select(Category).where(Category.name == name, Category.mode == mode)
        ).scalar_one_or_none()

    def create(self, name: str, mode: str) -> Category:
        cat = Category(name=name, mode=mode)
        self.db.add(cat)
        self.db.commit()
        self.db.refresh(cat)
        return cat

    def update(self, cat: Category, data: dict) -> Category:
        for key, value in data.items():
            setattr(cat, key, value)
        self.db.commit()
        self.db.refresh(cat)
        return cat

    def delete(self, cat: Category) -> None:
        self.db.delete(cat)
        self.db.commit()
