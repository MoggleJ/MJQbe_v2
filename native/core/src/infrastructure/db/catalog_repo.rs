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

        Ok(rows
            .into_iter()
            .map(|r| App {
                id: r.get("id"),
                name: r.get("name"),
                icon: r.get("icon"),
                url: r.get("url"),
                category_id: r.get("category_id"),
                mode: r.get("mode"),
                is_web: r.get("is_web"),
                is_active: r.get("is_active"),
            })
            .collect())
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
}
