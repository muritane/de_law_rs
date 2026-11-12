mod common;

use common::cases::{CASES, Outcome};
use de_law::Date;
use de_law::de::bgb::{EvaluationError, Majority, majority_on};
use std::process::Command;

#[test]
fn library_matches_shared_cases() {
    for &(born, on, expected) in CASES {
        let result = born
            .parse::<Date>()
            .and_then(|b| on.parse::<Date>().map(|o| (b, o)));
        let actual = match result {
            Err(_) => Outcome::InvalidDate,
            Ok((b, o)) => match majority_on(b, o) {
                Ok(Majority::Minor) => Outcome::Minor,
                Ok(Majority::Adult) => Outcome::Adult,
                Err(EvaluationError::AssessmentBeforeBirth) => Outcome::BeforeBirth,
                Err(EvaluationError::UnsupportedHistoricalDate) => Outcome::Unsupported,
            },
        };
        assert_eq!(actual, expected, "{born} {on}");
    }
}

#[test]
fn cli_matches_shared_cases() {
    for &(born, on, expected) in CASES {
        let output = Command::new(env!("CARGO_BIN_EXE_de-law"))
            .args(["majority", "--born", born, "--on", on])
            .output()
            .expect("run the de-law CLI");
        let stdout = String::from_utf8(output.stdout).unwrap();
        let stderr = String::from_utf8(output.stderr).unwrap();
        let (success, text) = match expected {
            Outcome::Minor => (true, "status=minor"),
            Outcome::Adult => (true, "status=adult"),
            Outcome::InvalidDate => (false, "date"),
            Outcome::BeforeBirth => (false, "precedes the date of birth"),
            Outcome::Unsupported => (false, "before 1975-01-01"),
        };
        assert_eq!(output.status.success(), success, "{born} {on}: {stderr}");
        assert!(
            stdout.contains(text) || stderr.contains(text),
            "{born} {on}: {stdout}{stderr}"
        );
    }
}
