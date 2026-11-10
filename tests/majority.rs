use de_law::Date;
use de_law::de::bgb::{EvaluationError, Majority, majority_on};

fn date(input: &str) -> Date {
    input.parse().unwrap()
}

#[test]
fn bgb_section_2_confers_majority_on_completion_of_18_years() {
    let born = date("2008-11-10");
    assert_eq!(majority_on(born, date("2026-11-09")), Ok(Majority::Minor));
    assert_eq!(majority_on(born, date("2026-11-10")), Ok(Majority::Adult));
    assert_eq!(majority_on(born, date("2026-11-11")), Ok(Majority::Adult));
}

#[test]
fn february_29_birthday_in_non_leap_year() {
    let born = date("2008-02-29");
    assert_eq!(majority_on(born, date("2026-02-28")), Ok(Majority::Minor));
    assert_eq!(majority_on(born, date("2026-03-01")), Ok(Majority::Adult));
}

#[test]
fn february_29_across_century() {
    // A leap-day birth in 2084 reaches age 18 in the non-leap year 2102.
    let born = date("2084-02-29");
    assert_eq!(majority_on(born, date("2102-02-28")), Ok(Majority::Minor));
    assert_eq!(majority_on(born, date("2102-03-01")), Ok(Majority::Adult));
}

#[test]
fn january_1975_threshold() {
    let born = date("1957-01-01");
    assert_eq!(majority_on(born, date("1975-01-01")), Ok(Majority::Adult));
    assert_eq!(
        majority_on(born, date("1974-12-31")),
        Err(EvaluationError::UnsupportedHistoricalDate)
    );
}

#[test]
fn errors_are_explicit() {
    assert_eq!(
        majority_on(date("2026-01-01"), date("2025-12-31")),
        Err(EvaluationError::AssessmentBeforeBirth)
    );
}
