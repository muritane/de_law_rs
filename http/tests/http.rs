#[path = "../../tests/common/cases.rs"]
mod cases;

use cases::{CASES, Outcome};
use de_law::Date;
use de_law::de::bgb::majority_on;
use de_law_http::{Response, handle, serve};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn get(born: &str, on: &str) -> Response {
    handle("GET", &format!("/v1/majority?born={born}&on={on}"))
}

#[test]
fn shared_cases() {
    for &(born, on, expected) in CASES {
        let r = get(born, on);
        let (status, marker) = match expected {
            Outcome::Minor => (200, r#""status":"minor""#),
            Outcome::Adult => (200, r#""status":"adult""#),
            Outcome::InvalidDate => (400, r#""error":"invalid_date""#),
            Outcome::BeforeBirth => (422, r#""error":"assessment_before_birth""#),
            Outcome::Unsupported => (422, r#""error":"unsupported_date""#),
        };
        assert_eq!(r.status, status, "{born} {on}: {}", r.body);
        assert!(r.body.contains(marker), "{born} {on}: {}", r.body);
    }
}

#[test]
fn success_body_has_rule_and_source() {
    let r = get("2008-11-10", "2026-11-10");
    assert_eq!(
        r.body,
        r#"{"rule":"de.bgb.2.majority","born":"2008-11-10","on":"2026-11-10","status":"adult","source":"https://www.gesetze-im-internet.de/bgb/__2.html"}"#
    );
}

#[test]
fn http_follows_library_for_every_day_around_birthday() {
    let born: Date = "2008-02-29".parse().unwrap();
    for (m, d) in [(2, 27), (2, 28), (3, 1), (3, 2)] {
        let on = Date::new(2026, m, d).unwrap();
        let expected = majority_on(born, on).unwrap().to_string();
        let r = get(&born.to_string(), &on.to_string());
        assert!(r.body.contains(&format!(r#""status":"{expected}""#)));
    }
}

#[test]
fn transport_errors() {
    assert_eq!(handle("GET", "/v1/majority").status, 400);
    assert!(
        handle("GET", "/v1/majority?born=2008-11-10")
            .body
            .contains("missing_parameter")
    );
    assert!(
        handle("GET", "/v1/majority?born=2008-11-10&on=2026-11-10&x=1")
            .body
            .contains("invalid_query")
    );
    assert!(
        handle(
            "GET",
            "/v1/majority?born=2008-11-10&born=2008-11-11&on=2026-11-10"
        )
        .body
        .contains("invalid_query")
    );
    assert_eq!(handle("GET", "/v2/majority").status, 404);
    assert_eq!(handle("POST", "/v1/majority").status, 405);
}

#[test]
fn serves_over_tcp() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    thread::spawn(move || serve(listener));

    let request = |line: &str| {
        let mut s = TcpStream::connect(addr).unwrap();
        write!(s, "{line}\r\nHost: localhost\r\n\r\n").unwrap();
        let mut out = String::new();
        s.read_to_string(&mut out).unwrap();
        out
    };
    let ok = request("GET /v1/majority?born=2008-11-10&on=2026-11-09 HTTP/1.1");
    assert!(ok.starts_with("HTTP/1.1 200 OK"), "{ok}");
    assert!(ok.contains("Content-Type: application/json"));
    assert!(ok.ends_with(
        r#""status":"minor","source":"https://www.gesetze-im-internet.de/bgb/__2.html"}"#
    ));
    let bad = request("GET /v1/majority?born=x&on=2026-11-09 HTTP/1.1");
    assert!(bad.starts_with("HTTP/1.1 400 Bad Request"), "{bad}");
    assert!(request("garbage").starts_with("HTTP/1.1 400"));
}
