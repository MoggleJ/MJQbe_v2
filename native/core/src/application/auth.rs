use std::sync::Arc;

use crate::domain::{AuthRepository, CoreError};

/// Result of a successful local admin login / the current native session.
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
        let repo = self.repo()?;

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

        let _ = repo.touch_last_login(admin.id).await;

        Ok(AuthOutcome {
            user_id: admin.id,
            username: admin.username,
            role: admin.role,
        })
    }

    /// The implicit single-seat native user (lowest-id admin). Used to key
    /// favourites / settings without an explicit login in dev.
    pub async fn current_user(&self) -> Result<AuthOutcome, CoreError> {
        let admin = self
            .repo()?
            .default_admin()
            .await?
            .ok_or(CoreError::NotFound)?;
        Ok(AuthOutcome {
            user_id: admin.id,
            username: admin.username,
            role: admin.role,
        })
    }

    fn repo(&self) -> Result<&Arc<dyn AuthRepository>, CoreError> {
        self.repo.as_ref().ok_or(CoreError::DbUnavailable)
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
        async fn default_admin(&self) -> Result<Option<AdminRecord>, CoreError> {
            Ok(self.record.clone())
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

    fn service(pw: &str) -> AuthService {
        AuthService::new(Some(Arc::new(FakeRepo {
            record: Some(admin_with_password(pw)),
        })))
    }

    #[tokio::test]
    async fn accepts_valid_password() {
        let out = service("s3cret").login("admin", "s3cret").await.unwrap();
        assert_eq!(out.role, "admin");
        assert_eq!(out.user_id, 1);
    }

    #[tokio::test]
    async fn rejects_wrong_password() {
        assert!(matches!(
            service("s3cret").login("admin", "nope").await.unwrap_err(),
            CoreError::InvalidCredentials
        ));
    }

    #[tokio::test]
    async fn rejects_unknown_user() {
        assert!(matches!(
            service("s3cret")
                .login("ghost", "s3cret")
                .await
                .unwrap_err(),
            CoreError::InvalidCredentials
        ));
    }

    #[tokio::test]
    async fn current_user_returns_default_admin() {
        assert_eq!(service("x").current_user().await.unwrap().user_id, 1);
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
