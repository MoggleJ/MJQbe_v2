use std::sync::Arc;

use crate::domain::{CoreError, FavoritesRepository};

/// Favourites use cases: list + toggle, persisted in PostgreSQL.
pub struct FavoritesService {
    repo: Option<Arc<dyn FavoritesRepository>>,
}

impl FavoritesService {
    pub fn new(repo: Option<Arc<dyn FavoritesRepository>>) -> Self {
        Self { repo }
    }

    pub async fn list(&self, user_id: i32) -> Result<Vec<i32>, CoreError> {
        self.repo()?.list(user_id).await
    }

    /// Returns the new favourite state (`true` = now a favourite).
    pub async fn toggle(&self, user_id: i32, app_id: i32) -> Result<bool, CoreError> {
        self.repo()?.toggle(user_id, app_id).await
    }

    fn repo(&self) -> Result<&Arc<dyn FavoritesRepository>, CoreError> {
        self.repo.as_ref().ok_or(CoreError::DbUnavailable)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::Mutex;

    #[derive(Default)]
    struct FakeRepo {
        favs: Mutex<Vec<i32>>,
    }

    #[async_trait]
    impl FavoritesRepository for FakeRepo {
        async fn list(&self, _user_id: i32) -> Result<Vec<i32>, CoreError> {
            Ok(self.favs.lock().unwrap().clone())
        }
        async fn toggle(&self, _user_id: i32, app_id: i32) -> Result<bool, CoreError> {
            let mut favs = self.favs.lock().unwrap();
            if let Some(pos) = favs.iter().position(|&a| a == app_id) {
                favs.remove(pos);
                Ok(false)
            } else {
                favs.push(app_id);
                Ok(true)
            }
        }
    }

    #[tokio::test]
    async fn toggle_adds_then_removes() {
        let svc = FavoritesService::new(Some(Arc::new(FakeRepo::default())));
        assert!(svc.toggle(1, 42).await.unwrap());
        assert_eq!(svc.list(1).await.unwrap(), vec![42]);
        assert!(!svc.toggle(1, 42).await.unwrap());
        assert!(svc.list(1).await.unwrap().is_empty());
    }

    #[tokio::test]
    async fn without_db_is_unavailable() {
        let svc = FavoritesService::new(None);
        assert!(matches!(
            svc.list(1).await.unwrap_err(),
            CoreError::DbUnavailable
        ));
    }
}
