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

// ---------------------------------------------------------------------------
// Dev mode — system monitoring (Sprint 5)
// ---------------------------------------------------------------------------

/// One point-in-time reading of host resources.
#[derive(Debug, Clone, Serialize)]
pub struct SystemSnapshot {
    /// Global CPU usage, 0.0–100.0 (sampled over a short window).
    pub cpu_percent: f64,
    pub load_avg: [f64; 3],
    pub mem_total_kb: u64,
    pub mem_used_kb: u64,
    pub swap_total_kb: u64,
    pub swap_used_kb: u64,
    pub disk_total_kb: u64,
    pub disk_used_kb: u64,
    /// Aggregate throughput since the previous reading, bytes/s.
    pub net_rx_bytes_per_s: u64,
    pub net_tx_bytes_per_s: u64,
    /// SoC temperature in °C, if the platform exposes one.
    pub temp_celsius: Option<f64>,
    pub uptime_secs: u64,
}

/// A row for the process list.
#[derive(Debug, Clone, Serialize)]
pub struct ProcessInfo {
    pub pid: i32,
    pub name: String,
    pub state: String,
    pub mem_rss_kb: u64,
    pub nice: i64,
    pub threads: i64,
}

/// A Docker container as seen from `docker ps -a`.
#[derive(Debug, Clone, Serialize)]
pub struct DockerContainer {
    pub id: String,
    pub name: String,
    pub image: String,
    pub state: String,
    pub status: String,
}

// ---------------------------------------------------------------------------
// Voice control (Sprint 9)
// ---------------------------------------------------------------------------

/// What a recognised voice command resolves to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum VoiceAction {
    /// An HDMI-CEC action name (`tv_on`, `tv_off`, `ps4_on`, ...).
    Cec { action: String },
    /// A relay toggle (relay 1 = the hub).
    Relay { relay: u8, on: bool },
    /// Launch an app by (fuzzy) name — the caller resolves it against the catalog.
    LaunchApp { query: String },
}

/// Result of interpreting one utterance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ParsedUtterance {
    /// Whether the wake word ("ok hub") was present.
    pub wake: bool,
    /// The command portion after the wake word, normalised.
    pub command: Option<String>,
    /// The action the command maps to, if any.
    pub action: Option<VoiceAction>,
}
