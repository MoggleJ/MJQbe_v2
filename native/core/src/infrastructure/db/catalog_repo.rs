use async_trait::async_trait;
use sqlx::Row;

use crate::domain::{App, CatalogRepository, Category, CoreError};

use super::Db;

pub struct PgCatalogRepository {
    db: Db,
}

impl PgCatalogRepository {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}

fn app_from_row(r: &sqlx::postgres::PgRow) -> App {
    App {
        id: r.get("id"),
        name: r.get("name"),
        icon: r.get("icon"),
        url: r.get("url"),
        category_id: r.get("category_id"),
        mode: r.get("mode"),
        is_web: r.get("is_web"),
        is_active: r.get("is_active"),
    }
}

#[async_trait]
impl CatalogRepository for PgCatalogRepository {
    async fn list_apps(&self, mode: &str, category_id: Option<i32>) -> Result<Vec<App>, CoreError> {
        let rows = sqlx::query(
            r#"
            SELECT id, name, icon, url, category_id, mode, is_web, is_active
              FROM apps
             WHERE mode = $1
               AND is_active = TRUE
               AND ($2::int IS NULL OR category_id = $2)
             ORDER BY name
            "#,
        )
        .bind(mode)
        .bind(category_id)
        .fetch_all(&self.db.pool)
        .await
        .map_err(|e| CoreError::Db(e.to_string()))?;

        Ok(rows.iter().map(app_from_row).collect())
    }

    async fn list_categories(&self, mode: &str) -> Result<Vec<Category>, CoreError> {
        let rows =
            sqlx::query("SELECT id, name, mode FROM categories WHERE mode = $1 ORDER BY name")
                .bind(mode)
                .fetch_all(&self.db.pool)
                .await
                .map_err(|e| CoreError::Db(e.to_string()))?;

        Ok(rows
            .into_iter()
            .map(|r| Category {
                id: r.get("id"),
                name: r.get("name"),
                mode: r.get("mode"),
            })
            .collect())
    }

    async fn search_apps(&self, query: &str, limit: i64) -> Result<Vec<App>, CoreError> {
        let rows = sqlx::query(
            r#"
            SELECT id, name, icon, url, category_id, mode, is_web, is_active
              FROM apps
             WHERE mode IN ('tv', 'desktop')
               AND is_active = TRUE
               AND name ILIKE '%' || $1 || '%'
             ORDER BY length(name), name
             LIMIT $2
            "#,
        )
        .bind(query)
        .bind(limit.clamp(1, 20))
        .fetch_all(&self.db.pool)
        .await
        .map_err(|e| CoreError::Db(e.to_string()))?;

        Ok(rows.iter().map(app_from_row).collect())
    }

    async fn recent_apps(
        &self,
        user_id: i32,
        mode: &str,
        limit: i64,
    ) -> Result<Vec<App>, CoreError> {
        // Most recent distinct app_launch entries for this user, in this mode.
        let rows = sqlx::query(
            r#"
            SELECT a.id, a.name, a.icon, a.url, a.category_id, a.mode, a.is_web, a.is_active
              FROM apps a
              JOIN LATERAL (
                    SELECT MAX(l.created_at) AS last_launch
                      FROM logs l
                     WHERE l.user_id = $1
                       AND l.action = 'app_launch'
                       AND (l.metadata ->> 'app_id')::int = a.id
                   ) recent ON recent.last_launch IS NOT NULL
             WHERE a.mode = $2 AND a.is_active = TRUE
             ORDER BY recent.last_launch DESC
             LIMIT $3
            "#,
        )
        .bind(user_id)
        .bind(mode)
        .bind(limit)
        .fetch_all(&self.db.pool)
        .await
        .map_err(|e| CoreError::Db(e.to_string()))?;

        Ok(rows.iter().map(app_from_row).collect())
    }
}
