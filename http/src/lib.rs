//! Minimal HTTP adapter for `de_law::de::bgb::majority_at`.
//!
//! The adapter only parses transport input and renders JSON. All legal
//! classification, date validation and supported-date checks come from the
//! `de-law` library; nothing is reimplemented here.

#![forbid(unsafe_code)]

use de_law::de::bgb::{EvaluationError, MAJORITY_RULE_ID, majority_at};
use de_law::{Date, NaturalPerson};
use serde::Deserialize;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

const MAX_REQUEST_HEAD: usize = 8 * 1024;
const MAX_REQUEST_BODY: usize = 8 * 1024;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EvaluationRequest {
    subject: Subject,
    at: String,
}

#[derive(Deserialize)]
#[serde(tag = "type", deny_unknown_fields)]
enum Subject {
    #[serde(rename = "natural_person")]
    NaturalPerson { born_on: String },
}

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

/// Handle `POST /v1/majority/evaluate` with a natural-person JSON subject.
///
/// Errors: 400 `invalid_request`, `invalid_date`; 422
/// `assessment_before_birth`, `unsupported_historical_date`; 404; 405.
pub fn handle(method: &str, target: &str, body: &str) -> Response {
    if target != "/v1/majority/evaluate" {
        return error(404, "not_found", "unknown path");
    }
    if method != "POST" {
        return error(405, "method_not_allowed", "only POST is supported");
    }

    let request: EvaluationRequest = match serde_json::from_str(body) {
        Ok(request) => request,
        Err(_) => {
            return error(
                400,
                "invalid_request",
                "expected a natural-person subject and an assessment date",
            );
        }
    };

    let Subject::NaturalPerson { born_on } = request.subject;
    let (Ok(born_on), Ok(at)) = (born_on.parse::<Date>(), request.at.parse::<Date>()) else {
        return error(
            400,
            "invalid_date",
            "dates must be valid calendar dates in YYYY-MM-DD format",
        );
    };
    let person = NaturalPerson::new(born_on);

    match majority_at(&person, at) {
        Ok(status) => Response {
            status: 200,
            body: format!(
                r#"{{"rule":"{MAJORITY_RULE_ID}","status":"{status}","evaluated_at":"{at}"}}"#
            ),
        },
        Err(EvaluationError::AssessmentBeforeBirth) => error(
            422,
            "assessment_before_birth",
            "assessment date precedes the date of birth",
        ),
        Err(EvaluationError::UnsupportedHistoricalDate) => error(
            422,
            "unsupported_historical_date",
            "majority evaluations before 1975-01-01 are not supported",
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

fn read_request(stream: &mut TcpStream) -> Option<(String, String, String)> {
    let mut buf = Vec::new();
    let mut chunk = [0u8; 1024];
    let head_end = loop {
        if buf.len() > MAX_REQUEST_HEAD {
            return None;
        }
        match stream.read(&mut chunk) {
            Ok(0) | Err(_) => return None,
            Ok(n) => {
                buf.extend_from_slice(&chunk[..n]);
                if let Some(end) = buf.windows(4).position(|w| w == b"\r\n\r\n") {
                    break end + 4;
                }
            }
        }
    };
    if head_end > MAX_REQUEST_HEAD {
        return None;
    }

    let head = std::str::from_utf8(&buf[..head_end]).ok()?;
    let mut lines = head.lines();
    let mut request_line = lines.next()?.split_ascii_whitespace();
    let (method, target, version) = (
        request_line.next()?.to_owned(),
        request_line.next()?.to_owned(),
        request_line.next()?,
    );
    if !version.starts_with("HTTP/") || request_line.next().is_some() {
        return None;
    }

    let mut content_length = None;
    for line in lines {
        if let Some((name, value)) = line.split_once(':') {
            if name.eq_ignore_ascii_case("transfer-encoding") {
                return None;
            }
            if name.eq_ignore_ascii_case("content-length") {
                if content_length.is_some() {
                    return None;
                }
                content_length = Some(value.trim().parse::<usize>().ok()?);
            }
        }
    }
    let content_length = content_length.unwrap_or(0);
    if content_length > MAX_REQUEST_BODY {
        return None;
    }
    let body_end = head_end.checked_add(content_length)?;
    while buf.len() < body_end {
        let remaining = body_end - buf.len();
        let chunk_size = remaining.min(chunk.len());
        let n = stream.read(&mut chunk[..chunk_size]).ok()?;
        if n == 0 {
            return None;
        }
        buf.extend_from_slice(&chunk[..n]);
    }

    Some((
        method,
        target,
        std::str::from_utf8(&buf[head_end..body_end])
            .ok()?
            .to_owned(),
    ))
}

fn serve_connection(mut stream: TcpStream) {
    let _ = stream.set_read_timeout(Some(Duration::from_secs(5)));
    let response = match read_request(&mut stream) {
        Some((method, target, body)) => handle(&method, &target, &body),
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
