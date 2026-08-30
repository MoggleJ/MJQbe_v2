"""Sprint 12 — admin panel: logs / users / config / services."""
import uuid

import pytest
import yaml


@pytest.fixture
def tmp_config(tmp_path, monkeypatch):
    """Redirect CONFIG_PATH to a disposable copy so tests never touch the real file."""
    from app.infrastructure.config_file import read_config

    src = read_config()  # current real config (via the default CONFIG_PATH)
    dest = tmp_path / "config.yml"
    dest.write_text(yaml.safe_dump(src, sort_keys=False))
    monkeypatch.setenv("CONFIG_PATH", str(dest))
    return dest


def _admin(client):
    tok = client.post(
        "/auth/login", json={"username": "admin", "password": "admin"}
    ).json()["access_token"]
    return {"Authorization": f"Bearer {tok}"}


def _user(client):
    u = f"adm_{uuid.uuid4().hex[:8]}"
    client.post("/auth/register", json={"username": u, "password": "password123"})
    tok = client.post(
        "/auth/login", json={"username": u, "password": "password123"}
    ).json()["access_token"]
    return {"Authorization": f"Bearer {tok}"}


def test_admin_routes_reject_non_admin(client):
    for path in ("/admin/logs", "/admin/users", "/admin/config", "/admin/services"):
        assert client.get(path).status_code == 401, path
        assert client.get(path, headers=_user(client)).status_code == 403, path


def test_admin_logs_pagination(client):
    h = _admin(client)
    r = client.get("/admin/logs", headers=h, params={"limit": 5, "offset": 0})
    assert r.status_code == 200
    body = r.json()
    assert set(body) == {"total", "limit", "offset", "items"}
    assert body["limit"] == 5
    assert len(body["items"]) <= 5


def test_admin_users_lists_admin(client):
    users = client.get("/admin/users", headers=_admin(client)).json()
    assert any(u["username"] == "admin" and u["role"] == "admin" for u in users)


def test_admin_config_read_and_roundtrip(client, tmp_config):
    h = _admin(client)
    cfg = client.get("/admin/config", headers=h)
    assert cfg.status_code == 200
    data = cfg.json()
    assert "server" in data and "web_port" in data["server"]

    # write it back unchanged (proves the write path + validation)
    ok = client.put(
        "/admin/config", headers=h, json={"config": data, "password": "admin"}
    )
    assert ok.status_code == 200, ok.text
    assert ok.json()["server"]["web_port"] == data["server"]["web_port"]


def test_admin_config_rejects_bad_structure_and_password(client, tmp_config):
    h = _admin(client)
    assert client.put(
        "/admin/config", headers=h, json={"config": {"nope": 1}, "password": "admin"}
    ).status_code == 422
    assert client.put(
        "/admin/config", headers=h,
        json={"config": {"server": {"web_port": 1, "api_port": 2}}, "password": "wrong"},
    ).status_code == 401


def test_admin_services_returns_list_or_503(client):
    r = client.get("/admin/services", headers=_admin(client))
    assert r.status_code in (200, 503)
    if r.status_code == 200:
        assert isinstance(r.json(), list)


def test_admin_reboot_requires_reauth(client):
    h = _admin(client)
    assert client.post(
        "/admin/reboot", headers=h, json={"password": "wrong"}
    ).status_code == 401
