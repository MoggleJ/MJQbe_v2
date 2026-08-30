//! End-to-end IPC test: bind a real Unix socket, connect a client, exchange
//! newline-delimited JSON. No database needed (degraded-mode handler).

use std::sync::Arc;

use mjqbe_core::application::{AuthService, CatalogService, FavoritesService, SettingsService};
use mjqbe_core::infrastructure::hardware::Platform;
use mjqbe_core::interface::ipc::{handle_conn, Handler, Services};

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{UnixListener, UnixStream};

#[tokio::test]
async fn ping_health_and_error_over_socket() {
    let dir = tempfile::tempdir().unwrap();
    let sock = dir.path().join("test.sock");

    let listener = UnixListener::bind(&sock).unwrap();
    let handler = Arc::new(Handler::new(
        Services {
            catalog: CatalogService::new(None),
            auth: AuthService::new(None),
            favorites: FavoritesService::new(None),
            settings: SettingsService::new(None),
        },
        Platform::Stub,
    ));

    tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        handle_conn(stream, handler).await.unwrap();
    });

    let client = UnixStream::connect(&sock).await.unwrap();
    let (rd, mut wr) = client.into_split();
    let mut reader = BufReader::new(rd).lines();

    async fn roundtrip(
        wr: &mut (impl AsyncWriteExt + Unpin),
        reader: &mut tokio::io::Lines<impl AsyncBufReadExt + Unpin>,
        line: &str,
    ) -> serde_json::Value {
        wr.write_all(line.as_bytes()).await.unwrap();
        wr.write_all(b"\n").await.unwrap();
        serde_json::from_str(&reader.next_line().await.unwrap().unwrap()).unwrap()
    }

    let v = roundtrip(&mut wr, &mut reader, r#"{"id":"a","method":"ping"}"#).await;
    assert_eq!(v["id"], "a");
    assert_eq!(v["data"]["pong"], true);

    let v = roundtrip(&mut wr, &mut reader, r#"{"id":"b","method":"health"}"#).await;
    assert_eq!(v["data"]["platform"], "stub");

    let v = roundtrip(
        &mut wr,
        &mut reader,
        r#"{"id":"c","method":"favorites.list","params":{"user_id":1}}"#,
    )
    .await;
    assert_eq!(v["ok"], false);
    assert_eq!(v["error"]["code"], "db_unavailable");

    let v = roundtrip(&mut wr, &mut reader, "not json at all").await;
    assert_eq!(v["ok"], false);
    assert_eq!(v["error"]["code"], "bad_request");
}
