use serde::{Deserialize, Serialize};
use serde_json::Value;

/// One inbound request line.
#[derive(Debug, Deserialize)]
pub struct Request {
    /// Opaque id echoed back on the matching response (optional).
    #[serde(default)]
    pub id: Option<String>,
    pub method: String,
    #[serde(default)]
    pub params: Value,
}

/// One outbound response line.
#[derive(Debug, Serialize)]
pub struct Response {
    pub id: Option<String>,
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorBody>,
}

#[derive(Debug, Serialize)]
pub struct ErrorBody {
    pub code: String,
    pub message: String,
}

impl Response {
    pub fn ok(id: Option<String>, data: Value) -> Self {
        Self {
            id,
            ok: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(id: Option<String>, code: &str, message: &str) -> Self {
        Self {
            id,
            ok: false,
            data: None,
            error: Some(ErrorBody {
                code: code.to_string(),
                message: message.to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_request_without_id_or_params() {
        let r: Request = serde_json::from_str(r#"{"method":"ping"}"#).unwrap();
        assert_eq!(r.method, "ping");
        assert!(r.id.is_none());
        assert!(r.params.is_null());
    }

    #[test]
    fn ok_response_shape() {
        let json = serde_json::to_value(Response::ok(
            Some("7".into()),
            serde_json::json!({"pong": true}),
        ))
        .unwrap();
        assert_eq!(json["ok"], true);
        assert_eq!(json["id"], "7");
        assert_eq!(json["data"]["pong"], true);
        assert!(json.get("error").is_none());
    }

    #[test]
    fn error_response_shape() {
        let json = serde_json::to_value(Response::error(None, "not_found", "nope")).unwrap();
        assert_eq!(json["ok"], false);
        assert_eq!(json["error"]["code"], "not_found");
        assert!(json.get("data").is_none());
    }
}
