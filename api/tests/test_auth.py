"""Sprint 10 — auth integration tests (local + guards + OAuth wiring)."""
import uuid

import pytest


@pytest.fixture
def new_user():
    suffix = uuid.uuid4().hex[:10]
    return {
        "username": f"user_{suffix}",
        "password": "s3cret-password",
        "email": f"{suffix}@example.com",
    }


def _login(client, username, password):
    r = client.post("/auth/login", json={"username": username, "password": password})
    assert r.status_code == 200, r.text
    return r.json()


# --- registration ------------------------------------------------------------

def test_register_then_login(client, new_user):
    r = client.post("/auth/register", json=new_user)
    assert r.status_code == 201, r.text
    body = r.json()
    assert body["username"] == new_user["username"]
    assert body["role"] == "user"

    tokens = _login(client, new_user["username"], new_user["password"])
    assert tokens["token_type"] == "bearer"
    assert tokens["access_token"] and tokens["refresh_token"]


def test_register_rejects_duplicate(client, new_user):
    assert client.post("/auth/register", json=new_user).status_code == 201
    assert client.post("/auth/register", json=new_user).status_code == 409


def test_register_rejects_short_password(client, new_user):
    new_user["password"] = "short"
    assert client.post("/auth/register", json=new_user).status_code == 422


# --- login / me / refresh --------------------------------------------------

def test_login_wrong_password(client):
    r = client.post("/auth/login", json={"username": "admin", "password": "nope"})
    assert r.status_code == 401


def test_me_requires_token(client):
    assert client.get("/auth/me").status_code == 401

    tokens = _login(client, "admin", "admin")
    r = client.get("/auth/me", headers={"Authorization": f"Bearer {tokens['access_token']}"})
    assert r.status_code == 200
    assert r.json()["role"] == "admin"


def test_refresh_issues_new_access_token(client):
    tokens = _login(client, "admin", "admin")
    r = client.post("/auth/refresh", json={"refresh_token": tokens["refresh_token"]})
    assert r.status_code == 200
    assert r.json()["access_token"]

    bad = client.post("/auth/refresh", json={"refresh_token": tokens["access_token"]})
    assert bad.status_code == 401  # an access token is not a refresh token


# --- route guards (/dev/* is admin-only) ----------------------------------

def test_dev_routes_are_protected(client, new_user):
    assert client.get("/dev/hardware").status_code == 401  # no token

    client.post("/auth/register", json=new_user)
    user_tok = _login(client, new_user["username"], new_user["password"])["access_token"]
    assert client.get(
        "/dev/hardware", headers={"Authorization": f"Bearer {user_tok}"}
    ).status_code == 403  # not admin

    admin_tok = _login(client, "admin", "admin")["access_token"]
    r = client.get("/dev/hardware", headers={"Authorization": f"Bearer {admin_tok}"})
    assert r.status_code in (200, 503)  # authorised; 503 only if the daemon is down


# --- OAuth wiring --------------------------------------------------------------

def test_oauth_unknown_provider_is_404(client):
    assert client.get("/auth/oauth/myspace", follow_redirects=False).status_code == 404


def test_oauth_disabled_provider_is_404(client, monkeypatch):
    monkeypatch.delenv("GOOGLE_CLIENT_ID", raising=False)
    monkeypatch.delenv("GOOGLE_CLIENT_SECRET", raising=False)
    assert client.get("/auth/oauth/google", follow_redirects=False).status_code == 404


def test_oauth_redirects_when_configured(client, monkeypatch):
    monkeypatch.setenv("GITHUB_CLIENT_ID", "cid-test")
    monkeypatch.setenv("GITHUB_CLIENT_SECRET", "csecret-test")
    r = client.get("/auth/oauth/github", follow_redirects=False)
    assert r.status_code in (302, 307)
    assert r.headers["location"].startswith("https://github.com/login/oauth/authorize?")
    assert "client_id=cid-test" in r.headers["location"]


def test_oauth_upsert_creates_and_reuses_user():
    from app.application.auth_service import AuthService
    from app.infrastructure.db.session import SessionLocal
    from app.infrastructure.db.user_repo import UserRepository
    from app.infrastructure.oauth.providers import OAuthUser

    db = SessionLocal()
    try:
        svc = AuthService(UserRepository(db))
        oid = uuid.uuid4().hex
        info = OAuthUser(
            provider="github", oauth_id=oid, email=f"{oid}@oauth.example.com", username="ghuser"
        )
        user1, _ = svc.oauth_upsert(info)
        user2, _ = svc.oauth_upsert(info)
        assert user1.id == user2.id
        assert user1.oauth_provider == "github"
    finally:
        db.close()
