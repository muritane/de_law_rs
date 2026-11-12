#[path = "../../tests/common/cases.rs"]
mod cases;

use cases::{CASES, Outcome};
use de_law::de::bgb::majority_at;
use de_law::{Date, NaturalPerson};
use de_law_http::{Response, handle, serve};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

const PATH: &str = "/v1/majority/evaluate";

fn evaluate(born_on: &str, at: &str) -> Response {
    let body =
        format!(r#"{{"subject":{{"type":"natural_person","born_on":"{born_on}"}},"at":"{at}"}}"#);
    handle("POST", PATH, &body)
}

#[test]
fn shared_cases() {
    for &(born, on, expected) in CASES {
        let r = evaluate(born, on);
        let (status, marker) = match expected {
            Outcome::Minor => (200, r#""status":"minor""#),
            Outcome::Adult => (200, r#""status":"adult""#),
            Outcome::InvalidDate => (400, r#""error":"invalid_date""#),
            Outcome::BeforeBirth => (422, r#""error":"assessment_before_birth""#),
            Outcome::Unsupported => (422, r#""error":"unsupported_historical_date""#),
        };
        assert_eq!(r.status, status, "{born} {on}: {}", r.body);
        assert!(r.body.contains(marker), "{born} {on}: {}", r.body);
    }
}

#[test]
fn success_body_identifies_the_rule_and_assessment_date() {
    let r = evaluate("2008-11-10", "2026-11-10");
    assert_eq!(
        r.body,
        r#"{"rule":"de.bgb.2.majority","status":"adult","evaluated_at":"2026-11-10"}"#
    );
}

#[test]
fn http_follows_library_for_every_day_around_birthday() {
    let person = NaturalPerson::new("2008-02-29".parse().unwrap());
    for (m, d) in [(2, 27), (2, 28), (3, 1), (3, 2)] {
        let at = Date::new(2026, m, d).unwrap();
        let expected = majority_at(&person, at).unwrap().to_string();
        let response = evaluate(&person.born_on.to_string(), &at.to_string());
        assert!(response.body.contains(&format!(r#""status":"{expected}""#)));
    }
}

#[test]
fn future_scenarios_are_evaluated_against_the_current_rule() {
    let response = evaluate("1907-01-01", "2100-11-10");
    assert_eq!(response.status, 200);
    assert!(response.body.contains(r#""status":"adult""#));
}

#[test]
fn request_errors() {
    assert!(handle("POST", PATH, "").body.contains("invalid_request"));
    assert!(
        handle(
            "POST",
            PATH,
            r#"{"subject":{"type":"natural_person","born_on":"2008-02-30"},"at":"2026-11-10"}"#
        )
        .body
        .contains("invalid_date")
    );
    assert!(
        handle(
            "POST",
            PATH,
            r#"{"subject":{"type":"company","born_on":"2008-11-10"},"at":"2026-11-10"}"#
        )
        .body
        .contains("invalid_request")
    );
    assert_eq!(handle("POST", "/v2/majority/evaluate", "{}").status, 404);
    assert_eq!(handle("GET", PATH, "").status, 405);
}

#[test]
fn serves_json_request_over_tcp() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    thread::spawn(move || serve(listener));

    let body = r#"{"subject":{"type":"natural_person","born_on":"2008-11-10"},"at":"2026-11-09"}"#;
    let mut stream = TcpStream::connect(addr).unwrap();
    write!(
        stream,
        "POST {PATH} HTTP/1.1\r\nHost: localhost\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{body}",
        body.len()
    )
    .unwrap();
    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();
    assert!(response.starts_with("HTTP/1.1 200 OK"), "{response}");
    assert!(response.contains("Content-Type: application/json"));
    assert!(
        response.ends_with(
            r#"{"rule":"de.bgb.2.majority","status":"minor","evaluated_at":"2026-11-09"}"#
        )
    );
}
