//! Minimal, machine-native representations of selected German legal rules.
//!
//! The library is intentionally ordinary Rust: no legal DSL, XML runtime,
//! database, network service, or special build system.
//!
//! This is an experimental implementation, not an official legal publication.

#![forbid(unsafe_code)]

pub mod date;
pub mod de;
pub mod person;

pub use date::{Date, DateError};
pub use person::{AgeError, NaturalPerson, age_at, has_reached_age};
