"""Minimal OAuth 2.0 clients for Google and GitHub.

Client id/secret come from the environment (never ``config.yml``). If they are
unset the provider is considered *disabled* and the routes return 404.
"""
import os
from dataclasses import dataclass
from urllib.parse import urlencode

import httpx


@dataclass
class OAuthUser:
    provider: str
    oauth_id: str
    email: str | None
    username: str


class OAuthProvider:
    name: str
    authorize_url: str
    token_url: str
    scope: str

    def __init__(self):
        prefix = self.name.upper()
        self.client_id = os.getenv(f"{prefix}_CLIENT_ID", "")
        self.client_secret = os.getenv(f"{prefix}_CLIENT_SECRET", "")

    @property
    def enabled(self) -> bool:
        return bool(self.client_id and self.client_secret)

    def authorization_url(self, redirect_uri: str, state: str) -> str:
        params = {
            "client_id": self.client_id,
            "redirect_uri": redirect_uri,
            "response_type": "code",
            "scope": self.scope,
            "state": state,
        }
        return f"{self.authorize_url}?{urlencode(params)}"

    async def exchange(self, code: str, redirect_uri: str) -> OAuthUser:
        async with httpx.AsyncClient(timeout=10) as client:
            token_resp = await client.post(
                self.token_url,
                data={
                    "client_id": self.client_id,
                    "client_secret": self.client_secret,
                    "code": code,
                    "redirect_uri": redirect_uri,
                    "grant_type": "authorization_code",
                },
                headers={"Accept": "application/json"},
            )
            token_resp.raise_for_status()
            access_token = token_resp.json()["access_token"]
            return await self._userinfo(client, access_token)

    async def _userinfo(self, client: httpx.AsyncClient, access_token: str) -> OAuthUser:
        raise NotImplementedError


class GoogleProvider(OAuthProvider):
    name = "google"
    authorize_url = "https://accounts.google.com/o/oauth2/v2/auth"
    token_url = "https://oauth2.googleapis.com/token"
    scope = "openid email profile"

    async def _userinfo(self, client, access_token):
        r = await client.get(
            "https://openidconnect.googleapis.com/v1/userinfo",
            headers={"Authorization": f"Bearer {access_token}"},
        )
        r.raise_for_status()
        data = r.json()
        return OAuthUser(
            provider="google",
            oauth_id=str(data["sub"]),
            email=data.get("email"),
            username=data.get("email", f"google-{data['sub']}").split("@")[0],
        )


class GitHubProvider(OAuthProvider):
    name = "github"
    authorize_url = "https://github.com/login/oauth/authorize"
    token_url = "https://github.com/login/oauth/access_token"
    scope = "read:user user:email"

    async def _userinfo(self, client, access_token):
        headers = {
            "Authorization": f"Bearer {access_token}",
            "Accept": "application/vnd.github+json",
        }
        r = await client.get("https://api.github.com/user", headers=headers)
        r.raise_for_status()
        data = r.json()
        email = data.get("email")
        if not email:
            er = await client.get("https://api.github.com/user/emails", headers=headers)
            if er.status_code == 200:
                primary = next(
                    (e for e in er.json() if e.get("primary")), None
                ) or next(iter(er.json()), None)
                email = primary["email"] if primary else None
        return OAuthUser(
            provider="github",
            oauth_id=str(data["id"]),
            email=email,
            username=data.get("login", f"github-{data['id']}"),
        )


_PROVIDERS = {"google": GoogleProvider, "github": GitHubProvider}


def get_provider(name: str) -> OAuthProvider | None:
    cls = _PROVIDERS.get(name)
    return cls() if cls else None
