//! End-to-end IPC test: bind a real Unix socket, connect a client, exchange
//! newline-delimited JSON. No database needed (degraded-mode handler).

use std::sync::Arc;

use mjqbe_core::application::{AuthService, CatalogService};
use mjqbe_core::infrastructure::hardware::Platform;
use mjqbe_core::interface::ipc::{handle_conn, Handler};

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{UnixListener, UnixStream};

#[tokio::test]
async fn ping_and_health_over_socket() {
    let dir = tempfile::tempdir().unwrap();
    let sock = dir.path().join("test.sock");

    let listener = UnixListener::bind(&sock).unwrap();
    let handler = Arc::new(Handler::new(
        CatalogService::new(None),
        AuthService::new(None),
        Platform::Stub,
    ));

    tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        handle_conn(stream, handler).await.unwrap();
    });

    let client = UnixStream::connect(&sock).await.unwrap();
    let (rd, mut wr) = client.into_split();
    let mut reader = BufReader::new(rd).lines();

    wr.write_all(b"{\"id\":\"a\",\"method\":\"ping\"}\n")
        .await
        .unwrap();
    let line = reader.next_line().await.unwrap().unwrap();
    let v: serde_json::Value = serde_json::from_str(&line).unwrap();
    assert_eq!(v["id"], "a");
    assert_eq!(v["ok"], true);
    assert_eq!(v["data"]["pong"], true);

    wr.write_all(b"{\"id\":\"b\",\"method\":\"health\"}\n")
        .await
        .unwrap();
    let line = reader.next_line().await.unwrap().unwrap();
    let v: serde_json::Value = serde_json::from_str(&line).unwrap();
    assert_eq!(v["data"]["platform"], "stub");

    wr.write_all(b"not json at all\n").await.unwrap();
    let line = reader.next_line().await.unwrap().unwrap();
    let v: serde_json::Value = serde_json::from_str(&line).unwrap();
    assert_eq!(v["ok"], false);
    assert_eq!(v["error"]["code"], "bad_request");
}
