use std::sync::Arc;

use crate::domain::{AuthRepository, CoreError};

/// Result of a successful local admin login.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthOutcome {
    pub user_id: i32,
    pub username: String,
    pub role: String,
}

/// Local admin authentication use case (bcrypt verify against the shared
/// `users` table). No OAuth here — that is web-only.
pub struct AuthService {
    repo: Option<Arc<dyn AuthRepository>>,
}

impl AuthService {
    pub fn new(repo: Option<Arc<dyn AuthRepository>>) -> Self {
        Self { repo }
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<AuthOutcome, CoreError> {
        let repo = self.repo.as_ref().ok_or(CoreError::DbUnavailable)?;

        let admin = repo
            .find_admin(username)
            .await?
            .ok_or(CoreError::InvalidCredentials)?;

        let hash = admin
            .password_hash
            .as_deref()
            .ok_or(CoreError::InvalidCredentials)?;

        let matches = bcrypt::verify(password, hash)
            .map_err(|e| CoreError::Internal(format!("bcrypt: {e}")))?;
        if !matches {
            return Err(CoreError::InvalidCredentials);
        }

        // Best-effort — a failed timestamp update must not fail the login.
        let _ = repo.touch_last_login(admin.id).await;

        Ok(AuthOutcome {
            user_id: admin.id,
            username: admin.username,
            role: admin.role,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::AdminRecord;
    use async_trait::async_trait;

    struct FakeRepo {
        record: Option<AdminRecord>,
    }

    #[async_trait]
    impl AuthRepository for FakeRepo {
        async fn find_admin(&self, username: &str) -> Result<Option<AdminRecord>, CoreError> {
            Ok(self.record.clone().filter(|r| r.username == username))
        }
        async fn touch_last_login(&self, _user_id: i32) -> Result<(), CoreError> {
            Ok(())
        }
    }

    fn admin_with_password(pw: &str) -> AdminRecord {
        AdminRecord {
            id: 1,
            username: "admin".into(),
            password_hash: Some(bcrypt::hash(pw, 4).unwrap()),
            role: "admin".into(),
        }
    }

    #[tokio::test]
    async fn accepts_valid_password() {
        let svc = AuthService::new(Some(Arc::new(FakeRepo {
            record: Some(admin_with_password("s3cret")),
        })));
        let out = svc.login("admin", "s3cret").await.unwrap();
        assert_eq!(out.role, "admin");
        assert_eq!(out.user_id, 1);
    }

    #[tokio::test]
    async fn rejects_wrong_password() {
        let svc = AuthService::new(Some(Arc::new(FakeRepo {
            record: Some(admin_with_password("s3cret")),
        })));
        assert!(matches!(
            svc.login("admin", "nope").await.unwrap_err(),
            CoreError::InvalidCredentials
        ));
    }

    #[tokio::test]
    async fn rejects_unknown_user() {
        let svc = AuthService::new(Some(Arc::new(FakeRepo {
            record: Some(admin_with_password("s3cret")),
        })));
        assert!(matches!(
            svc.login("ghost", "s3cret").await.unwrap_err(),
            CoreError::InvalidCredentials
        ));
    }

    #[tokio::test]
    async fn without_db_is_unavailable() {
        let svc = AuthService::new(None);
        assert!(matches!(
            svc.login("admin", "x").await.unwrap_err(),
            CoreError::DbUnavailable
        ));
    }
}
