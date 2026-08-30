use std::sync::Arc;

use crate::domain::{
    CoreError, Settings, SettingsPatch, SettingsRepository, ICON_SIZES, LAYOUTS, THEMES, USER_MODES,
};

use super::validate_enum;

/// Per-user preferences use cases: get (create-on-first-read) + validated update.
pub struct SettingsService {
    repo: Option<Arc<dyn SettingsRepository>>,
}

impl SettingsService {
    pub fn new(repo: Option<Arc<dyn SettingsRepository>>) -> Self {
        Self { repo }
    }

    pub async fn get(&self, user_id: i32) -> Result<Settings, CoreError> {
        self.repo()?.get_or_create(user_id).await
    }

    pub async fn update(&self, user_id: i32, patch: &SettingsPatch) -> Result<Settings, CoreError> {
        if let Some(v) = &patch.theme {
            validate_enum("theme", v, &THEMES)?;
        }
        if let Some(v) = &patch.layout {
            validate_enum("layout", v, &LAYOUTS)?;
        }
        if let Some(v) = &patch.icon_size {
            validate_enum("icon_size", v, &ICON_SIZES)?;
        }
        if let Some(v) = &patch.default_mode {
            validate_enum("default_mode", v, &USER_MODES)?;
        }

        self.repo()?
            .update(
                user_id,
                patch.theme.as_deref(),
                patch.layout.as_deref(),
                patch.icon_size.as_deref(),
                patch.default_mode.as_deref(),
            )
            .await
    }

    fn repo(&self) -> Result<&Arc<dyn SettingsRepository>, CoreError> {
        self.repo.as_ref().ok_or(CoreError::DbUnavailable)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::Mutex;

    struct FakeRepo {
        row: Mutex<Settings>,
    }

    impl FakeRepo {
        fn new() -> Self {
            Self {
                row: Mutex::new(Settings {
                    user_id: 1,
                    theme: "amoled".into(),
                    layout: "grid".into(),
                    icon_size: "medium".into(),
                    default_mode: "tv".into(),
                }),
            }
        }
    }

    #[async_trait]
    impl SettingsRepository for FakeRepo {
        async fn get_or_create(&self, _user_id: i32) -> Result<Settings, CoreError> {
            Ok(self.row.lock().unwrap().clone())
        }
        async fn update(
            &self,
            _user_id: i32,
            theme: Option<&str>,
            layout: Option<&str>,
            icon_size: Option<&str>,
            default_mode: Option<&str>,
        ) -> Result<Settings, CoreError> {
            let mut row = self.row.lock().unwrap();
            if let Some(v) = theme {
                row.theme = v.to_string();
            }
            if let Some(v) = layout {
                row.layout = v.to_string();
            }
            if let Some(v) = icon_size {
                row.icon_size = v.to_string();
            }
            if let Some(v) = default_mode {
                row.default_mode = v.to_string();
            }
            Ok(row.clone())
        }
    }

    fn svc() -> SettingsService {
        SettingsService::new(Some(Arc::new(FakeRepo::new())))
    }

    #[tokio::test]
    async fn get_returns_defaults() {
        assert_eq!(svc().get(1).await.unwrap().theme, "amoled");
    }

    #[tokio::test]
    async fn update_applies_valid_patch() {
        let patch = SettingsPatch {
            theme: Some("light-blue".into()),
            icon_size: Some("large".into()),
            ..Default::default()
        };
        let out = svc().update(1, &patch).await.unwrap();
        assert_eq!(out.theme, "light-blue");
        assert_eq!(out.icon_size, "large");
        assert_eq!(out.layout, "grid"); // untouched
    }

    #[tokio::test]
    async fn update_rejects_bad_theme() {
        let patch = SettingsPatch {
            theme: Some("neon".into()),
            ..Default::default()
        };
        assert!(matches!(
            svc().update(1, &patch).await.unwrap_err(),
            CoreError::Internal(_)
        ));
    }

    #[tokio::test]
    async fn update_rejects_dev_as_default_mode() {
        let patch = SettingsPatch {
            default_mode: Some("dev".into()),
            ..Default::default()
        };
        assert!(svc().update(1, &patch).await.is_err());
    }
}
