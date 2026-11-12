# de-law-rs — v0.1.0

Experimental, **machine-native legal rules as ordinary Rust code**.

The maintained implementation is Rust source plus tests. Cargo compiles it;
other applications consume it as a library; formatted output is generated on
request. No custom legal language, parser, graph engine, registry, or runtime.

## First implemented rule

`de_law::de::bgb::majority_on` classifies a person as an adult or minor on a
given date under § 2 BGB (completion of 18 years), using § 187(2) and
§ 188(2)–(3) BGB for age calculation. It covers dates from **1975-01-01**,
when the age threshold of 18 took effect. Leap-day births and date validation
are tested. Other legal age thresholds and legal capacity are **not** modeled.

The reusable `de_law::has_reached_age` helper checks whether a date of birth
has reached a specified number of whole calendar years on an assessment date.
It is a date calculation only; callers remain responsible for determining
which legal age threshold applies.

Sources:

- [§ 2 BGB](https://www.gesetze-im-internet.de/bgb/__2.html)
- [§ 187 BGB](https://www.gesetze-im-internet.de/bgb/__187.html)
- [§ 188 BGB](https://www.gesetze-im-internet.de/bgb/__188.html)

This is an independent software implementation, **not an official legal
publication or authoritative legal decision**. The linked legal sources remain
available independently; no copy of the original documents is bundled.

## Use

Requires Rust 1.85+ (edition 2024).

```sh
cargo run -- majority --born 2008-11-10 --on 2026-11-10
cargo run --example consumer
cargo test --workspace --all-targets
cargo test --workspace --doc
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

The CLI prints a generated result:

```text
rule=de.bgb.2.majority
born=2008-11-10
on=2026-11-10
status=adult
source=https://www.gesetze-im-internet.de/bgb/__2.html
```

## HTTP service

`http/` is a separate workspace package (`de-law-http`) that depends on the
`de-law` library through a local path dependency and uses only the standard
library for networking. It parses query parameters and renders JSON; all legal
classification, date validation and supported-date checks come from
`de_law::de::bgb::majority_on`. It is a local demonstration, not a hardened
production server (no TLS, authentication or rate limiting).

```sh
cargo run -p de-law-http                 # listens on 127.0.0.1:8080
curl -i 'http://127.0.0.1:8080/v1/majority?born=2008-11-10&on=2026-11-10'
```

```json
{"rule":"de.bgb.2.majority","born":"2008-11-10","on":"2026-11-10","status":"adult","source":"https://www.gesetze-im-internet.de/bgb/__2.html"}
```

Errors are JSON `{"error":"<code>","message":"..."}`:

| Status | `error` | Cause |
| --- | --- | --- |
| 400 | `missing_parameter` | `born` or `on` absent |
| 400 | `invalid_query` | unknown or duplicate query parameter |
| 400 | `invalid_date` | not a valid `YYYY-MM-DD` Gregorian date (years 0001–9999) |
| 422 | `assessment_before_birth` | `on` precedes `born` |
| 422 | `unsupported_date` | `on` before 1975-01-01; no legal conclusion is guessed |
| 404 / 405 | `not_found` / `method_not_allowed` | other path / non-GET method |

Legal coverage is exactly that of `majority_on` (see above): only the § 2 BGB
age-18 classification from 1975-01-01 onward.

### Reuse across consumers

`tests/common/cases.rs` holds the representative cases (day before and on the
18th birthday, leap-day birth, malformed dates, supported-date boundary) used by
`tests/shared_cases.rs` (library and CLI) and `http/tests/http.rs` (HTTP). To
see a library change reach both consumers without touching either, change the
age `18` in `majority_on` (`src/de/bgb.rs`) and run
`cargo test --workspace --all-targets`: the shared-case tests of the library,
CLI and HTTP service all fail together, and `cargo run -- majority ...` and
`curl` both report the new classification.

Downstream Rust project:

```toml
[dependencies]
de-law = { git = "https://github.com/YOUR-ORG/de-law-rs", tag = "v0.1.0" }
```

The Git URL is an example; **there is no repository or published tag yet**.
Before hosting, replace `YOUR-ORG` with the real organization, review the
files, and create the release tag.

## Structure

```text
src/date.rs         Validated Gregorian dates
src/de/bgb.rs       Native majority rule
src/main.rs         Transient CLI rendering
http/               Separate HTTP consumer of the library
examples/consumer.rs Example external-style consumer
tests/              Unit/integration/CLI tests
.github/            CI and dependency maintenance
GOVERNANCE.md       Maintainer workflow
CONTRIBUTING.md     Review and testing guidance
AGENTS.md           Coding agent expectations
LICENSE             Apache-2.0
```

## Design constraints

- Use existing language, toolchain, permissions, versioning, and delivery systems.
- Implement one verified rule at a time. No placeholder general-purpose legal IR.
- Keep legal documents external and inspect them only when adding or reviewing
  implementations.
- Keep build outputs, CLI reports and other projections transient.
- Treat unsupported inputs as errors, not invented answers.

See `GOVERNANCE.md` and `CONTRIBUTING.md` before contributing.

## Scheduled Copilot Automations

For eligible private or internal repositories with Copilot cloud agent and
automations enabled by the organization, a maintainer with write access can
create a recurring automation from the repository's **Agents** tab. Choose
**Automations**, create a personal or shared automation, write a bounded prompt,
select a schedule (for example, weekly), and grant only the tools needed for
that task. Availability and organization policies may vary; see the
[Copilot automations documentation](https://docs.github.com/en/copilot/concepts/agent-capabilities/agent-powered-workflows-and-operations/about-automations)
for current requirements and setup details. Automation configuration is stored
separately from this repository.

For example, a weekly issue-triage automation could ask Copilot to identify open
development issues that lack acceptance criteria or test requirements and
summarize the gaps for a maintainer. Keep prompts and tools narrowly scoped;
review any proposed changes yourself. Do not grant or configure automatic
merging, release publishing, or production deployment.
