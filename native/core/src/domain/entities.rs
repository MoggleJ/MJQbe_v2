use serde::{Deserialize, Serialize};

/// A category groups apps inside one mode (`tv` | `desktop` | `dev`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub mode: String,
}

/// A launchable application entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct App {
    pub id: i32,
    pub name: String,
    pub icon: Option<String>,
    pub url: Option<String>,
    pub category_id: Option<i32>,
    pub mode: String,
    /// `true` → opens in an external browser, `false` → embedded view.
    pub is_web: bool,
    pub is_active: bool,
}

/// A local admin account, as needed to authenticate the native Dev mode.
///
/// Not serialised: the password hash must never leave the process.
#[derive(Debug, Clone)]
pub struct AdminRecord {
    pub id: i32,
    pub username: String,
    pub password_hash: Option<String>,
    pub role: String,
}
