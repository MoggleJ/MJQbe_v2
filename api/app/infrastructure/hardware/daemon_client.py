"""Client for the C hardware daemon (``mjqbe-daemon``).

Same wire protocol as the daemon and the Rust client: one JSON object per line
over a Unix socket. One short-lived connection per request.
"""
import json
import os
import socket

_TIMEOUT = 3.0


class HardwareUnavailable(RuntimeError):
    """The daemon socket could not be reached."""


class DaemonError(RuntimeError):
    """The daemon replied with ``ok: false``."""


def _socket_path() -> str:
    return os.getenv("DAEMON_SOCKET", "/run/mjqbe/daemon.sock")


def _request(cmd: str, **params) -> dict:
    payload = {"cmd": cmd, **params}
    try:
        s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        s.settimeout(_TIMEOUT)
        s.connect(_socket_path())
    except OSError as exc:
        raise HardwareUnavailable(str(exc)) from exc

    try:
        s.sendall((json.dumps(payload) + "\n").encode())
        buf = b""
        while b"\n" not in buf:
            chunk = s.recv(4096)
            if not chunk:
                break
            buf += chunk
    finally:
        s.close()

    try:
        resp = json.loads(buf.decode().splitlines()[0])
    except (ValueError, IndexError) as exc:
        raise DaemonError(f"bad daemon response: {buf!r}") from exc

    if not resp.get("ok"):
        raise DaemonError(resp.get("error", "daemon error"))
    return resp.get("data", {})


def info() -> dict:
    return _request("info")


def gpio_set(pin: int, value: int) -> dict:
    return _request("gpio_set", pin=int(pin), value=1 if value else 0)


def gpio_get(pin: int) -> dict:
    return _request("gpio_get", pin=int(pin))


def relay_set(relay: int, state: int) -> dict:
    return _request("relay_set", relay=int(relay), state=1 if state else 0)


def led_set(r: int, g: int, b: int) -> dict:
    return _request("led_set", r=int(r), g=int(g), b=int(b))


_AV_ACTIONS = {"tv_on", "tv_off", "tv_toggle", "ps4_on", "ps4_off"}


def av_status() -> dict:
    return _request("av_status")


def av_cec(action: str) -> dict:
    if action not in _AV_ACTIONS:
        raise DaemonError(f"invalid AV action: {action}")
    return _request("cec_send", action=action)
