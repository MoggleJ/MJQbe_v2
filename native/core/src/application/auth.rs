use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use rand::Rng;

use crate::domain::{AuthRepository, CoreError};

/// Short-lived tokens gate destructive Dev-mode actions ("re-auth required").
const REAUTH_TTL: Duration = Duration::from_secs(120);

/// Result of a successful local admin login / the current native session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthOutcome {
    pub user_id: i32,
    pub username: String,
    pub role: String,
}

/// Local admin authentication use case (bcrypt verify against the shared
/// `users` table). No OAuth here — that is web-only. Also mints and checks the
/// re-auth tokens required before destructive Dev actions.
pub struct AuthService {
    repo: Option<Arc<dyn AuthRepository>>,
    tokens: Mutex<HashMap<String, Instant>>,
}

impl AuthService {
    pub fn new(repo: Option<Arc<dyn AuthRepository>>) -> Self {
        Self {
            repo,
            tokens: Mutex::new(HashMap::new()),
        }
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<AuthOutcome, CoreError> {
        let repo = self.repo()?;
        let admin = repo
            .find_admin(username)
            .await?
            .ok_or(CoreError::InvalidCredentials)?;
        verify_password(password, admin.password_hash.as_deref())?;
        let _ = repo.touch_last_login(admin.id).await;
        Ok(AuthOutcome {
            user_id: admin.id,
            username: admin.username,
            role: admin.role,
        })
    }

    /// The implicit single-seat native user (lowest-id admin).
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

    /// Re-check the password and mint a token valid for [`REAUTH_TTL`].
    pub async fn verify(
        &self,
        username: Option<&str>,
        password: &str,
    ) -> Result<(String, u64), CoreError> {
        let repo = self.repo()?;
        let admin = match username {
            Some(u) => repo.find_admin(u).await?,
            None => repo.default_admin().await?,
        }
        .ok_or(CoreError::InvalidCredentials)?;
        verify_password(password, admin.password_hash.as_deref())?;

        let token: String = {
            let mut rng = rand::thread_rng();
            (0..32)
                .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
                .collect()
        };
        self.prune_and_insert(token.clone());
        Ok((token, REAUTH_TTL.as_secs()))
    }

    /// Consume a token: valid exactly once, and only within its TTL.
    pub fn check_token(&self, token: &str) -> Result<(), CoreError> {
        let mut tokens = self.tokens.lock().unwrap();
        let now = Instant::now();
        tokens.retain(|_, &mut created| now.duration_since(created) < REAUTH_TTL);
        match tokens.remove(token) {
            Some(created) if now.duration_since(created) < REAUTH_TTL => Ok(()),
            _ => Err(CoreError::ReauthRequired),
        }
    }

    fn prune_and_insert(&self, token: String) {
        let mut tokens = self.tokens.lock().unwrap();
        let now = Instant::now();
        tokens.retain(|_, &mut created| now.duration_since(created) < REAUTH_TTL);
        tokens.insert(token, now);
    }

    fn repo(&self) -> Result<&Arc<dyn AuthRepository>, CoreError> {
        self.repo.as_ref().ok_or(CoreError::DbUnavailable)
    }
}

fn verify_password(password: &str, hash: Option<&str>) -> Result<(), CoreError> {
    let hash = hash.ok_or(CoreError::InvalidCredentials)?;
    let matches =
        bcrypt::verify(password, hash).map_err(|e| CoreError::Internal(format!("bcrypt: {e}")))?;
    if matches {
        Ok(())
    } else {
        Err(CoreError::InvalidCredentials)
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

    fn service(pw: &str) -> AuthService {
        AuthService::new(Some(Arc::new(FakeRepo {
            record: Some(AdminRecord {
                id: 1,
                username: "admin".into(),
                password_hash: Some(bcrypt::hash(pw, 4).unwrap()),
                role: "admin".into(),
            }),
        })))
    }

    #[tokio::test]
    async fn accepts_valid_password() {
        let out = service("s3cret").login("admin", "s3cret").await.unwrap();
        assert_eq!((out.role.as_str(), out.user_id), ("admin", 1));
    }

    #[tokio::test]
    async fn rejects_wrong_password() {
        assert!(matches!(
            service("s3cret").login("admin", "nope").await.unwrap_err(),
            CoreError::InvalidCredentials
        ));
    }

    #[tokio::test]
    async fn current_user_returns_default_admin() {
        assert_eq!(service("x").current_user().await.unwrap().user_id, 1);
    }

    #[tokio::test]
    async fn verify_mints_a_single_use_token() {
        let svc = service("s3cret");
        let (token, ttl) = svc.verify(None, "s3cret").await.unwrap();
        assert!(ttl >= 60);
        assert!(svc.check_token(&token).is_ok());
        // second use fails
        assert!(matches!(
            svc.check_token(&token).unwrap_err(),
            CoreError::ReauthRequired
        ));
    }

    #[tokio::test]
    async fn verify_rejects_bad_password() {
        assert!(matches!(
            service("s3cret").verify(None, "nope").await.unwrap_err(),
            CoreError::InvalidCredentials
        ));
    }

    #[tokio::test]
    async fn unknown_token_is_rejected() {
        assert!(matches!(
            service("x").check_token("nope").unwrap_err(),
            CoreError::ReauthRequired
        ));
    }

    #[tokio::test]
    async fn without_db_is_unavailable() {
        assert!(matches!(
            AuthService::new(None)
                .login("admin", "x")
                .await
                .unwrap_err(),
            CoreError::DbUnavailable
        ));
    }
}
