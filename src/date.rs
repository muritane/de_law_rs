//! A validated civil date for rule inputs. Gregorian calendar, years 0001–9999.

use std::error::Error;
use std::fmt;
use std::str::FromStr;

use chrono::{Datelike, NaiveDate};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Date(NaiveDate);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DateError {
    InvalidFormat,
    InvalidValue,
}

impl fmt::Display for DateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormat => write!(f, "expected a date in YYYY-MM-DD format"),
            Self::InvalidValue => write!(f, "date is outside the supported Gregorian calendar"),
        }
    }
}

impl Error for DateError {}

impl Date {
    /// Construct a validated calendar date.
    pub fn new(year: i32, month: u8, day: u8) -> Result<Self, DateError> {
        if !(1..=9999).contains(&year) {
            return Err(DateError::InvalidValue);
        }

        NaiveDate::from_ymd_opt(year, u32::from(month), u32::from(day))
            .map(Self)
            .ok_or(DateError::InvalidValue)
    }

    pub fn year(self) -> i32 {
        self.0.year()
    }

    pub fn month(self) -> u8 {
        self.0.month() as u8
    }

    pub fn day(self) -> u8 {
        self.0.day() as u8
    }
}

impl FromStr for Date {
    type Err = DateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let date = NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|error| {
            use chrono::format::ParseErrorKind::{Impossible, OutOfRange};

            match error.kind() {
                Impossible | OutOfRange => DateError::InvalidValue,
                _ => DateError::InvalidFormat,
            }
        })?;
        if date.format("%Y-%m-%d").to_string() != s {
            return Err(DateError::InvalidFormat);
        }
        if !(1..=9999).contains(&date.year()) {
            return Err(DateError::InvalidValue);
        }
        Ok(Self(date))
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%d"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_leap_years_and_centuries() {
        assert!(Date::new(2000, 2, 29).is_ok());
        assert!(Date::new(1900, 2, 29).is_err());
        assert!(Date::new(2026, 2, 29).is_err());
        assert!(Date::new(2026, 4, 31).is_err());
        assert!(Date::new(0, 1, 1).is_err());
    }

    #[test]
    fn round_trip_date() {
        let d: Date = "2026-11-10".parse().unwrap();
        assert_eq!(d.to_string(), "2026-11-10");
    }

    #[test]
    fn preserves_strict_iso_date_input_and_year_range() {
        assert_eq!("2026-1-2".parse::<Date>(), Err(DateError::InvalidFormat));
        assert_eq!("0000-01-01".parse::<Date>(), Err(DateError::InvalidValue));
        assert_eq!("10000-01-01".parse::<Date>(), Err(DateError::InvalidFormat));
    }
}
