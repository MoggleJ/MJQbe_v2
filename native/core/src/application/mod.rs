//! Application layer — use cases. Holds no I/O of its own; it drives the
//! domain repository traits, which infrastructure implements.

mod auth;
mod catalog;
mod dev;
mod favorites;
mod settings;

pub use auth::{AuthOutcome, AuthService};
pub use catalog::CatalogService;
pub use dev::DevService;
pub use favorites::FavoritesService;
pub use settings::SettingsService;

use crate::domain::CoreError;

pub(crate) fn validate_mode(mode: &str) -> Result<(), CoreError> {
    match mode {
        "tv" | "desktop" | "dev" => Ok(()),
        other => Err(CoreError::Internal(format!("invalid mode: {other}"))),
    }
}

/// Rejects a value that is not in `allowed`.
pub(crate) fn validate_enum(field: &str, value: &str, allowed: &[&str]) -> Result<(), CoreError> {
    if allowed.contains(&value) {
        Ok(())
    } else {
        Err(CoreError::Internal(format!(
            "invalid {field}: {value} (expected one of {allowed:?})"
        )))
    }
}
