# Contributing

Use normal GitHub pull requests. Keep each change small enough to review.

For a new implemented rule:

1. Link the publicly available source in a Rust doc comment.
2. Encode the behavior in native Rust, without adding a custom DSL or parser.
3. Add tests for boundaries, failures and any known exceptional cases.
4. Run `cargo fmt --all`, `cargo clippy --all-targets -- -D warnings`,
   and `cargo test --all-targets` before proposing a merge.
5. Explain any unsupported cases in the documentation instead of making the
   implementation silently guess.

Do not commit `target/` or generated CLI output. The code and tests are the
maintained artifacts; rendered results are disposable.
