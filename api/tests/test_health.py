def test_health(client):
    r = client.get("/health")
    assert r.status_code == 200
    assert r.json() == {"status": "ok"}


def test_config_port(client):
    r = client.get("/config/port")
    assert r.status_code == 200
    body = r.json()
    assert "web_port" in body and "api_port" in body
    assert isinstance(body["web_port"], int)
    assert isinstance(body["api_port"], int)
