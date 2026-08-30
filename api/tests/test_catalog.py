"""Sprint 11 — apps & categories CRUD."""
import uuid


def _admin_headers(client):
    r = client.post("/auth/login", json={"username": "admin", "password": "admin"})
    return {"Authorization": f"Bearer {r.json()['access_token']}"}


def _user_headers(client):
    u = f"cat_user_{uuid.uuid4().hex[:8]}"
    client.post("/auth/register", json={"username": u, "password": "password123"})
    r = client.post("/auth/login", json={"username": u, "password": "password123"})
    return {"Authorization": f"Bearer {r.json()['access_token']}"}


# --- reads (public) ---------------------------------------------------------

def test_list_apps_by_mode(client):
    r = client.get("/apps", params={"mode": "tv"})
    assert r.status_code == 200
    apps = r.json()
    assert len(apps) >= 1
    assert all(a["mode"] == "tv" for a in apps)
    assert any(a["name"] == "Netflix" for a in apps)


def test_get_app_detail_and_404(client):
    first = client.get("/apps", params={"mode": "tv"}).json()[0]
    r = client.get(f"/apps/{first['id']}")
    assert r.status_code == 200
    assert r.json()["id"] == first["id"]
    assert client.get("/apps/99999999").status_code == 404


def test_list_categories_by_mode(client):
    r = client.get("/categories", params={"mode": "desktop"})
    assert r.status_code == 200
    assert all(c["mode"] == "desktop" for c in r.json())


# --- writes require admin -------------------------------------------------

def test_create_app_requires_admin(client):
    body = {"name": "Nope", "mode": "tv"}
    assert client.post("/apps", json=body).status_code == 401
    assert client.post("/apps", json=body, headers=_user_headers(client)).status_code == 403


def test_app_crud_as_admin(client):
    h = _admin_headers(client)
    name = f"TestApp {uuid.uuid4().hex[:6]}"

    created = client.post(
        "/apps", headers=h,
        json={"name": name, "mode": "desktop", "url": "https://x.test", "is_web": True},
    )
    assert created.status_code == 201, created.text
    app_id = created.json()["id"]

    upd = client.put(f"/apps/{app_id}", headers=h, json={"is_active": False, "name": name + "!"})
    assert upd.status_code == 200
    assert upd.json()["is_active"] is False
    assert upd.json()["name"] == name + "!"

    # inactive → hidden by default, visible with include_inactive
    listed = client.get("/apps", params={"mode": "desktop"}).json()
    assert app_id not in [a["id"] for a in listed]
    listed_all = client.get(
        "/apps", params={"mode": "desktop", "include_inactive": True}
    ).json()
    assert app_id in [a["id"] for a in listed_all]

    assert client.delete(f"/apps/{app_id}", headers=h).status_code == 204
    assert client.get(f"/apps/{app_id}").status_code == 404


def test_create_app_rejects_unknown_category(client):
    h = _admin_headers(client)
    r = client.post(
        "/apps", headers=h,
        json={"name": "BadCat", "mode": "tv", "category_id": 987654},
    )
    assert r.status_code == 422


def test_category_crud_as_admin(client):
    h = _admin_headers(client)
    name = f"Cat {uuid.uuid4().hex[:6]}"

    created = client.post("/categories", headers=h, json={"name": name, "mode": "desktop"})
    assert created.status_code == 201
    cid = created.json()["id"]

    # duplicate (same name+mode) → 409
    assert client.post(
        "/categories", headers=h, json={"name": name, "mode": "desktop"}
    ).status_code == 409

    ren = client.put(f"/categories/{cid}", headers=h, json={"name": name + " x"})
    assert ren.status_code == 200 and ren.json()["name"] == name + " x"

    assert client.delete(f"/categories/{cid}", headers=h).status_code == 204
    assert client.get("/categories").status_code == 200


def test_pydantic_rejects_bad_mode(client):
    h = _admin_headers(client)
    assert client.post(
        "/apps", headers=h, json={"name": "X", "mode": "holodeck"}
    ).status_code == 422
