"""The app lifespan runs migrations + seed; check the expected rows exist."""
from sqlalchemy import text


def test_schema_and_seed(client):
    # `client` has already triggered lifespan (migrations + seed).
    from app.infrastructure.db.session import SessionLocal

    db = SessionLocal()
    try:
        tables = {
            row[0]
            for row in db.execute(
                text(
                    "SELECT tablename FROM pg_tables WHERE schemaname = 'public'"
                )
            )
        }
        for expected in ("users", "apps", "categories", "settings", "favorites", "logs"):
            assert expected in tables, f"missing table {expected}"

        admin = db.execute(
            text("SELECT role FROM users WHERE username = 'admin'")
        ).first()
        assert admin is not None and admin[0] == "admin"

        (app_count,) = db.execute(text("SELECT count(*) FROM apps")).first()
        assert app_count >= 1

        (cat_count,) = db.execute(text("SELECT count(*) FROM categories")).first()
        assert cat_count >= 1
    finally:
        db.close()


def test_seed_is_idempotent(client):
    """Re-running the seed must not duplicate the admin user."""
    from app.infrastructure.db.session import SessionLocal
    from app.infrastructure.db import seed

    db = SessionLocal()
    try:
        seed.run(db)
        seed.run(db)
        (n,) = db.execute(
            text("SELECT count(*) FROM users WHERE username = 'admin'")
        ).first()
        assert n == 1
    finally:
        db.close()
