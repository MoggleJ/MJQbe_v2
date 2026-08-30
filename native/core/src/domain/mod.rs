//! Domain layer — pure data + behaviour contracts, zero I/O.

mod entities;
mod error;
mod repository;

pub use entities::{AdminRecord, App, Category};
pub use error::CoreError;
pub use repository::{AuthRepository, CatalogRepository};
