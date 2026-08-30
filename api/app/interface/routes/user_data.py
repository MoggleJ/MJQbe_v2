"""Per-user endpoints: settings + favourites (auth required)."""
from typing import Literal

from fastapi import APIRouter, Depends, HTTPException, status
from pydantic import BaseModel, Field
from sqlalchemy.orm import Session

from app.domain.entities import User
from app.infrastructure.db.user_data_repo import (
    SETTINGS_ENUMS,
    FavoritesRepository,
    SettingsRepository,
)
from app.interface.deps import get_current_user, get_db

router = APIRouter(tags=["user"])


class SettingsOut(BaseModel):
    user_id: int
    theme: str
    layout: str
    icon_size: str
    default_mode: str

    model_config = {"from_attributes": True}


class SettingsPatch(BaseModel):
    theme: str | None = None
    layout: Literal["grid", "list"] | None = None
    icon_size: Literal["small", "medium", "large"] | None = None
    default_mode: Literal["tv", "desktop"] | None = None


class FavoritesOut(BaseModel):
    app_ids: list[int] = Field(default_factory=list)


@router.get("/settings", response_model=SettingsOut)
def get_settings(user: User = Depends(get_current_user), db: Session = Depends(get_db)):
    return SettingsRepository(db).get_or_create(user.id)


@router.put("/settings", response_model=SettingsOut)
def put_settings(
    body: SettingsPatch,
    user: User = Depends(get_current_user),
    db: Session = Depends(get_db),
):
    patch = body.model_dump(exclude_unset=True)
    for key, value in patch.items():
        if value not in SETTINGS_ENUMS[key]:
            raise HTTPException(422, f"invalid {key}: {value}")
    return SettingsRepository(db).update(user.id, patch)


@router.get("/favorites", response_model=FavoritesOut)
def get_favorites(user: User = Depends(get_current_user), db: Session = Depends(get_db)):
    return FavoritesOut(app_ids=FavoritesRepository(db).list_app_ids(user.id))


@router.post("/favorites/{app_id}", response_model=FavoritesOut, status_code=status.HTTP_201_CREATED)
def add_favorite(
    app_id: int, user: User = Depends(get_current_user), db: Session = Depends(get_db)
):
    repo = FavoritesRepository(db)
    if not repo.add(user.id, app_id):
        raise HTTPException(status.HTTP_404_NOT_FOUND, "app not found")
    return FavoritesOut(app_ids=repo.list_app_ids(user.id))


@router.delete("/favorites/{app_id}", response_model=FavoritesOut)
def remove_favorite(
    app_id: int, user: User = Depends(get_current_user), db: Session = Depends(get_db)
):
    repo = FavoritesRepository(db)
    repo.remove(user.id, app_id)
    return FavoritesOut(app_ids=repo.list_app_ids(user.id))
