use de_law::{AgeError, Date, has_reached_age};

fn date(input: &str) -> Date {
    input.parse().unwrap()
}

#[test]
fn supports_different_age_thresholds() {
    assert_eq!(
        has_reached_age(date("2010-06-15"), date("2024-06-14"), 14),
        Ok(false)
    );
    assert_eq!(
        has_reached_age(date("2010-06-15"), date("2024-06-15"), 14),
        Ok(true)
    );
    assert_eq!(
        has_reached_age(date("2020-06-15"), date("2026-06-15"), 6),
        Ok(true)
    );
    assert_eq!(
        has_reached_age(date("1961-06-15"), date("2026-06-15"), 65),
        Ok(true)
    );
}

#[test]
fn leap_day_age_begins_on_march_first_in_a_non_leap_year() {
    let born = date("2008-02-29");
    assert_eq!(has_reached_age(born, date("2022-02-28"), 14), Ok(false));
    assert_eq!(has_reached_age(born, date("2022-03-01"), 14), Ok(true));
}

#[test]
fn zero_age_and_unrepresentable_anniversaries() {
    let born = date("9999-12-31");
    assert_eq!(has_reached_age(born, born, 0), Ok(true));
    assert_eq!(has_reached_age(born, born, 1), Ok(false));
    assert_eq!(has_reached_age(born, born, u32::MAX), Ok(false));
}

#[test]
fn assessment_before_birth_is_an_error() {
    assert_eq!(
        has_reached_age(date("2026-01-02"), date("2026-01-01"), 0),
        Err(AgeError::AssessmentBeforeBirth)
    );
}
