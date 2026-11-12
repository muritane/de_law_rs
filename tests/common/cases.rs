//! Representative cases shared by the library, CLI and HTTP service tests.
//! Expected outcomes are classifications of `de_law::de::bgb::majority_at`;
//! consumers must not re-derive them.

#![allow(dead_code)]

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Outcome {
    Minor,
    Adult,
    InvalidDate,
    BeforeBirth,
    Unsupported,
}

/// `(born, on, expected)`
pub const CASES: &[(&str, &str, Outcome)] = &[
    // Immediately before and on the 18th birthday.
    ("2008-11-10", "2026-11-09", Outcome::Minor),
    ("2008-11-10", "2026-11-10", Outcome::Adult),
    // Leap-day birth in a non-leap year.
    ("2008-02-29", "2026-02-28", Outcome::Minor),
    ("2008-02-29", "2026-03-01", Outcome::Adult),
    // Malformed or impossible dates.
    ("2008-1-10", "2026-11-10", Outcome::InvalidDate),
    ("2008-02-30", "2026-11-10", Outcome::InvalidDate),
    ("2008-11-10", "not-a-date", Outcome::InvalidDate),
    ("", "2026-11-10", Outcome::InvalidDate),
    // Supported-date boundaries.
    ("1957-01-01", "1975-01-01", Outcome::Adult),
    ("1957-01-01", "1974-12-31", Outcome::Unsupported),
    ("2026-01-01", "2025-12-31", Outcome::BeforeBirth),
];
