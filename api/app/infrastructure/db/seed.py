import os

import bcrypt
from sqlalchemy.orm import Session
from app.domain.entities import User, Category, App, Settings


def _bootstrap_admin_password() -> str:
    """Initial admin password — set MJQBE_ADMIN_PASSWORD in production. The
    fallback is a placeholder to be changed on first login (docs/deploiement.md)."""
    return os.getenv("MJQBE_ADMIN_PASSWORD") or "admin"


# ---------------------------------------------------------------------------
# Seed data
# ---------------------------------------------------------------------------

_CATEGORIES = [
    # (name, mode)
    ("Streaming",      "tv"),
    ("Navigateur",     "tv"),
    ("Streaming",      "desktop"),
    ("Productivité",   "desktop"),
    ("Développement",  "desktop"),
    ("Système",        "dev"),
    ("GPIO",           "dev"),
    ("Serveurs",       "dev"),
]

_APPS = [
    # (name, icon, url, category_name, mode, is_web)
    ("Netflix",       "netflix",       "https://netflix.com",          "Streaming",     "tv",      True),
    ("YouTube",       "youtube",       "https://youtube.com",          "Streaming",     "tv",      True),
    ("Twitch",        "twitch",        "https://twitch.tv",            "Streaming",     "tv",      True),
    ("Crunchyroll",   "crunchyroll",   "https://crunchyroll.com",      "Streaming",     "tv",      True),
    ("Disney+",       "disney",        "https://disneyplus.com",       "Streaming",     "tv",      True),
    ("Prime Video",   "prime",         "https://primevideo.com",       "Streaming",     "tv",      True),
    ("Navigateur",    "browser",       None,                           "Navigateur",    "tv",      False),
    ("Netflix",       "netflix",       "https://netflix.com",          "Streaming",     "desktop", True),
    ("YouTube",       "youtube",       "https://youtube.com",          "Streaming",     "desktop", True),
    ("Twitch",        "twitch",        "https://twitch.tv",            "Streaming",     "desktop", True),
    ("GitHub",        "github",        "https://github.com",           "Développement", "desktop", True),
    ("Gmail",         "gmail",         "https://mail.google.com",      "Productivité",  "desktop", True),
    ("Google Drive",  "drive",         "https://drive.google.com",     "Productivité",  "desktop", True),
    ("Moniteur",      "monitor",       None,                           "Système",       "dev",     False),
    ("Docker",        "docker",        None,                           "Système",       "dev",     False),
    ("LED Rouge",     "led",           None,                           "GPIO",          "dev",     False),
    ("LED Verte",     "led",           None,                           "GPIO",          "dev",     False),
    ("Relais 1",      "relay",         None,                           "GPIO",          "dev",     False),
    ("Minecraft",     "minecraft",     None,                           "Serveurs",      "dev",     False),
]


def run(db: Session) -> None:
    if db.query(User).filter_by(username="admin").first():
        return  # déjà seedé

    # Catégories
    cat_map: dict[tuple, Category] = {}
    for name, mode in _CATEGORIES:
        cat = Category(name=name, mode=mode)
        db.add(cat)
        db.flush()
        cat_map[(name, mode)] = cat

    # Apps
    for name, icon, url, cat_name, mode, is_web in _APPS:
        cat = cat_map.get((cat_name, mode))
        db.add(App(
            name=name,
            icon=icon,
            url=url,
            category_id=cat.id if cat else None,
            mode=mode,
            is_web=is_web,
        ))

    # Utilisateur admin
    pw_hash = bcrypt.hashpw(_bootstrap_admin_password().encode(), bcrypt.gensalt()).decode()
    admin = User(
        username="admin",
        email="admin@mjqbe.local",
        password_hash=pw_hash,
        role="admin",
    )
    db.add(admin)
    db.flush()
    db.add(Settings(user_id=admin.id))

    db.commit()
