"""Shared test fixtures.

`client` boots the FastAPI app through its lifespan (which runs the Alembic
migrations and the idempotent seed), so the tests below double as an
integration check against a real PostgreSQL.
"""
import os

import pytest
from fastapi.testclient import TestClient

# The auth rate limiter would trip during the suite's many logins — lift it here.
os.environ.setdefault("AUTH_RATE_LIMIT_PER_MIN", "100000")


@pytest.fixture(scope="session")
def client():
    from app.main import app

    with TestClient(app) as c:
        yield c
