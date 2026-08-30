use async_trait::async_trait;

use super::{AdminRecord, App, Category, CoreError, Settings};

/// Read access to the app catalog (apps + categories), filtered by mode.
#[async_trait]
pub trait CatalogRepository: Send + Sync {
    async fn list_apps(&self, mode: &str, category_id: Option<i32>) -> Result<Vec<App>, CoreError>;

    async fn list_categories(&self, mode: &str) -> Result<Vec<Category>, CoreError>;

    /// Most recently launched apps for a user in a mode (from `logs`).
    async fn recent_apps(
        &self,
        user_id: i32,
        mode: &str,
        limit: i64,
    ) -> Result<Vec<App>, CoreError>;
}

/// Access to local admin accounts for native authentication.
#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn find_admin(&self, username: &str) -> Result<Option<AdminRecord>, CoreError>;

    /// Lowest-id admin — the implicit "current user" for the single-seat native app.
    async fn default_admin(&self) -> Result<Option<AdminRecord>, CoreError>;

    async fn touch_last_login(&self, user_id: i32) -> Result<(), CoreError>;
}

/// Per-user favourite apps.
#[async_trait]
pub trait FavoritesRepository: Send + Sync {
    async fn list(&self, user_id: i32) -> Result<Vec<i32>, CoreError>;

    /// Adds the favourite if absent, removes it if present. Returns the new state.
    async fn toggle(&self, user_id: i32, app_id: i32) -> Result<bool, CoreError>;
}

/// Per-user preferences (1:1 with `users`).
#[async_trait]
pub trait SettingsRepository: Send + Sync {
    /// Returns the row, creating it with defaults if it does not exist yet.
    async fn get_or_create(&self, user_id: i32) -> Result<Settings, CoreError>;

    async fn update(
        &self,
        user_id: i32,
        theme: Option<&str>,
        layout: Option<&str>,
        icon_size: Option<&str>,
        default_mode: Option<&str>,
    ) -> Result<Settings, CoreError>;
}
