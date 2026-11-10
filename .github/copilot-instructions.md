# Repository instructions

`de-law-rs` maintains narrowly scoped German legal rules as ordinary Rust code.
The maintained artifacts are source code and tests; generated output is
transient.

- Prefer standard Rust, Cargo, Git, and GitHub features. Keep changes small,
  independently testable, and reversible.
- Do not add a custom DSL, parser, build engine, legal database, or orchestration
  framework without a concrete requirement.
- Before implementing or reviewing legal behavior, verify the applicable public
  legal source. Never invent legal rules or citations. Keep links in Rust doc
  comments and human-facing documentation, not duplicated as runtime data.
- Make legal interpretations explicit when they affect executable behavior.
  State the rule's scope and unsupported cases; do not claim comprehensive
  coverage or change behavior merely to satisfy tests.
- For behavior changes, add regression tests for normal cases, boundaries,
  failures, and known exceptions as applicable. Use synthetic data.
- Do not add generated artifacts or duplicate source representations. Do not
  introduce automatic merging, release publishing, or production deployment.
- Before proposing changes, run `cargo fmt --all -- --check`,
  `cargo clippy --all-targets -- -D warnings`, `cargo test --all-targets`, and
  `cargo test --doc` when the toolchain is available. Report checks that could
  not be run.

Use the repository skills for focused tasks:

- For a new narrow executable legal rule, follow
  `.github/skills/legal-rule-development/SKILL.md`. For example, when extending
  the majority calculation in `src/de/bgb.rs`, follow that skill and use
  `tests/majority.rs` as a model for birthday, leap-day, and unsupported-date
  boundaries.
- For a change review, follow `.github/skills/code-review/SKILL.md`.

The bounded implementation profile is in
`.github/agents/maintainer.agent.md`. Contributor and governance guidance is in
`CONTRIBUTING.md` and `GOVERNANCE.md`.
