use async_trait::async_trait;
use sqlx::Row;

use crate::domain::{CoreError, Settings, SettingsRepository};

use super::Db;

pub struct PgSettingsRepository {
    db: Db,
}

impl PgSettingsRepository {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}

fn settings_from_row(r: &sqlx::postgres::PgRow) -> Settings {
    Settings {
        user_id: r.get("user_id"),
        theme: r.get("theme"),
        layout: r.get("layout"),
        icon_size: r.get("icon_size"),
        default_mode: r.get("default_mode"),
    }
}

#[async_trait]
impl SettingsRepository for PgSettingsRepository {
    async fn get_or_create(&self, user_id: i32) -> Result<Settings, CoreError> {
        let row = sqlx::query(
            r#"
            INSERT INTO settings (user_id) VALUES ($1)
            ON CONFLICT (user_id) DO UPDATE SET user_id = EXCLUDED.user_id
            RETURNING user_id, theme, layout, icon_size, default_mode
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.db.pool)
        .await
        .map_err(|e| CoreError::Db(e.to_string()))?;

        Ok(settings_from_row(&row))
    }

    async fn update(
        &self,
        user_id: i32,
        theme: Option<&str>,
        layout: Option<&str>,
        icon_size: Option<&str>,
        default_mode: Option<&str>,
    ) -> Result<Settings, CoreError> {
        // Ensure the row exists, then COALESCE each column with its new value.
        self.get_or_create(user_id).await?;

        let row = sqlx::query(
            r#"
            UPDATE settings
               SET theme        = COALESCE($2, theme),
                   layout       = COALESCE($3, layout),
                   icon_size    = COALESCE($4, icon_size),
                   default_mode = COALESCE($5, default_mode)
             WHERE user_id = $1
            RETURNING user_id, theme, layout, icon_size, default_mode
            "#,
        )
        .bind(user_id)
        .bind(theme)
        .bind(layout)
        .bind(icon_size)
        .bind(default_mode)
        .fetch_one(&self.db.pool)
        .await
        .map_err(|e| CoreError::Db(e.to_string()))?;

        Ok(settings_from_row(&row))
    }
}
