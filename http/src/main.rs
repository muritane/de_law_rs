use std::env;
use std::net::TcpListener;
use std::process::ExitCode;

const USAGE: &str = "Usage: de-law-http [ADDRESS]   (default 127.0.0.1:8080)";

fn main() -> ExitCode {
    let addr = match env::args().nth(1).as_deref() {
        Some("--help" | "-h") => {
            println!("{USAGE}");
            return ExitCode::SUCCESS;
        }
        Some(addr) => addr.to_owned(),
        None => "127.0.0.1:8080".to_owned(),
    };
    match TcpListener::bind(&addr) {
        Ok(listener) => {
            eprintln!("listening on http://{addr}/v1/majority");
            de_law_http::serve(listener);
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("cannot bind {addr}: {e}\n{USAGE}");
            ExitCode::FAILURE
        }
    }
}
