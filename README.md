# de-law-rs

A typed, navigable representation of German law in Rust.

## Project idea

The goal of this project is to represent German law as a structured Rust domain model from which familiar legal representations — especially the XML published by **Gesetze im Internet** — can be generated automatically.

The project is not primarily an exercise in translating legal prose into executable algorithms.

It is an attempt to make the underlying structure of law **load-bearing**.

Instead of maintaining several loosely coupled representations:

```text
law text
XML
documentation
cross-reference database
indexes
validation rules
developer notes
```

the intended direction is:

```text
                    Rust domain model
                           │
             ┌─────────────┼──────────────┐
             │             │              │
             ▼             ▼              ▼
            XML           HTML        plain text
             │
             ▼
     comparison against
       published XML
```

The law remains readable in its familiar form, but the mechanics behind that representation become explicit, typed, navigable and testable.

A useful analogy is a spreadsheet in which users still see the familiar shift plan, while names, calculations, validation, lookups and exports are no longer maintained manually in several unrelated places.

The visible artifact remains familiar.

The structure behind it becomes authoritative.

---

# Goals

The long-term goal is to make German law feel navigable from a developer environment in roughly the same way a well-structured software project is navigable.

For example:

- go to the definition of a referenced provision;
- find all references to a provision or defined term;
- inspect the structure of a law through Rust types;
- detect broken or malformed references;
- detect duplicate identifiers;
- validate structural invariants;
- distinguish different legal concepts through types instead of strings;
- generate XML automatically;
- compare generated XML with the externally published XML;
- generate alternative views without duplicating the law;
- inspect dependencies between laws and provisions;
- use ordinary Git diffs to understand changes to the modeled law;
- eventually query the corpus as a structured legal system rather than as disconnected documents.

The project should remain useful to one person working locally in VS Code. It does not initially need infrastructure for collaboration, web hosting, APIs, databases or production deployment.

---

# Core principle

The central design rule is:

> **Represent a fact about the law once, in the domain model, and derive other representations from it whenever practical.**

For example, avoid independently maintaining:

```text
§ number in Rust
§ number in XML template
§ number in documentation
§ number in reference table
```

Prefer:

```rust
struct Paragraph {
    id: ParagraphId,
    ...
}
```

and derive the other forms from `Paragraph::id`.

Similarly, a cross-reference should ideally not be only arbitrary text:

```rust
"§ 15 Absatz 2"
```

if the model can instead contain something like:

```rust
Reference<Paragraph>
```

whose displayed form is generated later.

The exact type system will evolve as the project encounters real German law.

The law should determine which distinctions are necessary.

---

# What is authoritative?

There are several different notions of "source of truth" and they should not be conflated.

## Legal authority

The project does **not** determine what German law is.

For federal legislation, promulgation in the **Bundesgesetzblatt** is legally authoritative.

## External machine-readable comparison source

For initial development, the main practical external source is:

**Gesetze im Internet**

It provides consolidated versions of German federal statutes and regulations in formats including:

- HTML
- PDF
- EPUB
- XML

The XML representation is specifically intended for automated processing.

Its XML structure is described by the published `gii-norm.dtd`.

Gesetze im Internet also publishes a daily XML table of contents.

## Internal project authority

Inside this repository, the intended authoritative representation is the Rust domain model.

Therefore:

```text
Bundesgesetzblatt
       │
       │ legal authority
       ▼
German law
       │
       │ consolidated external representation
       ▼
Gesetze im Internet XML
       │
       │ regression/comparison target
       ▼
┌───────────────────────────┐
│     Rust domain model     │
└───────────────────────────┘
       │
       │ deterministic rendering
       ▼
generated XML
```

The important comparison is:

```text
normalize(render(model))
    ==
normalize(reference_xml)
```

The published XML is therefore primarily an **oracle/regression target**, not the ontology from which the internal model must blindly inherit every implementation detail.

---

# Why not use the XML itself as the model?

The XML is an external serialization format.

Its structure may contain distinctions required for publishing, formatting, compatibility or document exchange that are not necessarily the best domain abstractions for working with law.

Therefore the initial architecture should distinguish:

```text
legal/domain structure
        ↓
document structure
        ↓
XML serialization
```

Sometimes these layers will map almost one-to-one.

Sometimes they will not.

The project should not introduce extra layers merely for architectural purity, but it should also not assume that an XML element automatically deserves to become a fundamental legal domain type.

---

# No parallel handwritten legal documentation

The project should avoid this:

```text
law.rs          ← one description
law.xml         ← manually maintained second description
law.md          ← manually maintained third description
references.csv  ← manually maintained fourth description
```

Those representations can silently diverge.

Instead:

```text
law.rs
  │
  ├──► XML
  ├──► text
  ├──► HTML
  ├──► reference index
  └──► documentation
```

Generated files should normally be recognizable as generated and should not be edited manually.

README files and architectural documentation are exceptions because they explain the project rather than encode individual legal provisions.

---

# Scope

## Initial scope

Start small enough that the complete model can be understood.

A reasonable first target is the **Grundgesetz**.

The initial project should focus primarily on:

1. document structure;
2. exact legal text;
3. stable identifiers;
4. hierarchy;
5. cross-references;
6. metadata required to reproduce the reference XML;
7. deterministic XML generation;
8. validation;
9. developer navigation.

The first milestone is **not** automated constitutional interpretation.

## Initial legal universe

Initially:

```text
current consolidated federal law
```

Not yet:

```text
all historical versions
all Länder law
EU law
court decisions
administrative rules
private standards
legal commentary
automated legal advice
```

These may become separate domains later.

---

# Non-goals

At least initially, this project is not intended to:

- replace the Bundesgesetzblatt;
- replace Gesetze im Internet;
- provide legal advice;
- determine the legally correct interpretation of disputed provisions;
- automatically decide constitutional questions;
- convert every sentence into a Boolean function;
- pretend natural-language law is completely executable;
- create a general legal AI system;
- model every German law at once;
- reproduce an XML schema internally merely for the sake of reproducing it;
- create a GUI before the underlying model is useful.

---

# Repository philosophy

The repository should remain understandable when opened directly in VS Code.

Prefer ordinary Rust mechanisms where they express the domain adequately.

Introduce abstraction only when repeated concrete legal structures show that it is useful.

In particular:

> Do not design an elaborate legal DSL before enough ordinary Rust exists to reveal what the DSL needs to remove.

---

# Possible workspace structure

An initial Cargo workspace might eventually look like:

```text
de-law-rs/
├── Cargo.toml
├── README.md
│
├── crates/
│   ├── legal-core/
│   │   └── src/
│   │
│   ├── legal-xml/
│   │   └── src/
│   │
│   ├── legal-check/
│   │   └── src/
│   │
│   └── legal-cli/
│       └── src/
│
├── laws/
│   └── gg/
│       └── src/
│
├── reference/
│   └── gesetze-im-internet/
│       └── gg/
│
└── tests/
```

This is only an initial direction.

A single crate is preferable until multiple crates solve an actual problem.

---

# Domain model

The domain model should be built incrementally from real law.

The first version may contain fairly structural concepts such as:

```rust
struct Law {
    metadata: LawMetadata,
    units: Vec<StructuralUnit>,
}

enum StructuralUnit {
    Part(Part),
    Chapter(Chapter),
    Article(Article),
    Paragraph(Paragraph),
}

struct Article {
    id: ArticleId,
    heading: Option<Heading>,
    content: Vec<Content>,
}
```

These types should not be treated as final.

Real statutes should be used to discover where the model is insufficient.

When the law forces a distinction, refine the model.

---

# Make distinctions explicit

Avoid excessive primitive values when the domain distinguishes their meanings.

Instead of:

```rust
u32
String
String
bool
```

consider types such as:

```rust
ArticleNumber
ParagraphNumber
ProvisionId
LawId
Heading
LegalText
Reference
EffectiveDate
```

when those distinctions actually provide value.

For example:

```rust
struct ArticleId(...);
struct ParagraphId(...);
```

can prevent accidental substitution even if both eventually contain similar primitive representations.

---

# Avoid Boolean collapse

A Boolean is appropriate when the domain really contains exactly two relevant states.

It should not silently collapse:

```text
true
false
unknown
not applicable
not yet determined
conflicting information
```

into:

```rust
bool
```

When necessary, use explicit enums:

```rust
enum Finding {
    Established,
    NotEstablished,
    Unknown,
}
```

The same principle applies throughout the project:

> If the law distinguishes two things, the model should not accidentally identify them merely because representing them separately requires more code.

---

# References should become relationships

A legal cross-reference has at least two aspects:

1. textual appearance;
2. relationship to another legal object.

The project should eventually be able to represent:

```text
Art. X refers to Art. Y
```

as an actual relationship.

For example, conceptually:

```rust
struct Reference<T> {
    target: Id<T>,
}
```

instead of only:

```rust
String
```

Rendering can later produce:

```text
Artikel 20 Absatz 3
```

or the corresponding XML.

This should eventually enable:

```text
Go to definition
Find all references
Dependency graphs
Broken-reference checks
```

Not every reference will necessarily be statically resolvable at Rust compile time. Project validation may therefore supplement the Rust type checker.

---

# Compile-time checks vs project validation

Rust should be used for constraints Rust can honestly enforce.

Examples:

## Good compile-time candidates

- incompatible identifier types;
- enum exhaustiveness;
- construction of impossible variants;
- required fields;
- generic constraints;
- API misuse;
- explicit error handling.

## Good validation-test candidates

- duplicate provision IDs;
- reference targets that do not exist in the loaded corpus;
- numbering inconsistencies;
- invalid effective-date relationships;
- XML structure constraints;
- generated output differences;
- corpus-wide dependency checks.

Do not force a runtime/corpus property into the type system merely to claim that it is "compile-time checked."

---

# Rendering

The model should support multiple generated representations.

Conceptually:

```text
              Domain Model
                   │
       ┌───────────┼────────────┐
       ▼           ▼            ▼
      XML         text         HTML
```

Formatting should therefore be separate from legal semantics.

For example:

```rust
trait XmlRender {
    fn render_xml(
        &self,
        writer: &mut XmlWriter,
    ) -> Result<(), XmlError>;
}
```

or serialization may be derived/generated when appropriate.

`Display` may be useful for convenient human-readable output, but it should not necessarily define the canonical XML representation.

---

# Why the fields should be load-bearing

Suppose an article is represented as:

```rust
struct Article {
    id: ArticleId,
    heading: Option<Heading>,
    paragraphs: Vec<Paragraph>,
}
```

Then XML rendering must obtain those values from those fields.

It should not separately contain:

```rust
write("<article id=\"1\">...");
```

with another independently maintained copy of the identifier.

Removing or changing a field should therefore have an observable effect on the generated representation.

This is intentional.

The structure should matter.

---

# Serialization strategy

Initially evaluate two approaches.

## Serde-based serialization

Serde can derive serialization implementations from Rust structs and enums.

This may provide a concise mapping where the XML structure matches the Rust structure naturally.

Potential advantage:

```rust
#[derive(Serialize)]
struct Something {
    ...
}
```

Potential problem:

The Gesetze-im-Internet XML format may contain structural details for which forcing the domain model to match Serde/XML conventions would distort the model.

## Explicit XML renderer

Use an XML writer such as `quick-xml` to render the domain model explicitly.

Potential advantage:

```text
domain model
    │
    ▼
XML adapter
    │
    ▼
exact external schema
```

The adapter controls serialization without making XML quirks fundamental domain concepts.

The XML adapter may contain formatting/schema knowledge.

It should not contain duplicated legal facts.

The correct choice can differ between structures. There is no requirement to use one technique everywhere.

---

# Official XML as a golden test

For each modeled law, keep an external reference snapshot.

Example:

```text
reference/
└── gesetze-im-internet/
    └── gg/
        ├── source.xml
        └── source.json
```

The metadata file might record:

```text
retrieval date
source identifier
published stand/version information
checksum
normalization rules
```

Then:

```rust
#[test]
fn generated_gg_matches_reference() {
    let model = gg::law();

    let generated =
        render_xml(&model).expect("rendering failed");

    let expected =
        load_reference_xml("gg");

    assert_eq!(
        normalize_xml(&generated),
        normalize_xml(&expected),
    );
}
```

This becomes one of the project's central invariants.

---

# Normalization

Byte-for-byte equality may not always represent meaningful equality.

Potential irrelevant differences can include:

- indentation;
- newline style;
- XML declaration details;
- attribute ordering where semantically irrelevant;
- insignificant whitespace.

Therefore comparison should use an explicitly defined normalization process.

However:

> Never normalize away a difference merely because it makes a failing test disappear.

Every normalization rule should have a documented reason.

Ideally provide both:

```text
semantic/normalized diff
raw diff
```

when debugging.

---

# Reference XML is a test oracle, not hidden input

The model should not secretly load the reference XML at runtime and then emit it again.

That would satisfy:

```text
generated XML == reference XML
```

without demonstrating that the Rust structure is load-bearing.

A useful rule is:

> The generated output must be reconstructible from the modeled law and renderer without reading the comparison fixture.

Fixtures exist to test the implementation.

They are not the implementation.

---

# Exact statutory text

The legal wording itself must obviously exist somewhere.

That is not "duplicated documentation."

It is domain content.

For example:

```rust
Sentence::new(
    "Die Würde des Menschen ist unantastbar."
)
```

contains actual legal content.

The distinction is:

```text
one stored legal text
→ many generated views
```

versus:

```text
independently maintained text in Rust
+ independently maintained XML text
+ independently maintained documentation text
```

Only the latter is unwanted duplication.

---

# Developer experience

Rust is used partly because the project should work well with normal developer tools.

With VS Code and rust-analyzer, the intended experience includes:

- completion;
- go to definition;
- find all references;
- type information;
- diagnostics;
- refactoring;
- code formatting;
- Clippy;
- symbol search;
- generated Rust documentation;
- normal Git integration.

The legal corpus should increasingly become explorable using these ordinary developer affordances.

---

# Rust learning philosophy

Rust learning is a consequence of building the legal model.

The project should not introduce Rust features merely to demonstrate Rust features.

Instead:

```text
legal modeling problem
        ↓
need a stronger representation
        ↓
learn the Rust mechanism that expresses it
```

Examples:

```text
different possible legal forms
→ enum

shared behavior
→ trait

possibly absent value
→ Option<T>

fallible transformation
→ Result<T, E>

different semantic IDs
→ newtypes

repeated safe construction syntax
→ macro

generic relationship
→ generics

state transition that should restrict operations
→ possibly typestate
```

This keeps the project law-first rather than Rust-first.

---

# Domain-specific language

A DSL is not an initial requirement.

Ordinary Rust should be used first.

For example:

```rust
Article {
    id: ArticleId::new(1),
    heading: None,
    paragraphs: vec![
        ...
    ],
}
```

If this becomes repetitive and obscures the domain, an embedded Rust macro might later allow:

```rust
article! {
    1 {
        ...
    }
}
```

The macro would still construct the same ordinary typed Rust domain objects.

Therefore:

```text
optional law-shaped DSL
          ↓
typed Rust domain model
          ↓
generated representations
```

The DSL is syntax convenience, not another source of truth.

---

# Suggested implementation phases

## Phase 0 — repository and tooling

Create the smallest possible Rust project.

Set up:

```text
cargo
rustfmt
clippy
rust-analyzer
Git
```

Do not design the entire architecture.

---

## Phase 1 — inspect one real source

Use the Grundgesetz as the first corpus.

Download its current XML representation from Gesetze im Internet.

Inspect:

- document hierarchy;
- metadata;
- identifiers;
- article representation;
- text representation;
- cross-references;
- special cases;
- the referenced DTD.

Do not immediately assume every XML distinction belongs in the domain model.

---

## Phase 2 — minimal structural model

Implement only enough Rust types to represent a small section faithfully.

For example:

```text
Law
Article
Paragraph/Absatz if required
Sentence/content
Heading
Identifier
Text
```

Manually construct perhaps one or two articles.

Render them.

Compare them with the corresponding XML fragment.

---

## Phase 3 — complete one law structurally

Extend the model until the complete Grundgesetz can be represented.

At this stage the primary objective is:

```text
complete structured law
→ deterministic XML
→ successful comparison
```

Not advanced semantic interpretation.

---

## Phase 4 — validation

Introduce explicit invariants as they become apparent.

Examples:

```text
IDs are unique
required fields exist
hierarchy is legal
numbering is coherent
references resolve where expected
rendering succeeds
generated XML matches fixture
```

Create useful error types rather than panicking where failure is a normal possibility.

---

## Phase 5 — typed references

Replace textual-only references with structured references where practical.

Enable:

```text
reference validation
reverse-reference lookup
dependency traversal
```

Do not discard the ability to reproduce the exact visible citation text where that is relevant.

---

## Phase 6 — developer views

Generate useful representations from the same model:

```text
plain legal text
HTML
rustdoc-like documentation
reference graphs
structural tree
search indexes
```

These are derived views.

They should not become independent representations of the law.

---

## Phase 7 — deeper legal semantics

Only after the structural model is stable, investigate whether recurring legal constructs justify explicit domain types such as:

```text
Definition
Tatbestand
Rechtsfolge
Ausnahme
Ermächtigung
Zuständigkeit
Frist
Verweisung
Inkrafttreten
```

Do not assume in advance that every statute decomposes cleanly into these categories.

Let the corpus determine how useful they are.

---

## Phase 8 — additional laws

Add another law specifically to challenge assumptions made while modeling the Grundgesetz.

A second corpus is important because:

> A model that perfectly represents one document may merely be an elaborate encoding of that document.

Repeated structures across multiple laws are stronger candidates for reusable abstractions.

---

# Editing workflow

The desired long-term editing loop is:

```text
edit Rust legal structure
        ↓
cargo check
        ↓
domain validation
        ↓
render XML
        ↓
compare against reference
        ↓
inspect diff
        ↓
commit
```

Eventually this could become:

```text
cargo law check
cargo law render gg
cargo law diff gg
cargo law refs GG::Art20
```

The exact CLI is not an initial priority.

---

# Source update workflow

When the external consolidated law changes:

```text
download new reference XML
        ↓
record provenance/checksum
        ↓
run comparison
        ↓
observe structural/textual differences
        ↓
modify Rust model
        ↓
checks pass again
        ↓
commit model + reference update
```

This creates a Git history that can eventually expose legal changes in a developer-readable form.

---

# Potential future Git workflow

A useful future convention may be:

```text
one legal source update
≈
one focused commit
```

Then Git history becomes another generated perspective on legal development.

Possible future tooling:

```text
show provisions changed between two commits
show references affected by a changed provision
show generated XML diff
show laws depending on a changed definition
```

---

# Source provenance

Every external snapshot should retain enough metadata to answer:

```text
Where did this come from?
When was it retrieved?
Which law/version did it represent?
What checksum did the downloaded artifact have?
Was it modified before storage?
```

Source metadata should be machine-readable where practical.

For example:

```rust
struct SourceSnapshot {
    source: Source,
    retrieved_at: DateTime,
    checksum: Checksum,
    declared_version: Option<String>,
}
```

The exact representation can remain simple initially.

---

# Primary external sources

## Gesetze im Internet

Primary practical source for consolidated current German federal laws and regulations.

Used for:

- XML reference output;
- comparison fixtures;
- structural investigation;
- consolidated text;
- metadata.

The service is provided by the Federal Ministry of Justice / Federal Office of Justice environment and provides German legal norms for free use and reuse.

Its XML format is specifically intended for automated processing.

## Gesetze-im-Internet DTD

The XML documents reference a public Document Type Definition:

```text
gii-norm.dtd
```

Use it to understand and validate the expected external XML document structure.

It is an external serialization specification.

It is not automatically the project's internal domain ontology.

## Gesetze-im-Internet XML table of contents

A daily XML table of contents is available.

This may later be useful for:

```text
discovering laws
tracking external updates
maintaining source metadata
automated comparison
```

It is not needed for the first implementation.

## Bundesgesetzblatt

The Bundesgesetzblatt remains important as the legally authoritative promulgation source.

Gesetze im Internet provides consolidated working versions; the two roles should remain distinct in the project documentation.

---

# Copyright and licensing

German statutory material requires some care in repository licensing because different material may have different legal status.

Under § 5 UrhG, German:

- statutes;
- regulations;
- official decrees and notices;
- court decisions;
- officially authored headnotes

do not enjoy copyright protection.

Gesetze im Internet also states that the German legal norms it provides are available for free use and reuse.

However, this should not be generalized to every external document associated with law.

In particular, private standards referenced by legislation may remain copyrighted.

Translations may also have separate usage conditions.

Therefore the repository should distinguish:

```text
project-authored source code
official statutory material
third-party material
```

rather than assuming one project license automatically governs everything.

A possible approach is:

```text
LICENSE
LICENSES/
NOTICE.md
```

with the main software license applying specifically to project-authored code and annotations.

The precise project license can be chosen later.

---

# Candidate Rust dependencies

Dependencies should be added only when they solve an immediate problem.

Likely candidates include:

## `quick-xml`

For XML reading/writing and possibly Serde-based XML serialization.

Useful both for:

```text
reference XML inspection
generated XML output
```

## `serde`

Potentially useful for derived serialization and for project metadata.

Whether the legal domain model itself should directly derive XML serialization should be determined experimentally rather than assumed.

## Error-handling utilities

A library such as `thiserror` may eventually be useful for explicit domain and rendering errors.

A CLI may use a more application-oriented error layer separately.

Exact dependency choices and versions should not be treated as architectural commitments.

---

# Generated outputs

Generated files should normally live outside the hand-authored domain representation.

For example:

```text
target/generated/
├── gg.xml
├── gg.txt
├── gg.html
└── refs.dot
```

or another clearly generated directory.

Generated files may optionally be committed if doing so materially improves diffing or archival use, but the repository should make clear that they are reproducible artifacts.

---

# Testing strategy

The project should gradually develop several kinds of tests.

## Unit tests

For small domain operations:

```text
identifier formatting
reference formatting
constructors
validation
normalization
```

## Structural tests

For corpus-wide invariants:

```text
unique IDs
valid hierarchy
resolved references
expected ordering
```

## Golden/reference tests

For generated external representations:

```text
Rust model
→ XML
→ normalized comparison
→ reference XML
```

## Regression tests

Whenever a bug is discovered, preserve a minimal example that would reproduce it.

---

# A useful rule for every abstraction

Whenever a new type, trait, macro or layer is proposed, ask:

> What repeated distinction in the actual law or workflow does this abstraction preserve?

If the answer is unclear, the abstraction may be premature.

The project is expected to evolve from concrete examples toward reusable structure rather than from an elaborate abstract schema toward the law.

---

# A useful rule for duplication

Ask:

> If I change this legal fact, how many places must I manually update?

The desired answer is normally:

```text
one
```

Everything else should follow through:

```text
types
references
rendering
validation
generated views
tests
```

or fail loudly.

---

# A useful rule for generated output

Ask:

> If I delete the reference XML fixture, can the Rust model still generate the complete law?

The desired answer is:

```text
yes
```

Then ask:

> If I restore the reference XML, can the project prove or show where the generated representation differs?

Again:

```text
yes
```

That is the intended relationship between mechanism and comparison target.

---

# Definition of done for the first milestone

The first substantial milestone can be considered complete when:

- the Grundgesetz is represented entirely through the project's Rust model;
- the exact required legal text exists only once in the load-bearing representation;
- XML can be generated without reading the reference fixture;
- generated XML matches the selected Gesetze-im-Internet XML after explicitly documented normalization;
- identifiers are structured rather than scattered arbitrary strings where practical;
- structural validation passes;
- `cargo check` passes;
- `cargo test` passes;
- `cargo clippy` is reasonably clean;
- navigating the project in VS Code is more useful than navigating the raw XML;
- there is no manually maintained second copy of the law merely for rendering.

---

# Current status

Exploratory solo project.

The architecture is intentionally not fixed yet.

The first objective is to discover a minimally sufficient Rust representation by trying to reproduce one real German law faithfully.

Expected development pattern:

```text
real law
  ↓
initial representation
  ↓
failure / awkwardness / duplication
  ↓
identify missing distinction
  ↓
refine types
  ↓
repeat
```

The evolving model is itself part of the investigation.

---

# Guiding principle

The project should preserve the familiar external form of law while replacing avoidable manual coupling underneath it.

In short:

```text
do not make humans synchronize representations
when structure can synchronize them instead.
```

Or more operationally:

```text
German law
    ↓
typed, navigable Rust structure
    ↓
validation + relationships + mechanics
    ↓
generated familiar legal representations
    ↓
comparison against external authoritative/reference sources
```

Rust is the medium.

The law is the domain.

The generated document is the view.

The structure underneath is what should carry the weight.
