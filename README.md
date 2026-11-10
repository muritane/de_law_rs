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
cargo test --all-targets
cargo test --doc
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
```

The CLI prints a generated result:

```text
rule=de.bgb.2.majority
born=2008-11-10
on=2026-11-10
status=adult
source=https://www.gesetze-im-internet.de/bgb/__2.html
```

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
