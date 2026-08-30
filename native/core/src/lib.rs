//! MJQbe native core — library crate.
//!
//! The `mjqbe-core` binary is a thin `main` over this library; tests and future
//! tools link against it directly.
//!
//! Layering (Clean Architecture):
//!   [`domain`]         → entities + repository traits (no I/O)
//!   [`application`]     → use cases
//!   [`infrastructure`]  → PostgreSQL (sqlx), hardware platform detection
//!   [`interface`]       → IPC server, request routing

pub mod application;
pub mod config;
pub mod domain;
pub mod infrastructure;
pub mod interface;
