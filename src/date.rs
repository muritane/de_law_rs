//! A validated civil date for rule inputs. Gregorian calendar, years 0001–9999.

use std::str::FromStr;
use std::{error::Error, fmt};
use time::{Date as CalendarDate, Month};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Date(CalendarDate);

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

        let month = Month::try_from(month).map_err(|_| DateError::InvalidValue)?;
        CalendarDate::from_calendar_date(year, month, day)
            .map(Self)
            .map_err(|_| DateError::InvalidValue)
    }

    pub const fn year(self) -> i32 {
        self.0.year()
    }

    pub const fn month(self) -> u8 {
        self.0.month() as u8
    }

    pub const fn day(self) -> u8 {
        self.0.day()
    }
}

impl FromStr for Date {
    type Err = DateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        if bytes.len() != 10
            || bytes[4] != b'-'
            || bytes[7] != b'-'
            || !bytes[0..4].iter().all(|c| c.is_ascii_digit())
            || !bytes[5..7].iter().all(|c| c.is_ascii_digit())
            || !bytes[8..10].iter().all(|c| c.is_ascii_digit())
        {
            return Err(DateError::InvalidFormat);
        }
        let year = s[0..4].parse().map_err(|_| DateError::InvalidFormat)?;
        let month = s[5..7].parse().map_err(|_| DateError::InvalidFormat)?;
        let day = s[8..10].parse().map_err(|_| DateError::InvalidFormat)?;
        Self::new(year, month, day)
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
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
    fn malformed_utf8_does_not_panic() {
        assert_eq!("abcd-é-01".parse::<Date>(), Err(DateError::InvalidFormat));
    }
}
