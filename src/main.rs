//! Human-readable output is generated on demand; no result files are stored.

use de_law::Date;
use de_law::de::bgb::{MAJORITY_RULE_ID, MAJORITY_SOURCE_URL, majority_on};
use std::env;
use std::process::ExitCode;

const USAGE: &str = "Usage: de-law majority --born YYYY-MM-DD --on YYYY-MM-DD";

fn main() -> ExitCode {
    match run(env::args().skip(1)) {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}\n{USAGE}");
            ExitCode::FAILURE
        }
    }
}

fn run(mut args: impl Iterator<Item = String>) -> Result<(), String> {
    match args.next().as_deref() {
        Some("--help" | "-h") => {
            println!("{USAGE}");
            return Ok(());
        }
        Some("majority") => {}
        _ => return Err("expected the `majority` command".into()),
    }

    let (mut born, mut on) = (None, None);
    while let Some(flag) = args.next() {
        let value = args
            .next()
            .ok_or_else(|| format!("missing value for {flag}"))?;
        match flag.as_str() {
            "--born" if born.is_none() => born = Some(parse_date(&value)?),
            "--on" if on.is_none() => on = Some(parse_date(&value)?),
            _ => return Err(format!("unknown or duplicate argument: {flag}")),
        }
    }

    let born = born.ok_or("missing --born")?;
    let on = on.ok_or("missing --on")?;
    let status = majority_on(born, on).map_err(|e| e.to_string())?;

    println!("rule={MAJORITY_RULE_ID}");
    println!("born={born}");
    println!("on={on}");
    println!("status={status}");
    println!("source={MAJORITY_SOURCE_URL}");
    Ok(())
}

fn parse_date(s: &str) -> Result<Date, String> {
    s.parse().map_err(|e: de_law::DateError| e.to_string())
}
