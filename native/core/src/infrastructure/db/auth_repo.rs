use async_trait::async_trait;
use sqlx::Row;

use crate::domain::{AdminRecord, AuthRepository, CoreError};

use super::Db;

pub struct PgAuthRepository {
    db: Db,
}

impl PgAuthRepository {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}

fn admin_from_row(r: &sqlx::postgres::PgRow) -> AdminRecord {
    AdminRecord {
        id: r.get("id"),
        username: r.get("username"),
        password_hash: r.get("password_hash"),
        role: r.get("role"),
    }
}

#[async_trait]
impl AuthRepository for PgAuthRepository {
    async fn find_admin(&self, username: &str) -> Result<Option<AdminRecord>, CoreError> {
        let row = sqlx::query(
            "SELECT id, username, password_hash, role \
               FROM users \
              WHERE username = $1 AND role = 'admin'",
        )
        .bind(username)
        .fetch_optional(&self.db.pool)
        .await
        .map_err(|e| CoreError::Db(e.to_string()))?;

        Ok(row.as_ref().map(admin_from_row))
    }

    async fn default_admin(&self) -> Result<Option<AdminRecord>, CoreError> {
        let row = sqlx::query(
            "SELECT id, username, password_hash, role \
               FROM users WHERE role = 'admin' ORDER BY id LIMIT 1",
        )
        .fetch_optional(&self.db.pool)
        .await
        .map_err(|e| CoreError::Db(e.to_string()))?;

        Ok(row.as_ref().map(admin_from_row))
    }

    async fn touch_last_login(&self, user_id: i32) -> Result<(), CoreError> {
        sqlx::query("UPDATE users SET last_login = NOW() WHERE id = $1")
            .bind(user_id)
            .execute(&self.db.pool)
            .await
            .map_err(|e| CoreError::Db(e.to_string()))?;
        Ok(())
    }
}
