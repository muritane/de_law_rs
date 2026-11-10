---
name: code-review
description: Review a proposed change for correctness, scope, tests, and source-backed legal behavior in de-law-rs.
---

# Code review

Review the change against the issue and repository guidance in `AGENTS.md`,
`CONTRIBUTING.md`, and `GOVERNANCE.md`.

- Check that the change is limited to the requested scope and preserves
  existing behavior outside it. Prefer direct Rust and existing project
  conventions over new infrastructure.
- For executable legal behavior, check that the cited public source is linked
  in relevant documentation, the implementation makes its interpretation
  explicit, and unsupported cases are not silently treated as supported.
  Flag unverifiable or fabricated legal claims; do not supply legal rules or
  citations from guesswork.
- Check that changed behavior has regression tests for relevant ordinary,
  boundary, failure, and exceptional cases. Ensure tests do not encode a legal
  claim without a source or make production behavior pass by weakening
  assertions.
- Check that generated outputs and duplicate copies of source documents are
  not being added, and that no automatic merge, release, or deployment behavior
  is introduced.
- Run the repository's formatting, Clippy, and test commands when available.
  Report results and any checks not run.

Report only actionable findings, explain their impact, and identify the
relevant file and location. Separate a confirmed issue from a question or
uncertainty. If there are no findings, state that and summarize the checks
performed.
