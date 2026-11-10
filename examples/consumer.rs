//! Example of an independent application consuming the library.
//! Run: cargo run --example consumer

use de_law::Date;
use de_law::de::bgb::{Majority, majority_on};

fn main() {
    let born: Date = "2008-11-10".parse().expect("valid example birth date");
    let on: Date = "2026-11-10".parse().expect("valid example assessment date");
    let status = majority_on(born, on).expect("supported assessment");
    assert_eq!(status, Majority::Adult);
    println!("{status}");
}
