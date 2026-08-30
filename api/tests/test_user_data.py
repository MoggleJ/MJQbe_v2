"""Sprint 12 — settings + favourites (per-user)."""
import uuid


def _register_and_login(client):
    u = f"ud_{uuid.uuid4().hex[:10]}"
    r = client.post("/auth/register", json={"username": u, "password": "password123"})
    assert r.status_code == 201, r.text
    tok = client.post(
        "/auth/login", json={"username": u, "password": "password123"}
    ).json()["access_token"]
    return {"Authorization": f"Bearer {tok}"}, u


def test_settings_created_on_register_and_updatable(client):
    h, _ = _register_and_login(client)

    r = client.get("/settings", headers=h)
    assert r.status_code == 200
    body = r.json()
    assert body["theme"] == "dark" and body["default_mode"] == "tv"

    upd = client.put("/settings", headers=h, json={"theme": "amoled", "layout": "list"})
    assert upd.status_code == 200
    assert upd.json()["theme"] == "amoled" and upd.json()["layout"] == "list"
    # persisted
    assert client.get("/settings", headers=h).json()["theme"] == "amoled"


def test_settings_rejects_bad_value(client):
    h, _ = _register_and_login(client)
    assert client.put("/settings", headers=h, json={"theme": "neon"}).status_code == 422
    assert client.put("/settings", headers=h, json={"default_mode": "dev"}).status_code == 422


def test_settings_requires_auth(client):
    assert client.get("/settings").status_code == 401


def test_favorites_add_list_remove(client):
    h, _ = _register_and_login(client)
    app_id = client.get("/apps", params={"mode": "tv"}).json()[0]["id"]

    assert client.get("/favorites", headers=h).json()["app_ids"] == []

    added = client.post(f"/favorites/{app_id}", headers=h)
    assert added.status_code == 201
    assert app_id in added.json()["app_ids"]

    # idempotent add
    again = client.post(f"/favorites/{app_id}", headers=h)
    assert again.json()["app_ids"].count(app_id) == 1

    removed = client.delete(f"/favorites/{app_id}", headers=h)
    assert app_id not in removed.json()["app_ids"]


def test_favorite_unknown_app_is_404(client):
    h, _ = _register_and_login(client)
    assert client.post("/favorites/99999999", headers=h).status_code == 404


def test_app_launch_is_logged(client):
    """GET /apps/{id} by an authenticated user records an app_launch log."""
    h, _ = _register_and_login(client)
    app_id = client.get("/apps", params={"mode": "tv"}).json()[0]["id"]

    admin_tok = client.post(
        "/auth/login", json={"username": "admin", "password": "admin"}
    ).json()["access_token"]
    admin_h = {"Authorization": f"Bearer {admin_tok}"}

    before = client.get("/admin/logs", headers=admin_h, params={"limit": 1}).json()["total"]
    client.get(f"/apps/{app_id}", headers=h)
    after = client.get("/admin/logs", headers=admin_h, params={"limit": 5}).json()
    assert after["total"] == before + 1
    assert after["items"][0]["action"] == "app_launch"
    assert after["items"][0]["metadata"]["app_id"] == app_id
