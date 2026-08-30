use serde::Deserialize;
use serde_json::{json, Value};

use crate::application::{
    AuthService, CatalogService, DevService, FavoritesService, HardwareService, SettingsService,
};
use crate::domain::{CoreError, SettingsPatch};
use crate::infrastructure::hardware::Platform;

use super::protocol::{Request, Response};

/// Routes one request to the matching use case and shapes the reply.
pub struct Handler {
    catalog: CatalogService,
    auth: AuthService,
    favorites: FavoritesService,
    settings: SettingsService,
    dev: DevService,
    hardware: HardwareService,
    platform: Platform,
}

/// Everything the handler needs, grouped so `new` stays readable.
pub struct Services {
    pub catalog: CatalogService,
    pub auth: AuthService,
    pub favorites: FavoritesService,
    pub settings: SettingsService,
    pub dev: DevService,
    pub hardware: HardwareService,
}

#[derive(Deserialize)]
struct ModeParams {
    mode: String,
    #[serde(default)]
    category_id: Option<i32>,
}

#[derive(Deserialize)]
struct RecentParams {
    user_id: i32,
    mode: String,
    #[serde(default = "default_recent_limit")]
    limit: i64,
}
fn default_recent_limit() -> i64 {
    12
}

#[derive(Deserialize)]
struct LoginParams {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct VerifyParams {
    #[serde(default)]
    username: Option<String>,
    password: String,
}

#[derive(Deserialize)]
struct UserParams {
    user_id: i32,
}

#[derive(Deserialize)]
struct FavoriteToggleParams {
    user_id: i32,
    app_id: i32,
}

#[derive(Deserialize)]
struct SettingsUpdateParams {
    user_id: i32,
    #[serde(flatten)]
    patch: SettingsPatch,
}

#[derive(Deserialize)]
struct ProcessListParams {
    #[serde(default = "default_proc_limit")]
    limit: usize,
}
fn default_proc_limit() -> usize {
    60
}

#[derive(Deserialize)]
struct KillParams {
    token: String,
    pid: i32,
    #[serde(default)]
    signal: Option<i32>,
}

#[derive(Deserialize)]
struct NiceParams {
    token: String,
    pid: i32,
    niceness: i32,
}

#[derive(Deserialize)]
struct ContainerActionParams {
    token: String,
    id: String,
}

#[derive(Deserialize)]
struct GpioGetParams {
    pin: u8,
}

#[derive(Deserialize)]
struct GpioSetParams {
    token: String,
    pin: u8,
    value: bool,
}

#[derive(Deserialize)]
struct RelaySetParams {
    token: String,
    relay: u8,
    state: bool,
}

#[derive(Deserialize)]
struct LedSetParams {
    token: String,
    #[serde(default)]
    r: u8,
    #[serde(default)]
    g: u8,
    #[serde(default)]
    b: u8,
}

impl Handler {
    pub fn new(services: Services, platform: Platform) -> Self {
        Self {
            catalog: services.catalog,
            auth: services.auth,
            favorites: services.favorites,
            settings: services.settings,
            dev: services.dev,
            hardware: services.hardware,
            platform,
        }
    }

    pub async fn handle(&self, req: Request) -> Response {
        let id = req.id.clone();
        match self.dispatch(&req).await {
            Ok(data) => Response::ok(id, data),
            Err(e) => {
                let (code, message) = classify(&e);
                Response::error(id, code, &message)
            }
        }
    }

    async fn dispatch(&self, req: &Request) -> Result<Value, CoreError> {
        match req.method.as_str() {
            "ping" => Ok(json!({ "pong": true })),

            "health" => Ok(json!({
                "platform": self.platform.to_string(),
                "version": env!("CARGO_PKG_VERSION"),
            })),

            "session.current" => {
                let out = self.auth.current_user().await?;
                Ok(json!({
                    "user_id": out.user_id, "username": out.username, "role": out.role,
                }))
            }

            "auth.login" => {
                let p: LoginParams = params(req)?;
                let out = self.auth.login(&p.username, &p.password).await?;
                Ok(json!({
                    "user_id": out.user_id, "username": out.username, "role": out.role,
                }))
            }

            "auth.verify" => {
                let p: VerifyParams = params(req)?;
                let (token, expires_in) =
                    self.auth.verify(p.username.as_deref(), &p.password).await?;
                Ok(json!({ "token": token, "expires_in": expires_in }))
            }

            "apps.list" => {
                let p: ModeParams = params(req)?;
                Ok(to_value(
                    self.catalog.list_apps(&p.mode, p.category_id).await?,
                ))
            }

            "apps.recent" => {
                let p: RecentParams = params(req)?;
                Ok(to_value(
                    self.catalog
                        .recent_apps(p.user_id, &p.mode, p.limit)
                        .await?,
                ))
            }

            "categories.list" => {
                let p: ModeParams = params(req)?;
                Ok(to_value(self.catalog.list_categories(&p.mode).await?))
            }

            "favorites.list" => {
                let p: UserParams = params(req)?;
                Ok(to_value(self.favorites.list(p.user_id).await?))
            }

            "favorites.toggle" => {
                let p: FavoriteToggleParams = params(req)?;
                let favorited = self.favorites.toggle(p.user_id, p.app_id).await?;
                Ok(json!({ "app_id": p.app_id, "favorited": favorited }))
            }

            "settings.get" => {
                let p: UserParams = params(req)?;
                Ok(to_value(self.settings.get(p.user_id).await?))
            }

            "settings.update" => {
                let p: SettingsUpdateParams = params(req)?;
                Ok(to_value(self.settings.update(p.user_id, &p.patch).await?))
            }

            // --- Dev mode (monitoring is open; mutations need a re-auth token) ---
            "system.snapshot" => Ok(to_value(self.dev.snapshot().await?)),

            "process.list" => {
                let p: ProcessListParams = params(req)?;
                Ok(to_value(self.dev.list_processes(p.limit)?))
            }

            "process.kill" => {
                let p: KillParams = params(req)?;
                self.auth.check_token(&p.token)?;
                self.dev.kill_process(p.pid, p.signal)?;
                Ok(json!({ "ok": true }))
            }

            "process.nice" => {
                let p: NiceParams = params(req)?;
                self.auth.check_token(&p.token)?;
                self.dev.renice_process(p.pid, p.niceness)?;
                Ok(json!({ "ok": true }))
            }

            "docker.list" => Ok(to_value(self.dev.list_containers().await?)),

            "docker.start" => {
                let p: ContainerActionParams = params(req)?;
                self.auth.check_token(&p.token)?;
                self.dev.start_container(&p.id).await?;
                Ok(json!({ "ok": true }))
            }

            "docker.stop" => {
                let p: ContainerActionParams = params(req)?;
                self.auth.check_token(&p.token)?;
                self.dev.stop_container(&p.id).await?;
                Ok(json!({ "ok": true }))
            }

            // --- Hardware (C daemon) : get/info ouverts, set token-gated ---
            "hardware.info" => Ok(to_value(self.hardware.info().await?)),

            "gpio.get" => {
                let p: GpioGetParams = params(req)?;
                Ok(json!({ "pin": p.pin, "value": self.hardware.gpio_get(p.pin).await? }))
            }

            "gpio.set" => {
                let p: GpioSetParams = params(req)?;
                self.auth.check_token(&p.token)?;
                Ok(to_value(self.hardware.gpio_set(p.pin, p.value).await?))
            }

            "relay.set" => {
                let p: RelaySetParams = params(req)?;
                self.auth.check_token(&p.token)?;
                Ok(to_value(self.hardware.relay_set(p.relay, p.state).await?))
            }

            "led.set" => {
                let p: LedSetParams = params(req)?;
                self.auth.check_token(&p.token)?;
                Ok(to_value(self.hardware.led_set(p.r, p.g, p.b).await?))
            }

            other => Err(CoreError::Internal(format!("unknown method: {other}"))),
        }
    }
}

fn params<T: serde::de::DeserializeOwned>(req: &Request) -> Result<T, CoreError> {
    serde_json::from_value(req.params.clone())
        .map_err(|e| CoreError::Internal(format!("bad params for {}: {e}", req.method)))
}

fn to_value<T: serde::Serialize>(v: T) -> Value {
    serde_json::to_value(v).expect("serialize response payload")
}

fn classify(e: &CoreError) -> (&'static str, String) {
    let code = match e {
        CoreError::DbUnavailable => "db_unavailable",
        CoreError::NotFound => "not_found",
        CoreError::InvalidCredentials => "invalid_credentials",
        CoreError::HardwareUnavailable => "hardware_unavailable",
        CoreError::ReauthRequired => "reauth_required",
        CoreError::PermissionDenied(_) => "permission_denied",
        CoreError::Db(_) => "db_error",
        CoreError::Internal(_) => "internal",
    };
    (code, e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::hardware::DaemonClient;

    fn handler() -> Handler {
        Handler::new(
            Services {
                catalog: CatalogService::new(None),
                auth: AuthService::new(None),
                favorites: FavoritesService::new(None),
                settings: SettingsService::new(None),
                dev: DevService::new(Platform::Stub),
                hardware: HardwareService::new(DaemonClient::new("/nonexistent.sock")),
            },
            Platform::Stub,
        )
    }

    fn req(method: &str, params: Value) -> Request {
        serde_json::from_value(json!({ "id": "1", "method": method, "params": params })).unwrap()
    }

    async fn call(method: &str, params: Value) -> Value {
        serde_json::to_value(handler().handle(req(method, params)).await).unwrap()
    }

    #[tokio::test]
    async fn ping_pongs() {
        let v = call("ping", Value::Null).await;
        assert_eq!(v["data"]["pong"], true);
    }

    #[tokio::test]
    async fn unknown_method_is_internal_error() {
        assert_eq!(call("nope", Value::Null).await["error"]["code"], "internal");
    }

    #[tokio::test]
    async fn db_backed_methods_report_unavailable_without_db() {
        for (m, p) in [
            ("apps.list", json!({ "mode": "tv" })),
            ("favorites.list", json!({ "user_id": 1 })),
            ("settings.get", json!({ "user_id": 1 })),
        ] {
            assert_eq!(call(m, p).await["error"]["code"], "db_unavailable", "{m}");
        }
    }

    #[tokio::test]
    async fn system_snapshot_is_open() {
        let v = call("system.snapshot", Value::Null).await;
        assert_eq!(v["ok"], true);
        assert!(v["data"]["mem_total_kb"].as_u64().unwrap() > 0);
    }

    #[tokio::test]
    async fn process_list_is_open() {
        let v = call("process.list", json!({ "limit": 5 })).await;
        assert_eq!(v["ok"], true);
        assert!(v["data"].as_array().unwrap().len() <= 5);
    }

    #[tokio::test]
    async fn destructive_dev_calls_require_a_valid_token() {
        for (m, p) in [
            ("process.kill", json!({ "token": "bogus", "pid": 999999 })),
            (
                "process.nice",
                json!({ "token": "bogus", "pid": 999999, "niceness": 5 }),
            ),
            ("docker.start", json!({ "token": "bogus", "id": "x" })),
            ("docker.stop", json!({ "token": "bogus", "id": "x" })),
            (
                "gpio.set",
                json!({ "token": "bogus", "pin": 23, "value": true }),
            ),
            (
                "relay.set",
                json!({ "token": "bogus", "relay": 1, "state": true }),
            ),
            ("led.set", json!({ "token": "bogus", "r": 1 })),
        ] {
            assert_eq!(call(m, p).await["error"]["code"], "reauth_required", "{m}");
        }
    }

    #[tokio::test]
    async fn gpio_get_without_daemon_is_hardware_unavailable() {
        assert_eq!(
            call("gpio.get", json!({ "pin": 23 })).await["error"]["code"],
            "hardware_unavailable"
        );
    }
}
