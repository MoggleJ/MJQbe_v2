use serde::Deserialize;
use serde_json::{json, Value};

use crate::application::{AuthService, CatalogService};
use crate::domain::CoreError;
use crate::infrastructure::hardware::Platform;

use super::protocol::{Request, Response};

/// Routes one request to the matching use case and shapes the reply.
pub struct Handler {
    catalog: CatalogService,
    auth: AuthService,
    platform: Platform,
}

#[derive(Deserialize)]
struct ModeParams {
    mode: String,
    #[serde(default)]
    category_id: Option<i32>,
}

#[derive(Deserialize)]
struct LoginParams {
    username: String,
    password: String,
}

impl Handler {
    pub fn new(catalog: CatalogService, auth: AuthService, platform: Platform) -> Self {
        Self {
            catalog,
            auth,
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

            "apps.list" => {
                let p: ModeParams = params(req)?;
                let apps = self.catalog.list_apps(&p.mode, p.category_id).await?;
                Ok(serde_json::to_value(apps).expect("serialize apps"))
            }

            "categories.list" => {
                let p: ModeParams = params(req)?;
                let cats = self.catalog.list_categories(&p.mode).await?;
                Ok(serde_json::to_value(cats).expect("serialize categories"))
            }

            "auth.login" => {
                let p: LoginParams = params(req)?;
                let out = self.auth.login(&p.username, &p.password).await?;
                Ok(json!({
                    "user_id": out.user_id,
                    "username": out.username,
                    "role": out.role,
                }))
            }

            other => Err(CoreError::Internal(format!("unknown method: {other}"))),
        }
    }
}

fn params<T: serde::de::DeserializeOwned>(req: &Request) -> Result<T, CoreError> {
    serde_json::from_value(req.params.clone())
        .map_err(|e| CoreError::Internal(format!("bad params for {}: {e}", req.method)))
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

    fn handler() -> Handler {
        Handler::new(
            CatalogService::new(None),
            AuthService::new(None),
            Platform::Stub,
        )
    }

    fn req(method: &str, params: Value) -> Request {
        serde_json::from_value(json!({ "id": "1", "method": method, "params": params })).unwrap()
    }

    #[tokio::test]
    async fn ping_pongs() {
        let r = handler().handle(req("ping", Value::Null)).await;
        let v = serde_json::to_value(r).unwrap();
        assert_eq!(v["ok"], true);
        assert_eq!(v["data"]["pong"], true);
    }

    #[tokio::test]
    async fn health_reports_stub_platform() {
        let r = handler().handle(req("health", Value::Null)).await;
        let v = serde_json::to_value(r).unwrap();
        assert_eq!(v["data"]["platform"], "stub");
    }

    #[tokio::test]
    async fn unknown_method_is_internal_error() {
        let r = handler().handle(req("does.not.exist", Value::Null)).await;
        let v = serde_json::to_value(r).unwrap();
        assert_eq!(v["ok"], false);
        assert_eq!(v["error"]["code"], "internal");
    }

    #[tokio::test]
    async fn apps_list_without_db_is_unavailable() {
        let r = handler()
            .handle(req("apps.list", json!({ "mode": "tv" })))
            .await;
        let v = serde_json::to_value(r).unwrap();
        assert_eq!(v["error"]["code"], "db_unavailable");
    }

    #[tokio::test]
    async fn bad_params_are_rejected() {
        let r = handler()
            .handle(req("apps.list", json!({ "wrong": 1 })))
            .await;
        let v = serde_json::to_value(r).unwrap();
        assert_eq!(v["ok"], false);
        assert_eq!(v["error"]["code"], "internal");
    }
}
