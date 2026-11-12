//! Example of an independent application consuming the library.
//! Run: cargo run --example consumer

use de_law::Date;
use de_law::NaturalPerson;
use de_law::de::bgb::{Majority, majority_at};

fn main() {
    let person = NaturalPerson::new(
        "2008-11-10"
            .parse::<Date>()
            .expect("valid example birth date"),
    );
    let on: Date = "2026-11-10".parse().expect("valid example assessment date");
    let status = majority_at(&person, on).expect("supported assessment");
    assert_eq!(status, Majority::Adult);
    println!("{status}");
}
