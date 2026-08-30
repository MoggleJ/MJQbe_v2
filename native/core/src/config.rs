//! Runtime configuration, read from the environment.
//!
//! Secrets (DB password) never live in `config/config.yml` — they arrive here
//! via environment variables, exactly like the FastAPI side.

/// Effective configuration for one `mjqbe-core` process.
#[derive(Debug, Clone)]
pub struct Config {
    /// Path of the Unix socket the IPC server binds to.
    pub socket_path: String,
    /// PostgreSQL connection string.
    pub database_url: String,
    /// Connection pool upper bound.
    pub db_pool_size: u32,
}

impl Config {
    pub fn from_env() -> Self {
        let socket_path = env_or("MJQBE_NATIVE_SOCKET", "/run/mjqbe/native.sock");

        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            let user = env_or("POSTGRES_USER", "mjqbe");
            let pass = env_or("POSTGRES_PASSWORD", "mjqbe");
            let host = env_or("POSTGRES_HOST", "localhost");
            let port = env_or("POSTGRES_PORT", "5432");
            let name = env_or("POSTGRES_DB", "mjqbe");
            format!("postgres://{user}:{pass}@{host}:{port}/{name}?sslmode=disable")
        });

        let db_pool_size = std::env::var("MJQBE_DB_POOL_SIZE")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(5);

        Self {
            socket_path,
            database_url,
            db_pool_size,
        }
    }
}

fn env_or(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}
