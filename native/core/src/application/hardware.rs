//! Hardware use cases — thin pass-through to the C daemon via [`DaemonClient`].
//! Mutations are re-auth-gated at the IPC layer (like the other Dev actions).

use serde_json::Value;

use crate::domain::CoreError;
use crate::infrastructure::hardware::DaemonClient;

pub struct HardwareService {
    client: DaemonClient,
}

impl HardwareService {
    pub fn new(client: DaemonClient) -> Self {
        Self { client }
    }

    pub async fn info(&self) -> Result<Value, CoreError> {
        self.client.info().await
    }

    pub async fn gpio_set(&self, pin: u8, value: bool) -> Result<Value, CoreError> {
        self.client.gpio_set(pin, value).await
    }

    pub async fn gpio_get(&self, pin: u8) -> Result<bool, CoreError> {
        self.client.gpio_get(pin).await
    }

    pub async fn relay_set(&self, relay: u8, state: bool) -> Result<Value, CoreError> {
        if !(1..=4).contains(&relay) {
            return Err(CoreError::Internal(format!("relay out of range: {relay}")));
        }
        self.client.relay_set(relay, state).await
    }

    pub async fn led_set(&self, r: u8, g: u8, b: u8) -> Result<Value, CoreError> {
        self.client.led_set(r, g, b).await
    }

    pub async fn av_status(&self) -> Result<Value, CoreError> {
        self.client.av_status().await
    }

    pub async fn av_cec(&self, action: &str) -> Result<Value, CoreError> {
        const ACTIONS: [&str; 5] = ["tv_on", "tv_off", "tv_toggle", "ps4_on", "ps4_off"];
        if !ACTIONS.contains(&action) {
            return Err(CoreError::Internal(format!("invalid AV action: {action}")));
        }
        self.client.cec_send(action).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn relay_range_is_validated_before_hitting_the_daemon() {
        let svc = HardwareService::new(DaemonClient::new("/nonexistent.sock"));
        assert!(matches!(
            svc.relay_set(9, true).await.unwrap_err(),
            CoreError::Internal(_)
        ));
    }

    #[tokio::test]
    async fn missing_daemon_socket_is_hardware_unavailable() {
        let svc = HardwareService::new(DaemonClient::new("/nonexistent.sock"));
        assert!(matches!(
            svc.gpio_set(23, true).await.unwrap_err(),
            CoreError::HardwareUnavailable
        ));
    }

    #[tokio::test]
    async fn av_action_is_validated_before_hitting_the_daemon() {
        let svc = HardwareService::new(DaemonClient::new("/nonexistent.sock"));
        assert!(matches!(
            svc.av_cec("explode").await.unwrap_err(),
            CoreError::Internal(_)
        ));
    }
}
