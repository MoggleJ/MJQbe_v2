"""Sprint 17 — security headers + auth rate limiting."""
from fastapi import FastAPI
from fastapi.testclient import TestClient

from app.interface.security_mw import RateLimitMiddleware, SecurityHeadersMiddleware


def test_security_headers_present(client):
    r = client.get("/health")
    assert r.headers["x-content-type-options"] == "nosniff"
    assert r.headers["x-frame-options"] == "DENY"
    assert "content-security-policy" in r.headers
    assert "referrer-policy" in r.headers


def test_rate_limit_middleware_returns_429_past_the_window():
    """Isolated app so the shared test client's auth budget is untouched."""
    app = FastAPI()
    app.add_middleware(RateLimitMiddleware, limit=3, window=60.0)
    app.add_middleware(SecurityHeadersMiddleware)

    @app.post("/auth/login")
    def login():
        return {"ok": True}

    @app.get("/health")
    def health():
        return {"ok": True}

    c = TestClient(app)
    codes = [c.post("/auth/login").status_code for _ in range(6)]
    assert codes[:3] == [200, 200, 200]
    assert codes[3:] == [429, 429, 429]
    assert "retry-after" in c.post("/auth/login").headers
    # unrelated route unaffected
    assert c.get("/health").status_code == 200


def test_sql_injection_attempt_is_harmless(client):
    # ORM + bound params: the payload is a literal username, never SQL.
    r = client.post(
        "/auth/login",
        json={"username": "admin'; DROP TABLE users; --", "password": "whatever"},
    )
    assert r.status_code in (401, 429)
    assert client.get("/apps", params={"mode": "tv"}).status_code == 200  # table intact
