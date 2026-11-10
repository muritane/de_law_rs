---
name: legal-rule-development
description: Implement or update a narrowly scoped, executable German legal rule in native Rust with verified sources and boundary tests.
---

# Legal rule development

Use this skill when an issue changes executable legal behavior.

1. Read `AGENTS.md`, `CONTRIBUTING.md`, the relevant Rust modules, and their
   tests. Keep the change within the issue's stated scope.
2. Verify the applicable rule using a public, authoritative source. Do not infer
   legal requirements from a test or invent a source citation. Link the source
   in Rust doc comments and human-facing documentation where relevant.
3. Implement the smallest useful behavior in ordinary, typed Rust. Reuse
   existing validated types and conventions. Represent unsupported inputs and
   failures explicitly; do not silently guess or add a general-purpose legal
   model, DSL, parser, or runtime.
4. Document the interpretation that determines behavior, the supported scope,
   and material exclusions. Do not claim that the project comprehensively
   implements a legal area.
5. Add regression tests for the ordinary case and relevant boundaries,
   invalid/failing inputs, and known exceptions. Use synthetic test data. Never
   alter legal behavior solely to make a test pass.
6. Run the Rust formatting, Clippy, and test commands in `AGENTS.md` when
   available, including documentation tests. Report any command that could not
   be run.

Example: for an issue about age of majority, inspect the cited BGB provisions
and `src/de/bgb.rs`, then use `tests/majority.rs` to follow the existing
before/on/after birthday and leap-day test pattern. Do not generalize that rule
to legal capacity or another age threshold unless the issue and verified
sources require it.
