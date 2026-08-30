//! Domain layer — pure data + behaviour contracts, zero I/O.

mod entities;
mod error;
mod repository;

pub use entities::{
    AdminRecord, App, Category, DockerContainer, ParsedUtterance, ProcessInfo, Settings,
    SettingsPatch, SystemSnapshot, VoiceAction, ICON_SIZES, LAYOUTS, THEMES, USER_MODES,
};
pub use error::CoreError;
pub use repository::{AuthRepository, CatalogRepository, FavoritesRepository, SettingsRepository};
