"""Apps + categories CRUD. Reads are public; writes require an admin JWT."""
from typing import Literal

from fastapi import APIRouter, Depends, HTTPException, Query, status
from pydantic import BaseModel, Field
from sqlalchemy.orm import Session

from app.infrastructure.db.catalog_repo import AppRepository, CategoryRepository
from app.interface.deps import get_db, require_admin

Mode = Literal["tv", "desktop", "dev"]

apps_router = APIRouter(prefix="/apps", tags=["apps"])
categories_router = APIRouter(prefix="/categories", tags=["categories"])


# --- schemas ---------------------------------------------------------------

class AppOut(BaseModel):
    id: int
    name: str
    icon: str | None = None
    url: str | None = None
    category_id: int | None = None
    mode: str
    is_web: bool
    is_active: bool

    model_config = {"from_attributes": True}


class AppCreate(BaseModel):
    name: str = Field(min_length=1, max_length=128)
    mode: Mode
    icon: str | None = Field(default=None, max_length=255)
    url: str | None = Field(default=None, max_length=512)
    category_id: int | None = None
    is_web: bool = True
    is_active: bool = True


class AppUpdate(BaseModel):
    name: str | None = Field(default=None, min_length=1, max_length=128)
    mode: Mode | None = None
    icon: str | None = Field(default=None, max_length=255)
    url: str | None = Field(default=None, max_length=512)
    category_id: int | None = None
    is_web: bool | None = None
    is_active: bool | None = None


class CategoryOut(BaseModel):
    id: int
    name: str
    mode: str

    model_config = {"from_attributes": True}


class CategoryCreate(BaseModel):
    name: str = Field(min_length=1, max_length=64)
    mode: Mode


class CategoryUpdate(BaseModel):
    name: str | None = Field(default=None, min_length=1, max_length=64)
    mode: Mode | None = None


# --- apps ----------------------------------------------------------------------

def _apps(db: Session = Depends(get_db)) -> AppRepository:
    return AppRepository(db)


def _cats(db: Session = Depends(get_db)) -> CategoryRepository:
    return CategoryRepository(db)


@apps_router.get("", response_model=list[AppOut])
def list_apps(
    repo: AppRepository = Depends(_apps),
    mode: Mode | None = Query(default=None),
    category_id: int | None = Query(default=None),
    include_inactive: bool = Query(default=False),
):
    return repo.list(mode=mode, category_id=category_id, include_inactive=include_inactive)


@apps_router.get("/{app_id}", response_model=AppOut)
def get_app(app_id: int, repo: AppRepository = Depends(_apps)):
    app = repo.get(app_id)
    if not app:
        raise HTTPException(status.HTTP_404_NOT_FOUND, "app not found")
    return app


@apps_router.post(
    "", response_model=AppOut, status_code=status.HTTP_201_CREATED,
    dependencies=[Depends(require_admin)],
)
def create_app(
    body: AppCreate,
    repo: AppRepository = Depends(_apps),
    cats: CategoryRepository = Depends(_cats),
):
    if body.category_id is not None and not cats.get(body.category_id):
        raise HTTPException(422, "unknown category_id")
    return repo.create(body.model_dump())


@apps_router.put("/{app_id}", response_model=AppOut, dependencies=[Depends(require_admin)])
def update_app(
    app_id: int,
    body: AppUpdate,
    repo: AppRepository = Depends(_apps),
    cats: CategoryRepository = Depends(_cats),
):
    app = repo.get(app_id)
    if not app:
        raise HTTPException(status.HTTP_404_NOT_FOUND, "app not found")
    data = body.model_dump(exclude_unset=True)
    if data.get("category_id") is not None and not cats.get(data["category_id"]):
        raise HTTPException(422, "unknown category_id")
    return repo.update(app, data)


@apps_router.delete(
    "/{app_id}", status_code=status.HTTP_204_NO_CONTENT,
    dependencies=[Depends(require_admin)],
)
def delete_app(app_id: int, repo: AppRepository = Depends(_apps)):
    app = repo.get(app_id)
    if not app:
        raise HTTPException(status.HTTP_404_NOT_FOUND, "app not found")
    repo.delete(app)


# --- categories --------------------------------------------------------------

@categories_router.get("", response_model=list[CategoryOut])
def list_categories(repo: CategoryRepository = Depends(_cats), mode: Mode | None = Query(default=None)):
    return repo.list(mode)


@categories_router.post(
    "", response_model=CategoryOut, status_code=status.HTTP_201_CREATED,
    dependencies=[Depends(require_admin)],
)
def create_category(body: CategoryCreate, repo: CategoryRepository = Depends(_cats)):
    if repo.find(body.name, body.mode):
        raise HTTPException(status.HTTP_409_CONFLICT, "category already exists for this mode")
    return repo.create(body.name, body.mode)


@categories_router.put(
    "/{category_id}", response_model=CategoryOut, dependencies=[Depends(require_admin)]
)
def update_category(category_id: int, body: CategoryUpdate, repo: CategoryRepository = Depends(_cats)):
    cat = repo.get(category_id)
    if not cat:
        raise HTTPException(status.HTTP_404_NOT_FOUND, "category not found")
    return repo.update(cat, body.model_dump(exclude_unset=True))


@categories_router.delete(
    "/{category_id}", status_code=status.HTTP_204_NO_CONTENT,
    dependencies=[Depends(require_admin)],
)
def delete_category(category_id: int, repo: CategoryRepository = Depends(_cats)):
    cat = repo.get(category_id)
    if not cat:
        raise HTTPException(status.HTTP_404_NOT_FOUND, "category not found")
    repo.delete(cat)
