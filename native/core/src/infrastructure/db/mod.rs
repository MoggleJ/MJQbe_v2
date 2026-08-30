//! PostgreSQL adapters (sqlx, async). Runtime queries only — no compile-time
//! `query!` macros — so `cargo check` / CI never need a live database.

mod auth_repo;
mod catalog_repo;

pub use auth_repo::PgAuthRepository;
pub use catalog_repo::PgCatalogRepository;

use std::time::Duration;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

/// Thin wrapper around a shared connection pool. Cheap to clone.
#[derive(Clone)]
pub struct Db {
    pub pool: PgPool,
}

impl Db {
    pub async fn connect(url: &str, pool_size: u32) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(pool_size.max(1))
            .acquire_timeout(Duration::from_secs(5))
            .connect(url)
            .await?;

        // Fail fast if the connection is dead on arrival.
        sqlx::query("SELECT 1").execute(&pool).await?;

        Ok(Self { pool })
    }
}
