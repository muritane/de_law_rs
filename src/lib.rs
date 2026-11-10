//! Minimal, machine-native representations of selected German legal rules.
//!
//! The library is intentionally ordinary Rust: no legal DSL, XML runtime,
//! database, network service, or special build system.
//!
//! This is an experimental implementation, not an official legal publication.

#![forbid(unsafe_code)]

pub mod date;
pub mod de;

pub use date::{AgeError, Date, DateError, has_reached_age};
