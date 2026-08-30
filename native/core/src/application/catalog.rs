use std::sync::Arc;

use crate::domain::{App, CatalogRepository, Category, CoreError};

use super::validate_mode;

/// Catalog use cases: list apps and categories for a given mode.
///
/// `repo` is `None` when the core booted without a database — every call then
/// fails with [`CoreError::DbUnavailable`] instead of panicking.
pub struct CatalogService {
    repo: Option<Arc<dyn CatalogRepository>>,
}

impl CatalogService {
    pub fn new(repo: Option<Arc<dyn CatalogRepository>>) -> Self {
        Self { repo }
    }

    pub async fn list_apps(
        &self,
        mode: &str,
        category_id: Option<i32>,
    ) -> Result<Vec<App>, CoreError> {
        validate_mode(mode)?;
        self.repo()?.list_apps(mode, category_id).await
    }

    pub async fn list_categories(&self, mode: &str) -> Result<Vec<Category>, CoreError> {
        validate_mode(mode)?;
        self.repo()?.list_categories(mode).await
    }

    fn repo(&self) -> Result<&Arc<dyn CatalogRepository>, CoreError> {
        self.repo.as_ref().ok_or(CoreError::DbUnavailable)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    struct FakeRepo;

    #[async_trait]
    impl CatalogRepository for FakeRepo {
        async fn list_apps(
            &self,
            mode: &str,
            _category_id: Option<i32>,
        ) -> Result<Vec<App>, CoreError> {
            Ok(vec![App {
                id: 1,
                name: format!("app-{mode}"),
                icon: None,
                url: Some("https://example.com".into()),
                category_id: None,
                mode: mode.to_string(),
                is_web: true,
                is_active: true,
            }])
        }

        async fn list_categories(&self, mode: &str) -> Result<Vec<Category>, CoreError> {
            Ok(vec![Category {
                id: 1,
                name: "Streaming".into(),
                mode: mode.to_string(),
            }])
        }
    }

    #[tokio::test]
    async fn rejects_unknown_mode() {
        let svc = CatalogService::new(Some(Arc::new(FakeRepo)));
        let err = svc.list_apps("holodeck", None).await.unwrap_err();
        assert!(matches!(err, CoreError::Internal(_)));
    }

    #[tokio::test]
    async fn without_db_is_unavailable() {
        let svc = CatalogService::new(None);
        assert!(matches!(
            svc.list_apps("tv", None).await.unwrap_err(),
            CoreError::DbUnavailable
        ));
    }

    #[tokio::test]
    async fn passes_through_to_repo() {
        let svc = CatalogService::new(Some(Arc::new(FakeRepo)));
        let apps = svc.list_apps("desktop", None).await.unwrap();
        assert_eq!(apps.len(), 1);
        assert_eq!(apps[0].name, "app-desktop");
        let cats = svc.list_categories("tv").await.unwrap();
        assert_eq!(cats[0].name, "Streaming");
    }
}
