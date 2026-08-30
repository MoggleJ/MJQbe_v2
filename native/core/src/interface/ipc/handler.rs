use serde::Deserialize;
use serde_json::{json, Value};

use crate::application::{AuthService, CatalogService, FavoritesService, SettingsService};
use crate::domain::{CoreError, SettingsPatch};
use crate::infrastructure::hardware::Platform;

use super::protocol::{Request, Response};

/// Routes one request to the matching use case and shapes the reply.
pub struct Handler {
    catalog: CatalogService,
    auth: AuthService,
    favorites: FavoritesService,
    settings: SettingsService,
    platform: Platform,
}

/// Everything the handler needs, grouped so `new` stays readable.
pub struct Services {
    pub catalog: CatalogService,
    pub auth: AuthService,
    pub favorites: FavoritesService,
    pub settings: SettingsService,
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

impl Handler {
    pub fn new(services: Services, platform: Platform) -> Self {
        Self {
            catalog: services.catalog,
            auth: services.auth,
            favorites: services.favorites,
            settings: services.settings,
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

            "apps.list" => {
                let p: ModeParams = params(req)?;
                let apps = self.catalog.list_apps(&p.mode, p.category_id).await?;
                Ok(to_value(apps))
            }

            "apps.recent" => {
                let p: RecentParams = params(req)?;
                let apps = self
                    .catalog
                    .recent_apps(p.user_id, &p.mode, p.limit)
                    .await?;
                Ok(to_value(apps))
            }

            "categories.list" => {
                let p: ModeParams = params(req)?;
                let cats = self.catalog.list_categories(&p.mode).await?;
                Ok(to_value(cats))
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
        CoreError::Db(_) => "db_error",
        CoreError::Internal(_) => "internal",
    };
    (code, e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::{AuthService, CatalogService, FavoritesService, SettingsService};

    fn handler() -> Handler {
        Handler::new(
            Services {
                catalog: CatalogService::new(None),
                auth: AuthService::new(None),
                favorites: FavoritesService::new(None),
                settings: SettingsService::new(None),
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
        assert_eq!(v["ok"], true);
        assert_eq!(v["data"]["pong"], true);
    }

    #[tokio::test]
    async fn health_reports_stub_platform() {
        assert_eq!(
            call("health", Value::Null).await["data"]["platform"],
            "stub"
        );
    }

    #[tokio::test]
    async fn unknown_method_is_internal_error() {
        let v = call("does.not.exist", Value::Null).await;
        assert_eq!(v["ok"], false);
        assert_eq!(v["error"]["code"], "internal");
    }

    #[tokio::test]
    async fn db_backed_methods_report_unavailable_without_db() {
        for (m, p) in [
            ("apps.list", json!({ "mode": "tv" })),
            ("apps.recent", json!({ "user_id": 1, "mode": "tv" })),
            ("favorites.list", json!({ "user_id": 1 })),
            ("favorites.toggle", json!({ "user_id": 1, "app_id": 2 })),
            ("settings.get", json!({ "user_id": 1 })),
            ("settings.update", json!({ "user_id": 1, "theme": "dark" })),
        ] {
            let v = call(m, p).await;
            assert_eq!(v["error"]["code"], "db_unavailable", "method {m}");
        }
    }

    #[tokio::test]
    async fn bad_params_are_rejected() {
        let v = call("favorites.toggle", json!({ "user_id": 1 })).await;
        assert_eq!(v["ok"], false);
        assert_eq!(v["error"]["code"], "internal");
    }

    #[tokio::test]
    async fn settings_update_validates_before_touching_db() {
        // Invalid enum is caught in the application layer → internal, not db_unavailable.
        let v = call("settings.update", json!({ "user_id": 1, "theme": "neon" })).await;
        assert_eq!(v["error"]["code"], "internal");
    }
}
