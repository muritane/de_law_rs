//! Selected rules from the German Civil Code (BGB).
//!
//! Sources for the majority calculation:
//! - § 2 BGB (age of majority): https://www.gesetze-im-internet.de/bgb/__2.html
//! - § 187(2) BGB (counting the day of birth):
//!   https://www.gesetze-im-internet.de/bgb/__187.html
//! - § 188(2)–(3) BGB (period ending, including leap-day births):
//!   https://www.gesetze-im-internet.de/bgb/__188.html
//!
//! This module applies the current 18-year threshold, as applicable from
//! 1975-01-01 onward, using the generic calendar-age calculation. It does not
//! establish other legal age thresholds or assert legal capacity under
//! separate provisions.

use crate::{Date, NaturalPerson, has_reached_age};
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

/// Classify a natural person's age of majority under § 2 BGB.
///
/// The assessment uses the supplied birth date and the installed rule version.
/// It does not verify a person's identity, existence, or future legal status.
///
/// For a person born on 29 February, majority begins on 1 March in the
/// non-leap year in which the person completes 18 years.
///
/// # Examples
///
/// ```
/// use de_law::{Date, NaturalPerson};
/// use de_law::de::bgb::{majority_at, Majority};
///
/// let person = NaturalPerson::new("2008-11-10".parse::<Date>().unwrap());
/// let today = "2026-11-10".parse::<Date>().unwrap();
/// assert_eq!(majority_at(&person, today), Ok(Majority::Adult));
/// ```
pub fn majority_at(person: &NaturalPerson, assessment: Date) -> Result<Majority, EvaluationError> {
    if assessment < person.born_on {
        return Err(EvaluationError::AssessmentBeforeBirth);
    }
    if (assessment.year(), assessment.month(), assessment.day()) < (1975, 1, 1) {
        return Err(EvaluationError::UnsupportedHistoricalDate);
    }

    let is_adult = has_reached_age(person, assessment, 18)
        .map_err(|_| EvaluationError::AssessmentBeforeBirth)?;

    Ok(if is_adult {
        Majority::Adult
    } else {
        Majority::Minor
    })
}
