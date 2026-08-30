"""Dev hardware endpoints — thin wrappers over the C daemon.

Sprint 7: POST /dev/gpio, POST /dev/relay (+ GET /dev/hardware for status).
These routes will be placed behind the admin JWT guard in Sprint 10
(protection of /dev/* and /admin/*).
"""
from typing import Literal

from fastapi import APIRouter, HTTPException
from pydantic import BaseModel, Field

from app.infrastructure.hardware import daemon_client

router = APIRouter(prefix="/dev", tags=["dev"])


class GpioRequest(BaseModel):
    pin: int = Field(ge=0, le=53)
    value: int = Field(ge=0, le=1)


class RelayRequest(BaseModel):
    relay: int = Field(ge=1, le=4)
    state: int = Field(ge=0, le=1)


class LedRequest(BaseModel):
    r: int = Field(default=0, ge=0, le=255)
    g: int = Field(default=0, ge=0, le=255)
    b: int = Field(default=0, ge=0, le=255)


class AvRequest(BaseModel):
    action: Literal["tv_on", "tv_off", "tv_toggle", "ps4_on", "ps4_off"]


def _call(fn, *args):
    try:
        return fn(*args)
    except daemon_client.HardwareUnavailable as exc:
        raise HTTPException(status_code=503, detail=f"hardware daemon unavailable: {exc}")
    except daemon_client.DaemonError as exc:
        raise HTTPException(status_code=400, detail=str(exc))


@router.get("/hardware")
def hardware_info():
    return _call(daemon_client.info)


@router.post("/gpio")
def gpio(req: GpioRequest):
    return _call(daemon_client.gpio_set, req.pin, req.value)


@router.get("/gpio/{pin}")
def gpio_read(pin: int):
    return _call(daemon_client.gpio_get, pin)


@router.post("/relay")
def relay(req: RelayRequest):
    return _call(daemon_client.relay_set, req.relay, req.state)


@router.post("/led")
def led(req: LedRequest):
    return _call(daemon_client.led_set, req.r, req.g, req.b)


@router.get("/av")
def av_status():
    return _call(daemon_client.av_status)


@router.post("/av")
def av(req: AvRequest):
    return _call(daemon_client.av_cec, req.action)
