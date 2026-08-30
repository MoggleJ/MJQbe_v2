"""Auth endpoints: local login/register/refresh + Google/GitHub OAuth."""
import secrets

from fastapi import APIRouter, Depends, HTTPException, Request, status
from fastapi.responses import RedirectResponse
from pydantic import BaseModel, EmailStr, Field

from app.application.auth_service import AuthError, AuthService
from app.domain.entities import User
from app.infrastructure.db.user_repo import UserRepository
from app.infrastructure.oauth.providers import get_provider
from app.interface.deps import get_current_user, get_users

router = APIRouter(prefix="/auth", tags=["auth"])

# In-memory OAuth state store (single-process API). Value = provider name.
_oauth_state: dict[str, str] = {}


class RegisterRequest(BaseModel):
    username: str = Field(min_length=3, max_length=64)
    password: str = Field(min_length=8, max_length=256)
    email: EmailStr | None = None


class LoginRequest(BaseModel):
    username: str
    password: str


class RefreshRequest(BaseModel):
    refresh_token: str


class TokenResponse(BaseModel):
    access_token: str
    refresh_token: str
    token_type: str = "bearer"


class UserResponse(BaseModel):
    id: int
    username: str
    email: str | None
    role: str


def _service(users: UserRepository = Depends(get_users)) -> AuthService:
    return AuthService(users)


def _handle(fn):
    try:
        return fn()
    except AuthError as exc:
        raise HTTPException(exc.status, str(exc))


@router.post("/register", response_model=UserResponse, status_code=status.HTTP_201_CREATED)
def register(body: RegisterRequest, svc: AuthService = Depends(_service)):
    user = _handle(lambda: svc.register(body.username, body.password, body.email))
    return UserResponse(id=user.id, username=user.username, email=user.email, role=user.role)


@router.post("/login", response_model=TokenResponse)
def login(body: LoginRequest, svc: AuthService = Depends(_service)):
    _, tokens = _handle(lambda: svc.login(body.username, body.password))
    return TokenResponse(**tokens.__dict__)


@router.post("/refresh", response_model=TokenResponse)
def refresh(body: RefreshRequest, svc: AuthService = Depends(_service)):
    tokens = _handle(lambda: svc.refresh(body.refresh_token))
    return TokenResponse(**tokens.__dict__)


@router.get("/me", response_model=UserResponse)
def me(user: User = Depends(get_current_user)):
    return UserResponse(id=user.id, username=user.username, email=user.email, role=user.role)


# --- OAuth ---------------------------------------------------------------------

def _redirect_uri(request: Request, provider: str) -> str:
    return str(request.url_for("oauth_callback", provider=provider))


@router.get("/oauth/{provider}")
def oauth_start(provider: str, request: Request):
    prov = get_provider(provider)
    if prov is None or not prov.enabled:
        raise HTTPException(status.HTTP_404_NOT_FOUND, f"provider '{provider}' not configured")
    state = secrets.token_urlsafe(24)
    _oauth_state[state] = provider
    return RedirectResponse(prov.authorization_url(_redirect_uri(request, provider), state))


@router.get("/oauth/{provider}/callback", name="oauth_callback", response_model=TokenResponse)
async def oauth_callback(
    provider: str,
    request: Request,
    code: str | None = None,
    state: str | None = None,
    svc: AuthService = Depends(_service),
):
    prov = get_provider(provider)
    if prov is None or not prov.enabled:
        raise HTTPException(status.HTTP_404_NOT_FOUND, "provider not configured")
    if not code or not state or _oauth_state.pop(state, None) != provider:
        raise HTTPException(status.HTTP_400_BAD_REQUEST, "invalid oauth state or code")
    try:
        info = await prov.exchange(code, _redirect_uri(request, provider))
    except Exception as exc:  # noqa: BLE001
        raise HTTPException(status.HTTP_502_BAD_GATEWAY, f"oauth exchange failed: {exc}")
    _, tokens = svc.oauth_upsert(info)
    return TokenResponse(**tokens.__dict__)
