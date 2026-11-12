//! Minimal HTTP adapter for `de_law::de::bgb::majority_on`.
//!
//! The adapter only parses transport input and renders JSON. All legal
//! classification, date validation and supported-date checks come from the
//! `de-law` library; nothing is reimplemented here.

#![forbid(unsafe_code)]

use de_law::Date;
use de_law::de::bgb::{EvaluationError, MAJORITY_RULE_ID, MAJORITY_SOURCE_URL, majority_on};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

const MAX_REQUEST_HEAD: usize = 8 * 1024;

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    pub status: u16,
    pub body: String,
}

fn error(status: u16, code: &str, message: &str) -> Response {
    Response {
        status,
        body: format!(r#"{{"error":"{code}","message":"{message}"}}"#),
    }
}

/// Handle `GET /v1/majority?born=YYYY-MM-DD&on=YYYY-MM-DD`.
///
/// Errors: 400 `missing_parameter`, `invalid_date`, `invalid_query`;
/// 422 `assessment_before_birth`, `unsupported_date`; 404; 405.
pub fn handle(method: &str, target: &str) -> Response {
    let (path, query) = target.split_once('?').unwrap_or((target, ""));
    if path != "/v1/majority" {
        return error(404, "not_found", "unknown path");
    }
    if method != "GET" {
        return error(405, "method_not_allowed", "only GET is supported");
    }

    let (mut born, mut on) = (None, None);
    for pair in query.split('&').filter(|p| !p.is_empty()) {
        let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
        let slot = match key {
            "born" => &mut born,
            "on" => &mut on,
            _ => return error(400, "invalid_query", "unknown query parameter"),
        };
        if slot.replace(value).is_some() {
            return error(400, "invalid_query", "duplicate query parameter");
        }
    }
    let (Some(born), Some(on)) = (born, on) else {
        return error(
            400,
            "missing_parameter",
            "both `born` and `on` are required",
        );
    };

    let (Ok(born), Ok(on)) = (born.parse::<Date>(), on.parse::<Date>()) else {
        return error(
            400,
            "invalid_date",
            "dates must be valid calendar dates in YYYY-MM-DD format",
        );
    };

    match majority_on(born, on) {
        Ok(status) => Response {
            status: 200,
            body: format!(
                r#"{{"rule":"{MAJORITY_RULE_ID}","born":"{born}","on":"{on}","status":"{status}","source":"{MAJORITY_SOURCE_URL}"}}"#
            ),
        },
        Err(EvaluationError::AssessmentBeforeBirth) => error(
            422,
            "assessment_before_birth",
            "assessment date precedes the date of birth",
        ),
        Err(EvaluationError::UnsupportedHistoricalDate) => error(
            422,
            "unsupported_date",
            "dates before 1975-01-01 are not supported",
        ),
    }
}

fn reason(status: u16) -> &'static str {
    match status {
        200 => "OK",
        400 => "Bad Request",
        404 => "Not Found",
        405 => "Method Not Allowed",
        422 => "Unprocessable Content",
        _ => "Error",
    }
}

fn read_request_line(stream: &mut TcpStream) -> Option<(String, String)> {
    let mut buf = Vec::new();
    let mut chunk = [0u8; 1024];
    while !buf.windows(4).any(|w| w == b"\r\n\r\n") {
        if buf.len() > MAX_REQUEST_HEAD {
            return None;
        }
        match stream.read(&mut chunk) {
            Ok(0) | Err(_) => return None,
            Ok(n) => buf.extend_from_slice(&chunk[..n]),
        }
    }
    let head = String::from_utf8(buf).ok()?;
    let mut parts = head.lines().next()?.split(' ');
    let (method, target) = (parts.next()?, parts.next()?);
    parts.next()?.starts_with("HTTP/").then_some(())?;
    Some((method.to_owned(), target.to_owned()))
}

fn serve_connection(mut stream: TcpStream) {
    let _ = stream.set_read_timeout(Some(Duration::from_secs(5)));
    let response = match read_request_line(&mut stream) {
        Some((method, target)) => handle(&method, &target),
        None => error(400, "bad_request", "malformed HTTP request"),
    };
    let head = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        response.status,
        reason(response.status),
        response.body.len()
    );
    let _ = stream.write_all(head.as_bytes());
    let _ = stream.write_all(response.body.as_bytes());
}

/// Serve connections forever, one thread per connection.
pub fn serve(listener: TcpListener) {
    for stream in listener.incoming().flatten() {
        thread::spawn(move || serve_connection(stream));
    }
}
