use de_law::{AgeError, Date, NaturalPerson, age_at, has_reached_age};

fn date(input: &str) -> Date {
    input.parse().unwrap()
}

fn person(born_on: &str) -> NaturalPerson {
    NaturalPerson::new(date(born_on))
}

#[test]
fn age_is_derived_for_the_same_person_at_multiple_dates() {
    let person = person("2010-06-15");
    assert_eq!(age_at(&person, date("2024-06-14")), Ok(13));
    assert_eq!(age_at(&person, date("2024-06-15")), Ok(14));
}

#[test]
fn supports_different_age_thresholds() {
    let young_person = person("2010-06-15");
    assert_eq!(
        has_reached_age(&young_person, date("2024-06-14"), 14),
        Ok(false)
    );
    assert_eq!(
        has_reached_age(&young_person, date("2024-06-15"), 14),
        Ok(true)
    );
    let child = person("2020-06-15");
    assert_eq!(has_reached_age(&child, date("2026-06-15"), 6), Ok(true));
    let older_person = person("1961-06-15");
    assert_eq!(
        has_reached_age(&older_person, date("2026-06-15"), 65),
        Ok(true)
    );
}

#[test]
fn leap_day_age_begins_on_march_first_in_a_non_leap_year() {
    let person = person("2008-02-29");
    assert_eq!(age_at(&person, date("2022-02-28")), Ok(13));
    assert_eq!(age_at(&person, date("2022-03-01")), Ok(14));
}

#[test]
fn zero_age_and_unrepresentable_anniversaries() {
    let person = NaturalPerson::new(date("9999-12-31"));
    assert_eq!(age_at(&person, person.born_on), Ok(0));
    assert_eq!(has_reached_age(&person, person.born_on, 0), Ok(true));
    assert_eq!(has_reached_age(&person, person.born_on, 1), Ok(false));
    assert_eq!(
        has_reached_age(&person, person.born_on, u32::MAX),
        Ok(false)
    );
}

#[test]
fn assessment_before_birth_is_an_error() {
    assert_eq!(
        has_reached_age(&person("2026-01-02"), date("2026-01-01"), 0),
        Err(AgeError::AssessmentBeforeBirth)
    );
}
