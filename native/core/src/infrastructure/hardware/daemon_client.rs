//! Client for the C hardware daemon (`mjqbe-daemon`).
//!
//! Same wire protocol as the daemon: one JSON object per line over a Unix
//! socket. One short-lived connection per request — the daemon is local and
//! requests are rare (GPIO toggles), so this keeps the client trivial and
//! stateless.

use std::time::Duration;

use serde_json::{json, Value};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;

use crate::domain::CoreError;

pub struct DaemonClient {
    socket_path: String,
}

impl DaemonClient {
    pub fn new(socket_path: impl Into<String>) -> Self {
        Self {
            socket_path: socket_path.into(),
        }
    }

    /// Reads `DAEMON_SOCKET`, else `/run/mjqbe/daemon.sock`.
    pub fn from_env() -> Self {
        Self::new(
            std::env::var("DAEMON_SOCKET").unwrap_or_else(|_| "/run/mjqbe/daemon.sock".to_string()),
        )
    }

    async fn request(&self, cmd: &str, mut params: Value) -> Result<Value, CoreError> {
        params["cmd"] = json!(cmd);

        let connect = UnixStream::connect(&self.socket_path);
        let stream = tokio::time::timeout(Duration::from_secs(8), connect)
            .await
            .map_err(|_| CoreError::HardwareUnavailable)?
            .map_err(|_| CoreError::HardwareUnavailable)?;

        let (rd, mut wr) = stream.into_split();
        let mut line = serde_json::to_vec(&params).expect("serialize daemon request");
        line.push(b'\n');
        wr.write_all(&line)
            .await
            .map_err(|e| CoreError::Internal(e.to_string()))?;
        wr.flush().await.ok();

        let mut reader = BufReader::new(rd);
        let mut resp = String::new();
        tokio::time::timeout(Duration::from_secs(8), reader.read_line(&mut resp))
            .await
            .map_err(|_| CoreError::Internal("daemon timeout".into()))?
            .map_err(|e| CoreError::Internal(e.to_string()))?;

        let v: Value = serde_json::from_str(resp.trim())
            .map_err(|e| CoreError::Internal(format!("bad daemon response: {e}")))?;

        if v.get("ok").and_then(Value::as_bool).unwrap_or(false) {
            Ok(v.get("data").cloned().unwrap_or(Value::Null))
        } else {
            let msg = v
                .get("error")
                .and_then(Value::as_str)
                .unwrap_or("daemon error")
                .to_string();
            Err(CoreError::Internal(msg))
        }
    }

    pub async fn info(&self) -> Result<Value, CoreError> {
        self.request("info", json!({})).await
    }

    pub async fn gpio_set(&self, pin: u8, value: bool) -> Result<Value, CoreError> {
        self.request("gpio_set", json!({ "pin": pin, "value": value as u8 }))
            .await
    }

    pub async fn gpio_get(&self, pin: u8) -> Result<bool, CoreError> {
        let d = self.request("gpio_get", json!({ "pin": pin })).await?;
        Ok(d.get("value").and_then(Value::as_i64).unwrap_or(0) != 0)
    }

    pub async fn relay_set(&self, relay: u8, state: bool) -> Result<Value, CoreError> {
        self.request("relay_set", json!({ "relay": relay, "state": state as u8 }))
            .await
    }

    pub async fn led_set(&self, r: u8, g: u8, b: u8) -> Result<Value, CoreError> {
        self.request("led_set", json!({ "r": r, "g": g, "b": b }))
            .await
    }

    pub async fn cec_send(&self, action: &str) -> Result<Value, CoreError> {
        self.request("cec_send", json!({ "action": action })).await
    }

    pub async fn av_status(&self) -> Result<Value, CoreError> {
        self.request("av_status", json!({})).await
    }
}
