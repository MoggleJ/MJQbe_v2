use thiserror::Error;

/// Every failure the core can surface to the UI.
///
/// The IPC layer maps each variant to a stable string `code` so the QML side
/// can branch on it without parsing messages.
#[derive(Debug, Error)]
pub enum CoreError {
    #[error("database unavailable")]
    DbUnavailable,

    #[error("resource not found")]
    NotFound,

    #[error("invalid credentials")]
    InvalidCredentials,

    #[error("hardware unavailable on this platform (stub mode)")]
    HardwareUnavailable,

    #[error("re-authentication required")]
    ReauthRequired,

    #[error("operation not permitted: {0}")]
    PermissionDenied(String),

    #[error("database error: {0}")]
    Db(String),

    #[error("{0}")]
    Internal(String),
}
