use async_trait::async_trait;

use super::{AdminRecord, App, Category, CoreError};

/// Read access to the app catalog (apps + categories), filtered by mode.
#[async_trait]
pub trait CatalogRepository: Send + Sync {
    async fn list_apps(&self, mode: &str, category_id: Option<i32>) -> Result<Vec<App>, CoreError>;

    async fn list_categories(&self, mode: &str) -> Result<Vec<Category>, CoreError>;
}

/// Access to local admin accounts for native authentication.
#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn find_admin(&self, username: &str) -> Result<Option<AdminRecord>, CoreError>;

    async fn touch_last_login(&self, user_id: i32) -> Result<(), CoreError>;
}
