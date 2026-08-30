//! Unix-domain socket IPC server.
//!
//! Wire format: one JSON object per line, in both directions.
//!   → `{"id":"<opaque>","method":"apps.list","params":{"mode":"tv"}}`
//!   ← `{"id":"<opaque>","ok":true,"data":[...]}`
//!   ← `{"id":"<opaque>","ok":false,"error":{"code":"db_unavailable","message":"..."}}`

mod handler;
mod protocol;

pub use handler::Handler;
pub use protocol::{ErrorBody, Request, Response};

use std::path::Path;
use std::sync::Arc;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{UnixListener, UnixStream};
use tokio::signal::unix::{signal, SignalKind};

/// Bind `socket_path` and serve until SIGINT/SIGTERM. The socket file is
/// (re)created on start and removed on shutdown.
pub async fn serve(socket_path: &str, handler: Arc<Handler>) -> anyhow::Result<()> {
    let path = Path::new(socket_path);
    if let Some(dir) = path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    if path.exists() {
        std::fs::remove_file(path)?;
    }

    let listener = UnixListener::bind(path)?;
    tracing::info!(socket = %socket_path, "IPC server listening");

    let mut sigterm = signal(SignalKind::terminate())?;

    loop {
        tokio::select! {
            accepted = listener.accept() => match accepted {
                Ok((stream, _)) => {
                    let h = Arc::clone(&handler);
                    tokio::spawn(async move {
                        if let Err(e) = handle_conn(stream, h).await {
                            tracing::debug!(error = %e, "client connection ended with error");
                        }
                    });
                }
                Err(e) => tracing::warn!(error = %e, "accept() failed"),
            },
            _ = sigterm.recv() => {
                tracing::info!("SIGTERM — shutting down");
                break;
            }
            _ = tokio::signal::ctrl_c() => {
                tracing::info!("SIGINT — shutting down");
                break;
            }
        }
    }

    let _ = std::fs::remove_file(path);
    Ok(())
}

/// Read requests line-by-line and write one response per request.
/// Public so integration tests can drive it without binding a listener.
pub async fn handle_conn(stream: UnixStream, handler: Arc<Handler>) -> anyhow::Result<()> {
    let (read_half, mut write_half) = stream.into_split();
    let mut lines = BufReader::new(read_half).lines();

    while let Some(line) = lines.next_line().await? {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let response = match serde_json::from_str::<Request>(line) {
            Ok(req) => handler.handle(req).await,
            Err(e) => Response::error(None, "bad_request", &format!("invalid JSON: {e}")),
        };

        let mut bytes = serde_json::to_vec(&response)?;
        bytes.push(b'\n');
        write_half.write_all(&bytes).await?;
        write_half.flush().await?;
    }

    Ok(())
}
