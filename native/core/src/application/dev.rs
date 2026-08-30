//! Dev-mode use cases: host monitoring + process / Docker control.
//!
//! Read-only calls (`snapshot`, `list_processes`, `list_containers`) are open.
//! Destructive calls take a re-auth `token` that the IPC layer has already
//! checked against [`crate::application::AuthService`] — this service just does
//! the work.

use crate::domain::{CoreError, DockerContainer, ProcessInfo, SystemSnapshot};
use crate::infrastructure::hardware::Platform;
use crate::infrastructure::system::{self, DockerCli};

pub struct DevService {
    platform: Platform,
}

impl DevService {
    pub fn new(platform: Platform) -> Self {
        Self { platform }
    }

    pub fn platform(&self) -> Platform {
        self.platform
    }

    pub async fn snapshot(&self) -> Result<SystemSnapshot, CoreError> {
        system::snapshot().await
    }

    pub fn list_processes(&self, limit: usize) -> Result<Vec<ProcessInfo>, CoreError> {
        system::list_processes(limit)
    }

    pub fn kill_process(&self, pid: i32, signal: Option<i32>) -> Result<(), CoreError> {
        system::terminate(pid, signal)
    }

    pub fn renice_process(&self, pid: i32, niceness: i32) -> Result<(), CoreError> {
        system::renice(pid, niceness)
    }

    pub async fn list_containers(&self) -> Result<Vec<DockerContainer>, CoreError> {
        DockerCli::list().await
    }

    pub async fn start_container(&self, id: &str) -> Result<(), CoreError> {
        DockerCli::start(id).await
    }

    pub async fn stop_container(&self, id: &str) -> Result<(), CoreError> {
        DockerCli::stop(id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn snapshot_and_process_list_work_on_this_host() {
        let dev = DevService::new(Platform::Stub);
        assert!(dev.snapshot().await.unwrap().mem_total_kb > 0);
        assert!(!dev.list_processes(20).unwrap().is_empty());
    }

    #[test]
    fn kill_init_is_denied() {
        let dev = DevService::new(Platform::Stub);
        assert!(matches!(
            dev.kill_process(1, None).unwrap_err(),
            CoreError::PermissionDenied(_)
        ));
    }
}
