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

/// Per-user preferences (1:1 with `users`). Mirrors the `settings` table.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Settings {
    pub user_id: i32,
    pub theme: String,
    pub layout: String,
    pub icon_size: String,
    pub default_mode: String,
}

/// Fields a client may change in one `settings.update` call. `None` = keep.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct SettingsPatch {
    pub theme: Option<String>,
    pub layout: Option<String>,
    pub icon_size: Option<String>,
    pub default_mode: Option<String>,
}

/// Allowed values, validated in the application layer.
pub const THEMES: [&str; 10] = [
    "amoled",
    "dark",
    "dark-blue",
    "dark-purple",
    "dark-green",
    "light",
    "light-warm",
    "light-blue",
    "light-purple",
    "light-green",
];
pub const LAYOUTS: [&str; 2] = ["grid", "list"];
pub const ICON_SIZES: [&str; 3] = ["small", "medium", "large"];
pub const USER_MODES: [&str; 2] = ["tv", "desktop"];
