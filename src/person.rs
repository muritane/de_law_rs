use crate::Date;
use std::error::Error;
use std::fmt;

/// A natural person represented by the supplied date of birth.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NaturalPerson {
    /// The person's supplied date of birth.
    pub born_on: Date,
}

impl NaturalPerson {
    pub const fn new(born_on: Date) -> Self {
        Self { born_on }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AgeError {
    AssessmentBeforeBirth,
}

impl fmt::Display for AgeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AssessmentBeforeBirth => write!(f, "assessment date precedes the date of birth"),
        }
    }
}

impl Error for AgeError {}

/// Calculate a natural person's completed calendar years at a date.
///
/// A person born on 29 February reaches the next age on 1 March in a
/// non-leap year, following the calculation of periods under §§ 187(2) and
/// 188(2)–(3) BGB:
/// <https://www.gesetze-im-internet.de/bgb/__187.html>,
/// <https://www.gesetze-im-internet.de/bgb/__188.html>.
///
/// # Errors
///
/// Returns [`AgeError::AssessmentBeforeBirth`] if `at` precedes the date of
/// birth.
pub fn age_at(person: &NaturalPerson, at: Date) -> Result<u32, AgeError> {
    if at < person.born_on {
        return Err(AgeError::AssessmentBeforeBirth);
    }

    let years = (at.year() - person.born_on.year()) as u32;
    Ok(if reaches_anniversary(person.born_on, at, years) {
        years
    } else {
        years - 1
    })
}

/// Check whether a natural person has reached a number of whole calendar years.
pub fn has_reached_age(person: &NaturalPerson, at: Date, threshold: u32) -> Result<bool, AgeError> {
    Ok(age_at(person, at)? >= threshold)
}

fn reaches_anniversary(born_on: Date, at: Date, years: u32) -> bool {
    let Ok(years) = i32::try_from(years) else {
        return false;
    };
    let Some(year) = born_on.year().checked_add(years) else {
        return false;
    };
    let anniversary = match Date::new(year, born_on.month(), born_on.day()) {
        Ok(date) => date,
        Err(_) if born_on.month() == 2 && born_on.day() == 29 => {
            let Ok(date) = Date::new(year, 3, 1) else {
                return false;
            };
            date
        }
        Err(_) => return false,
    };

    at >= anniversary
}
