"""Shared test fixtures.

`client` boots the FastAPI app through its lifespan (which runs the Alembic
migrations and the idempotent seed), so the tests below double as an
integration check against a real PostgreSQL.
"""
import pytest
from fastapi.testclient import TestClient


@pytest.fixture(scope="session")
def client():
    from app.main import app

    with TestClient(app) as c:
        yield c
