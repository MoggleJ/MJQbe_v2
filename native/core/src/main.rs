//! MJQbe native core binary — thin wrapper over the `mjqbe_core` library.

use std::sync::Arc;

use tracing_subscriber::EnvFilter;

use mjqbe_core::application::{AuthService, CatalogService};
use mjqbe_core::config::Config;
use mjqbe_core::domain;
use mjqbe_core::infrastructure::db::{Db, PgAuthRepository, PgCatalogRepository};
use mjqbe_core::infrastructure::hardware::Platform;
use mjqbe_core::interface::ipc::{self, Handler};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let cfg = Config::from_env();
    let platform = Platform::detect();
    tracing::info!(socket = %cfg.socket_path, %platform, "mjqbe-core starting");

    // The DB is optional at boot: the UI must still open (degraded mode) when
    // PostgreSQL is unreachable — an explicit Sprint 3 requirement.
    let db = match Db::connect(&cfg.database_url, cfg.db_pool_size).await {
        Ok(db) => {
            tracing::info!("connected to PostgreSQL");
            Some(db)
        }
        Err(e) => {
            tracing::warn!(error = %e, "PostgreSQL unavailable — degraded mode (no catalog / no auth)");
            None
        }
    };

    let catalog_repo = db
        .clone()
        .map(|db| Arc::new(PgCatalogRepository::new(db)) as Arc<dyn domain::CatalogRepository>);
    let auth_repo =
        db.map(|db| Arc::new(PgAuthRepository::new(db)) as Arc<dyn domain::AuthRepository>);

    let handler = Arc::new(Handler::new(
        CatalogService::new(catalog_repo),
        AuthService::new(auth_repo),
        platform,
    ));

    ipc::serve(&cfg.socket_path, handler).await
}
