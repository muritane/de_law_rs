use de_law::Date;
use de_law::NaturalPerson;
use de_law::de::bgb::{EvaluationError, Majority, majority_at};

fn date(input: &str) -> Date {
    input.parse().unwrap()
}

fn person(born_on: &str) -> NaturalPerson {
    NaturalPerson::new(date(born_on))
}

#[test]
fn ordinary_eighteenth_birthday() {
    let person = person("2008-11-10");
    assert_eq!(
        majority_at(&person, date("2026-11-09")),
        Ok(Majority::Minor)
    );
    assert_eq!(
        majority_at(&person, date("2026-11-10")),
        Ok(Majority::Adult)
    );
    assert_eq!(
        majority_at(&person, date("2026-11-11")),
        Ok(Majority::Adult)
    );
}

#[test]
fn february_29_birthday_in_non_leap_year() {
    let person = person("2008-02-29");
    assert_eq!(
        majority_at(&person, date("2026-02-28")),
        Ok(Majority::Minor)
    );
    assert_eq!(
        majority_at(&person, date("2026-03-01")),
        Ok(Majority::Adult)
    );
}

#[test]
fn february_29_across_century() {
    // A leap-day birth in 2084 reaches age 18 in the non-leap year 2102.
    let person = person("2084-02-29");
    assert_eq!(
        majority_at(&person, date("2102-02-28")),
        Ok(Majority::Minor)
    );
    assert_eq!(
        majority_at(&person, date("2102-03-01")),
        Ok(Majority::Adult)
    );
}

#[test]
fn january_1975_threshold() {
    let person = person("1957-01-01");
    assert_eq!(
        majority_at(&person, date("1975-01-01")),
        Ok(Majority::Adult)
    );
    assert_eq!(
        majority_at(&person, date("1974-12-31")),
        Err(EvaluationError::UnsupportedHistoricalDate)
    );
}

#[test]
fn errors_are_explicit() {
    assert_eq!(
        majority_at(&person("2026-01-01"), date("2025-12-31")),
        Err(EvaluationError::AssessmentBeforeBirth)
    );
}

#[test]
fn future_scenario_uses_the_installed_rule_version() {
    assert_eq!(
        majority_at(&person("1907-01-01"), date("2100-11-10")),
        Ok(Majority::Adult)
    );
}
