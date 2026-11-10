# Coding agent instructions

- Prefer direct Rust code over frameworks or legal-specific infrastructure.
- Do not introduce XML ASTs, legal knowledge graphs, new package managers,
  custom permission systems, or messaging infrastructure without an explicit
  requirement and a failing test that needs them.
- Never fabricate legal rules or source references.
- Keep source citations in doc comments and human-facing documentation;
  do not duplicate external legal documents as runtime data.
- For changed behavior add a regression test, including a boundary case.
- Treat Git history, Cargo builds, and CLI output as separate concerns.
- Run `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`,
  `cargo test --all-targets`, and `cargo test --doc` where tooling is available.
- Clearly disclose any step that was not executed or verified.
