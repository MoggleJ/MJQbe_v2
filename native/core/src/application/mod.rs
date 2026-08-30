//! Application layer — use cases. Holds no I/O of its own; it drives the
//! domain repository traits, which infrastructure implements.

mod auth;
mod catalog;

pub use auth::{AuthOutcome, AuthService};
pub use catalog::CatalogService;

pub(crate) fn validate_mode(mode: &str) -> Result<(), crate::domain::CoreError> {
    match mode {
        "tv" | "desktop" | "dev" => Ok(()),
        other => Err(crate::domain::CoreError::Internal(format!(
            "invalid mode: {other}"
        ))),
    }
}
