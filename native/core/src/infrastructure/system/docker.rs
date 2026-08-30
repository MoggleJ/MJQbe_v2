//! Docker control via the `docker` CLI (`docker ps` / `start` / `stop`).
//! The CDC allows either the CLI or the Unix socket API — the CLI keeps this
//! dependency-free and matches how the rest of the project drives Docker.

use crate::domain::{CoreError, DockerContainer};

pub struct DockerCli;

impl DockerCli {
    pub async fn list() -> Result<Vec<DockerContainer>, CoreError> {
        let out = run(&[
            "ps",
            "-a",
            "--no-trunc",
            "--format",
            "{{.ID}}\t{{.Names}}\t{{.Image}}\t{{.State}}\t{{.Status}}",
        ])
        .await?;

        Ok(out
            .lines()
            .filter_map(|line| {
                let mut f = line.split('\t');
                Some(DockerContainer {
                    id: f.next()?.chars().take(12).collect(),
                    name: f.next()?.to_string(),
                    image: f.next()?.to_string(),
                    state: f.next().unwrap_or("").to_string(),
                    status: f.next().unwrap_or("").to_string(),
                })
            })
            .collect())
    }

    pub async fn start(id: &str) -> Result<(), CoreError> {
        validate_id(id)?;
        run(&["start", id]).await.map(|_| ())
    }

    pub async fn stop(id: &str) -> Result<(), CoreError> {
        validate_id(id)?;
        run(&["stop", id]).await.map(|_| ())
    }
}

/// Container ids / names: alphanumerics plus `_.-` only (blocks arg injection).
fn validate_id(id: &str) -> Result<(), CoreError> {
    if !id.is_empty()
        && id.len() <= 128
        && id
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '-'))
    {
        Ok(())
    } else {
        Err(CoreError::Internal(format!("invalid container id: {id:?}")))
    }
}

async fn run(args: &[&str]) -> Result<String, CoreError> {
    let output = tokio::process::Command::new("docker")
        .args(args)
        .output()
        .await
        .map_err(|e| CoreError::Internal(format!("docker not available: {e}")))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(CoreError::Internal(format!(
            "docker {}: {}",
            args.first().unwrap_or(&""),
            stderr.trim()
        )));
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_validation() {
        assert!(validate_id("mjqbe_v2-db-1").is_ok());
        assert!(validate_id("7e24b382447f").is_ok());
        assert!(validate_id("bad; rm -rf").is_err());
        assert!(validate_id("").is_err());
    }
}
