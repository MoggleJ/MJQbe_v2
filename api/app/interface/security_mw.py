"""Security hardening middleware: response headers + auth rate limiting."""
import time
from collections import defaultdict, deque

from fastapi import Request
from fastapi.responses import JSONResponse
from starlette.middleware.base import BaseHTTPMiddleware

_SECURITY_HEADERS = {
    "X-Content-Type-Options": "nosniff",
    "X-Frame-Options": "DENY",
    "Referrer-Policy": "no-referrer",
    "Cross-Origin-Opener-Policy": "same-origin",
    # The API only ever returns JSON — lock scripting down hard.
    "Content-Security-Policy": "default-src 'none'; frame-ancestors 'none'",
    "Permissions-Policy": "geolocation=(), microphone=(), camera=()",
}


class SecurityHeadersMiddleware(BaseHTTPMiddleware):
    def __init__(self, app, hsts: bool = False):
        super().__init__(app)
        self.hsts = hsts

    async def dispatch(self, request: Request, call_next):
        response = await call_next(request)
        for k, v in _SECURITY_HEADERS.items():
            response.headers.setdefault(k, v)
        if self.hsts:
            response.headers.setdefault(
                "Strict-Transport-Security", "max-age=31536000; includeSubDomains"
            )
        return response


class RateLimitMiddleware(BaseHTTPMiddleware):
    """Fixed-window limiter for the auth endpoints (in-process, per client IP)."""

    def __init__(self, app, limit: int = 20, window: float = 60.0,
                 protected: tuple[str, ...] = ("/auth/login", "/auth/register", "/auth/refresh")):
        super().__init__(app)
        self.limit = limit
        self.window = window
        self.protected = protected
        self._hits: dict[str, deque[float]] = defaultdict(deque)

    async def dispatch(self, request: Request, call_next):
        path = request.url.path
        if any(path == p or path.startswith(p + "/") for p in self.protected):
            key = f"{request.client.host if request.client else 'unknown'}|{path}"
            now = time.monotonic()
            hits = self._hits[key]
            while hits and now - hits[0] > self.window:
                hits.popleft()
            if len(hits) >= self.limit:
                retry = int(self.window - (now - hits[0])) + 1
                return JSONResponse(
                    {"detail": "too many requests"},
                    status_code=429,
                    headers={"Retry-After": str(retry)},
                )
            hits.append(now)
        return await call_next(request)
