default:
    @just --list

check:
    cargo fmt --all -- --check
    cargo check --all-targets
    cargo clippy --all-targets --all-features -- -D warnings
    cargo test --all-targets

fetch law="gg":
    cargo run -- fetch {{law}}

inspect law="gg":
    cargo run -- inspect {{law}}

verify law="gg":
    cargo run -- verify {{law}}
