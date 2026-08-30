use async_trait::async_trait;
use sqlx::Row;

use crate::domain::{CoreError, FavoritesRepository};

use super::Db;

pub struct PgFavoritesRepository {
    db: Db,
}

impl PgFavoritesRepository {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}

#[async_trait]
impl FavoritesRepository for PgFavoritesRepository {
    async fn list(&self, user_id: i32) -> Result<Vec<i32>, CoreError> {
        let rows = sqlx::query("SELECT app_id FROM favorites WHERE user_id = $1 ORDER BY app_id")
            .bind(user_id)
            .fetch_all(&self.db.pool)
            .await
            .map_err(|e| CoreError::Db(e.to_string()))?;

        Ok(rows.iter().map(|r| r.get::<i32, _>("app_id")).collect())
    }

    async fn toggle(&self, user_id: i32, app_id: i32) -> Result<bool, CoreError> {
        // DELETE returns the row if it existed; otherwise INSERT and report added.
        let deleted =
            sqlx::query("DELETE FROM favorites WHERE user_id = $1 AND app_id = $2 RETURNING id")
                .bind(user_id)
                .bind(app_id)
                .fetch_optional(&self.db.pool)
                .await
                .map_err(|e| CoreError::Db(e.to_string()))?;

        if deleted.is_some() {
            return Ok(false);
        }

        sqlx::query("INSERT INTO favorites (user_id, app_id) VALUES ($1, $2)")
            .bind(user_id)
            .bind(app_id)
            .execute(&self.db.pool)
            .await
            .map_err(|e| CoreError::Db(e.to_string()))?;

        Ok(true)
    }
}
