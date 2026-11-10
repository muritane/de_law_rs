//! Selected rules from the German Civil Code (BGB).
//!
//! Sources for the majority calculation:
//! - § 2 BGB (age of majority): https://www.gesetze-im-internet.de/bgb/__2.html
//! - § 187(2) BGB (counting the day of birth):
//!   https://www.gesetze-im-internet.de/bgb/__187.html
//! - § 188(2)–(3) BGB (period ending, including leap-day births):
//!   https://www.gesetze-im-internet.de/bgb/__188.html
//!
//! This module implements only the current 18-year threshold, as applicable
//! from 1975-01-01 onward. It does not generalize to other legal age thresholds
//! or assert legal capacity under separate provisions.

use crate::Date;
use chrono::NaiveDate;
use std::error::Error;
use std::fmt;

pub const MAJORITY_RULE_ID: &str = "de.bgb.2.majority";
pub const MAJORITY_SOURCE_URL: &str = "https://www.gesetze-im-internet.de/bgb/__2.html";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Majority {
    Minor,
    Adult,
}

impl fmt::Display for Majority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Minor => write!(f, "minor"),
            Self::Adult => write!(f, "adult"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvaluationError {
    AssessmentBeforeBirth,
    UnsupportedHistoricalDate,
}

impl fmt::Display for EvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AssessmentBeforeBirth => write!(f, "assessment date precedes the date of birth"),
            Self::UnsupportedHistoricalDate => {
                write!(f, "dates before 1975-01-01 are not supported")
            }
        }
    }
}

impl Error for EvaluationError {}

/// Classify a person's age of majority under § 2 BGB, using the supplied
/// birth date and assessment date. Does not look up or retain personal data.
///
/// For a person born on 29 February, majority begins on 1 March in the
/// non-leap year in which the person completes 18 years.
///
/// # Examples
///
/// ```
/// use de_law::Date;
/// use de_law::de::bgb::{majority_on, Majority};
///
/// let birth = "2008-11-10".parse::<Date>().unwrap();
/// let today = "2026-11-10".parse::<Date>().unwrap();
/// assert_eq!(majority_on(birth, today), Ok(Majority::Adult));
/// ```
pub fn majority_on(birth: Date, assessment: Date) -> Result<Majority, EvaluationError> {
    if assessment < birth {
        return Err(EvaluationError::AssessmentBeforeBirth);
    }
    if (assessment.year(), assessment.month(), assessment.day()) < (1975, 1, 1) {
        return Err(EvaluationError::UnsupportedHistoricalDate);
    }

    let threshold_year = birth.year() + 18;
    let (threshold_month, threshold_day) = if birth.month() == 2
        && birth.day() == 29
        && NaiveDate::from_ymd_opt(threshold_year, 2, 29).is_none()
    {
        (3, 1)
    } else {
        (birth.month(), birth.day())
    };

    let is_adult = (assessment.year(), assessment.month(), assessment.day())
        >= (threshold_year, threshold_month, threshold_day);

    Ok(if is_adult {
        Majority::Adult
    } else {
        Majority::Minor
    })
}
