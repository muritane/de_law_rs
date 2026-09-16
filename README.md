# de-law-rs

A typed, navigable and mechanically inspectable representation of German law in Rust.

## Project idea

German law is normally encountered as documents: statutes, articles, paragraphs, sentences, references and amendments expressed in natural language.

Those documents are not arbitrary prose.

They describe permissions, prohibitions, obligations, powers, conditions, exceptions, definitions, deadlines, dependencies and changes over time.

In that sense, legislation carves structure into the space of possible situations and actions.

This project investigates how much of that structure can be represented explicitly without pretending that natural-language law is completely executable.

The long-term direction is:

```text
authoritative / published legal sources
                │
                ▼
        source snapshots
                │
                ▼
        document structure
                │
                ▼
         legal structure
                │
                ▼
        semantic legal norms
                │
                ▼
        formal constraints
                │
                ▼
     mechanical inspection
```

Possible analyses include:

```text
Does this reference resolve?

Can these provisions apply together?

Does one provision imply another?

Are two modeled obligations incompatible?

Does an exception remove an apparent conflict?

Can all applicable constraints be satisfied simultaneously?

If not, which small set of modeled norms causes the contradiction?

Does sequencing make an otherwise impossible combination possible?

Does the result depend on an unresolved interpretation?
```

The project is therefore not primarily an XML project and not primarily an attempt to turn statutes into executable programs.

XML is an important external representation.

Rust is the implementation medium.

The deeper subject is the structure and interaction of legal norms.

---

# Central question

A useful long-term question for the project is:

> Given a modeled set of facts, interpretations, applicable norms, temporal assumptions and priority rules, what states or courses of action are compatible with the modeled legal constraints?

This is deliberately narrower than asking:

```text
What is the law?
```

or:

```text
Is this behavior legally correct?
```

The system should not claim to mechanically settle disputed legal interpretation.

Instead, it should eventually be able to make statements of the form:

```text
Under model M
and assumptions A,

constraints derived from
provisions X, Y and Z

have no joint satisfying assignment.
```

or:

```text
Under these assumptions,
the modeled norms admit the following compliant state.
```

The distinction matters.

A contradiction in the formal model is evidence about the model and its assumptions.

It is not automatically proof that the legal system itself is contradictory.

---

# Why this is interesting

A legal provision rarely exists in isolation.

Its operation may depend on:

```text
another provision
a definition
an exception
a permission
a prohibition
a competence rule
an effective date
an amendment
a factual condition
a procedural step
a priority rule
```

For example, consider an artificial system:

```text
Rule A:
If condition P holds,
person X must perform action Q.

Rule B:
If condition P holds,
person X must not perform action Q.
```

If `P` can occur and both rules apply simultaneously, there is no state in which X complies with both.

Conceptually:

```text
P → required(Q)

P → forbidden(Q)
```

Under `P`, the system becomes inconsistent unless some other legal rule resolves the conflict.

A more subtle example:

```text
Rule A:
X must happen within seven days.

Rule B:
X may not happen before approval Y.

Rule C:
Approval Y cannot occur before day ten.
```

Each rule may be individually satisfiable.

Their composition may not be.

This kind of interaction is one of the things the project should eventually make inspectable.

---

# Not every apparent conflict is a contradiction

Legal rules interact in more ways than simple Boolean consistency.

The model should eventually distinguish cases such as:

```text
satisfiable
unsatisfiable
temporally satisfiable
conditionally satisfiable
vacuously satisfiable
exception-resolved
priority-resolved
underspecified
interpretation-dependent
unknown to the model
```

For example:

```text
General rule:
X is prohibited.

Exception:
Authority A may permit X when condition C holds.

Fact:
A valid permission was granted.
```

A naïve constraint system might encode:

```text
not X
X
```

and report a contradiction.

A legal model may instead need to understand:

```text
general prohibition
        ↓
exception / authorization mechanism
        ↓
specific permission
```

The difficulty is therefore not merely solving constraints.

The difficult part is representing the legal semantics that should become constraints.

---

# Architectural principle

The project should distinguish several layers.

```text
┌─────────────────────────────┐
│       external sources      │
│  BGBl / Gesetze im Internet│
└──────────────┬──────────────┘
               │
               ▼
┌─────────────────────────────┐
│       source snapshots      │
│ provenance / checksum / XML │
└──────────────┬──────────────┘
               │
               ▼
┌─────────────────────────────┐
│       document model        │
│ XML structure / text layout │
└──────────────┬──────────────┘
               │
               ▼
┌─────────────────────────────┐
│       legal structure       │
│ provisions / references /  │
│ definitions / amendments   │
└──────────────┬──────────────┘
               │
               ▼
┌─────────────────────────────┐
│       semantic model        │
│ conditions / actors /       │
│ modalities / legal effects │
└──────────────┬──────────────┘
               │
               ▼
┌─────────────────────────────┐
│       constraint model      │
│ logical / temporal /        │
│ relational constraints     │
└──────────────┬──────────────┘
               │
               ▼
┌─────────────────────────────┐
│          analysis           │
│ SAT / UNSAT / implication / │
│ conflict cores / examples  │
└─────────────────────────────┘
```

These layers should not be collapsed merely because doing so produces less code.

At the same time, layers should only become concrete when they solve an observed problem.

---

# The XML is not the law model

The XML published by **Gesetze im Internet** is a structured document format.

Its elements describe an external representation designed for publication and automated processing.

For example, the same external `<norm>` concept can represent things with substantially different legal roles:

```text
document metadata
Eingangsformel
Präambel
Artikel
other structural units
```

Therefore:

```text
GII <norm>
```

should not automatically become:

```rust
struct LegalNorm { ... }
```

Those are different concepts.

A useful distinction is:

```text
GiiNorm
```

for an external document-format object, versus:

```text
LegalNorm
```

for a semantic proposition extracted or modeled from legal content.

The names should make that distinction difficult to forget.

---

# Source fidelity still matters

The semantic ambitions of the project make provenance more important, not less.

Every modeled fact should ideally remain traceable back to the source from which it was derived.

Conceptually:

```text
constraint
    │
    ▼
semantic norm
    │
    ▼
legal provision
    │
    ▼
source document location
    │
    ▼
source snapshot
```

For example, an analysis result should eventually be able to say:

```text
Unsatisfiable modeled constraints:

  GG::ArtX::Abs2
  SomeLaw::§Y::Abs4
  OtherLaw::§Z::Abs1

Assumptions:

  date = ...
  actor = ...
  condition C = true
  interpretation D = ...

Derived constraints:

  ...

No assignment satisfies all listed constraints.
```

The source path should be inspectable all the way back to the legal text.

---

# What is authoritative?

There are several different meanings of "source of truth".

They should not be conflated.

## Legal authority

This repository does not determine what German law is.

For federal legislation, promulgation in the **Bundesgesetzblatt** is legally authoritative.

The project is an analytical and representational system, not a source of legal authority.

## Consolidated external source

For practical development, **Gesetze im Internet** is useful because it provides consolidated federal statutes and regulations in formats including:

```text
HTML
PDF
EPUB
XML
```

The XML is specifically intended for automated processing.

Gesetze im Internet also publishes:

```text
gii-toc.xml
```

as a machine-readable catalog of the available laws and regulations.

The catalog contains links to downloadable XML archives.

The XML documents reference the public:

```text
gii-norm.dtd
```

which describes their external document structure.

## Internal project representations

Inside this repository there may eventually be several authoritative representations for different purposes.

For example:

```text
source snapshot
    authoritative for:
    "what external artifact did we retrieve?"

document model
    authoritative for:
    "how did we structurally interpret that artifact?"

legal model
    authoritative for:
    "what legal objects did we model?"

semantic model
    authoritative for:
    "what legal meaning did we explicitly encode?"

constraint model
    derived from semantic model
```

The project should avoid pretending that one representation answers all of these questions.

---

# Source discovery

The Gesetze-im-Internet catalog provides a useful machine-readable entry point:

```text
https://www.gesetze-im-internet.de/gii-toc.xml
```

Conceptually:

```text
gii-toc.xml
     │
     │ discover law
     ▼
<item>
  <title>...</title>
  <link>.../xml.zip</link>
</item>
     │
     ▼
download XML archive
     │
     ▼
extract source XML
     │
     ▼
record provenance + checksum
```

For the Grundgesetz, the catalog identifies the XML archive associated with the law.

The project should discover such URLs through the catalog rather than maintaining avoidable handwritten download mappings.

---

# Source snapshots

External source material should be preserved as snapshots.

For example:

```text
reference/
└── gesetze-im-internet/
    ├── gii-toc.xml
    ├── dtd/
    │   └── 1.01/
    │       └── gii-norm.dtd
    │
    └── gg/
        ├── source.xml
        └── source.toml
```

`source.toml` might record:

```toml
[source]
provider = "Gesetze im Internet"
catalog_url = "https://www.gesetze-im-internet.de/gii-toc.xml"
download_url = "..."

[document]
jurabk = "GG"
doknr = "BJNR000010949"
builddate = "..."
ausfertigung_datum = "1949-05-23"
stand = "..."

[snapshot]
retrieved_at = "..."
archive_sha256 = "..."
xml_sha256 = "..."
modified = false
```

Fields that can be parsed from the XML should eventually be generated rather than manually synchronized.

---

# Core design rule

A central rule remains:

> Represent a fact once at the most appropriate layer and derive other representations from it whenever practical.

Avoid:

```text
article number in legal model
article number in XML renderer
article number in semantic model
article number in reference database
```

Prefer relationships between typed representations.

For example:

```rust
struct Article {
    id: ArticleId,
    ...
}
```

and:

```rust
struct LegalNorm {
    source: ProvisionRef,
    ...
}
```

The semantic norm refers back to the legal structure instead of independently reproducing the source identifier.

Likewise, a constraint should retain provenance:

```rust
struct Constraint {
    source: SemanticNormId,
    ...
}
```

This allows generated analysis to remain traceable.

---

# Exact legal text

Legal wording must obviously exist somewhere.

That is domain content, not duplicated documentation.

For example:

```rust
LegalText::new(
    "Die Würde des Menschen ist unantastbar."
)
```

contains actual source material.

The undesirable situation is:

```text
text manually copied into structural model
+
text independently copied into semantic model
+
text independently copied into XML fixture
+
text independently copied into documentation
```

where these copies can silently diverge.

The preferred relationship is:

```text
source / modeled legal text
          │
          ├──► document rendering
          ├──► developer view
          ├──► semantic references
          └──► analysis provenance
```

---

# Structural model

The first internal model will probably still be primarily structural.

For example:

```rust
struct Law {
    id: LawId,
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

These types are not intended as a final ontology.

They provide stable objects to which semantic representations can later refer.

Real statutes should reveal which distinctions are necessary.

---

# Legal relationships

References should become relationships where practical.

Instead of representing only:

```text
"Artikel 20 Absatz 3"
```

the project should eventually support something conceptually like:

```rust
Reference<Provision>
```

or:

```rust
struct ProvisionRef {
    law: LawId,
    provision: ProvisionId,
}
```

This supports:

```text
go to definition
find references
reverse-reference lookup
dependency traversal
broken-reference detection
semantic dependency graphs
```

Visible citation text can still be preserved or generated when exact reproduction matters.

---

# Semantic model

A structural provision is not necessarily one semantic rule.

One sentence may express multiple legal propositions.

Several provisions may jointly define one operative rule.

The semantic layer therefore should not be forced into a one-to-one relationship with document elements.

A possible experimental representation might look like:

```rust
struct LegalNorm {
    id: LegalNormId,
    source: SourceSpan,
    applicability: Condition,
    effect: LegalEffect,
}
```

with effects such as:

```rust
enum LegalEffect {
    Obligation(Action),
    Prohibition(Action),
    Permission(Action),
    Definition(Definition),
    Competence(Competence),
}
```

This is intentionally incomplete.

Real legal material may require distinctions such as:

```text
Tatbestand
Rechtsfolge
Definition
Ausnahme
Ermächtigung
Zuständigkeit
Frist
Verweisung
Fiktion
Vermutung
Inkrafttreten
Außerkrafttreten
condition
actor
object
recipient
legal status
procedural step
```

These should be introduced because the corpus requires them, not because a comprehensive legal ontology sounds attractive.

---

# Conditions

Conditions will likely become a major part of the semantic layer.

Conceptually:

```rust
enum Condition {
    Fact(FactPredicate),
    All(Vec<Condition>),
    Any(Vec<Condition>),
    Not(Box<Condition>),
    Temporal(TemporalCondition),
    Reference(SemanticReference),
}
```

But such a representation should not be committed to before concrete examples reveal its requirements.

Natural-language applicability conditions can be difficult.

The model must be allowed to say:

```text
not modeled
unknown
ambiguous
interpretation-dependent
```

rather than silently choosing an interpretation.

---

# Modalities

Legal effects are not adequately represented by ordinary Boolean values.

At minimum, the project may need distinctions such as:

```rust
enum Modality {
    Required,
    Forbidden,
    Permitted,
}
```

Even this is likely incomplete.

Potential later distinctions include:

```text
power
competence
entitlement
immunity
liability
legal consequence
procedural possibility
constitutive rule
```

The project should resist reducing these to the same concept merely because a particular solver understands only Boolean formulas.

---

# Lowering semantics to constraints

The semantic model should not necessarily be identical to the solver representation.

Instead, use an explicit lowering step:

```text
legal semantic model
        │
        │ lower
        ▼
formal constraint representation
        │
        ▼
solver / analyzer
```

Conceptually:

```rust
fn lower_to_constraints(
    norm: &LegalNorm,
    context: &LegalContext,
) -> Result<ConstraintSet, LoweringError>;
```

This separation matters because the project may eventually experiment with different formalisms:

```text
SAT
SMT
Datalog
temporal logic
deontic logic
graph analysis
custom constraint propagation
```

German law should not be declared equivalent to whichever formalism happens to be used first.

---

# Satisfiability

One useful analysis is whether a modeled set of applicable constraints has any satisfying state.

Conceptually:

```text
constraints
    │
    ▼
 solver
  │   │
  │   └───────────────┐
  ▼                   ▼
 SAT                 UNSAT
  │                   │
  ▼                   ▼
model / example    conflict core
```

A `SAT` result may produce an example state compatible with the modeled constraints.

An `UNSAT` result should ideally produce or help identify a small conflicting set.

For example:

```text
UNSAT

Relevant modeled norms:

  N14 derived from § X Abs. 2
  N27 derived from § Y Abs. 4
  N31 derived from § Z Abs. 1

Assumptions:

  P = true
  date = ...
  actor = ...

Conflict:

  N14 requires Q
  N27 forbids Q
  N31 makes both simultaneously applicable
```

This is considerably more useful than:

```text
law inconsistent
```

---

# Unsatisfiable cores

For large corpora, detecting inconsistency is not enough.

If 20,000 constraints are loaded and the system returns:

```text
UNSAT
```

the result is not yet useful.

The analysis should attempt to identify a smaller subset sufficient for the contradiction.

Conceptually:

```text
all constraints
      │
      ▼
    UNSAT
      │
      ▼
unsatisfiable core
      │
      ▼
source provisions
```

The eventual developer experience might resemble:

```text
cargo law explain-conflict conflict-42
```

with output showing:

```text
source provisions
semantic norms
assumptions
lowered constraints
minimal or reduced conflicting set
```

---

# Time matters

Legal rules often change over time.

A system that ignores temporal applicability may report false conflicts.

For example:

```text
Rule A applies until 2025-12-31.

Rule B applies from 2026-01-01.
```

These are not simultaneous constraints.

Similarly:

```text
action required before T
action forbidden until T
```

may or may not be satisfiable depending on exact timing.

Therefore time is likely to become a first-class concern:

```text
effective dates
expiry dates
deadlines
events
ordering
duration
transition rules
```

Historical versioning does not need to be solved immediately, but the architecture should avoid making it impossible later.

---

# Sequence matters

Some legal processes consist of ordered states.

For example:

```text
application
    ↓
review
    ↓
approval
    ↓
action permitted
```

Two rules that appear contradictory in an atemporal representation may be compatible when sequencing is modeled.

Therefore:

```text
simultaneous satisfiability
```

and:

```text
existence of a valid sequence of states
```

are different questions.

This may eventually require transition-system or temporal reasoning rather than ordinary SAT alone.

---

# Definitions matter

Legal definitions influence many other provisions.

A definition should eventually be represented as something more structured than arbitrary text when doing so is justified.

For example:

```text
Definition D
      │
      ├── used by N1
      ├── used by N2
      └── used by N3
```

Changing the modeled interpretation of `D` may alter satisfiability elsewhere.

This allows analysis such as:

```text
which conclusions depend on this definition?

which norms become applicable if this concept is interpreted broadly?

which conflicts disappear under another admissible interpretation?
```

The system should report such dependencies rather than hiding them.

---

# Interpretations and assumptions

Formalization necessarily introduces assumptions.

Those assumptions should be explicit and inspectable.

For example:

```text
Assumption A1:
"unverzüglich" modeled here as deadline <= N days

Assumption A2:
definition D interpreted as ...

Assumption A3:
provisions X and Y both applicable

Assumption A4:
event E occurred before date T
```

The analysis result should depend visibly on those assumptions.

Ideally:

```text
result
  │
  ├── source norms
  ├── explicit facts
  ├── interpretation assumptions
  └── solver assumptions
```

This is important for legal honesty.

A hidden modeling choice should not masquerade as a conclusion of the statute.

---

# Unknown is a valid result

The project should not force every question into:

```text
true
false
```

Possible results may include:

```text
satisfied
violated
unknown
not applicable
underspecified
interpretation-dependent
unsupported by current model
```

Likewise, the model should distinguish:

```text
the constraints are satisfiable
```

from:

```text
the implementation was unable to determine satisfiability
```

and:

```text
the semantic representation is incomplete
```

These are different states.

---

# Compile-time checks vs validation vs formal analysis

Different mechanisms should solve different classes of problems.

## Rust type system

Good candidates include:

```text
incompatible identifier types
required fields
enum exhaustiveness
invalid API use
construction restrictions
generic constraints
```

## Project validation

Good candidates include:

```text
duplicate provision IDs
broken references
numbering inconsistencies
invalid hierarchy
missing source provenance
invalid effective-date relationships
```

## Formal analysis

Good candidates may include:

```text
joint satisfiability
logical implication
constraint redundancy
incompatible obligations
unreachable conditions
temporal impossibility
counterexample generation
```

Do not force runtime or corpus-wide properties into the type system merely to call them "compile-time checked".

Do not force semantic legal questions into the XML validator.

Use the appropriate mechanism for each layer.

---

# Generated representations

The internal models may generate familiar external representations:

```text
                  Legal model
                       │
          ┌────────────┼────────────┐
          ▼            ▼            ▼
         XML          text         HTML
```

and analytical representations:

```text
                Semantic model
                       │
          ┌────────────┼────────────┐
          ▼            ▼            ▼
    constraint IR   graph       diagnostics
          │
          ▼
       solver
```

Generated representations should remain clearly derived.

They should not become independently maintained copies of legal facts.

---

# XML round-tripping

XML reproduction remains a valuable test.

Conceptually:

```text
model
  │
  ▼
render XML
  │
  ▼
normalize
  │
  ▼
compare with reference snapshot
```

The comparison should help establish that the structural representation has not accidentally lost external document information.

However, XML equality does not prove semantic correctness.

Two separate questions must remain separate:

```text
Can we reproduce the source representation?
```

and:

```text
Have we modeled the legal meaning correctly?
```

Passing the first test does not imply the second.

---

# Normalization

Byte-for-byte XML equality may be too strict where differences are semantically irrelevant.

Potential normalization rules may include:

```text
newline normalization
indentation normalization
XML declaration handling
attribute ordering where irrelevant
insignificant whitespace
```

Every normalization rule should have a documented reason.

Never normalize away a difference merely because it makes a failing test disappear.

Ideally debugging can show both:

```text
raw diff
normalized diff
```

---

# Reference fixtures are not implementation input

The external XML fixture must not secretly become the generated output.

This would be meaningless:

```text
load reference XML
      ↓
return reference XML
      ↓
test says generated XML matches
```

A useful invariant is:

> The modeled representation must be able to generate its outputs without reading the comparison fixture.

Fixtures test the implementation.

They are not the implementation.

---

# Repository philosophy

The repository should remain understandable when opened directly in VS Code.

Prefer ordinary Rust mechanisms where they express the domain adequately.

Use:

```text
struct
enum
trait
Option<T>
Result<T, E>
newtypes
generics
```

before inventing custom language machinery.

Introduce abstraction after repeated concrete legal examples reveal what the abstraction preserves.

In particular:

> Do not design a comprehensive legal DSL before ordinary representations reveal what such a DSL would actually need to express.

---

# Possible initial repository

Start with one crate.

```text
de-law-rs/
├── Cargo.toml
├── Cargo.lock
├── rust-toolchain.toml
├── README.md
├── justfile
│
├── src/
│   ├── lib.rs
│   └── main.rs
│
├── examples/
│   └── inspect_xml.rs
│
├── tests/
│   └── reference_xml.rs
│
├── scripts/
│   ├── fetch-toc.sh
│   ├── fetch-law.sh
│   └── check.sh
│
└── reference/
    └── gesetze-im-internet/
        ├── gii-toc.xml
        ├── dtd/
        │   └── 1.01/
        │       └── gii-norm.dtd
        └── gg/
            ├── source.xml
            └── source.toml
```

Do not create a large workspace before actual code demonstrates that multiple crates improve the project.

---

# Possible later architecture

If boundaries become real, the repository may eventually separate concerns such as:

```text
crates/
├── legal-source/
├── legal-document/
├── legal-core/
├── legal-semantics/
├── legal-constraints/
├── legal-analysis/
├── legal-xml/
└── legal-cli/
```

These names are illustrative.

The project should not create these crates merely because this diagram exists.

---

# Initial tooling

The first repository should support:

```text
cargo check
cargo test
cargo fmt
cargo clippy
```

A small task runner such as `just` can provide convenience:

```text
just check
just fetch-law gg
just inspect-law gg
just test
```

A custom `cargo law` command can wait until there is enough behavior to justify it.

---

# Source acquisition workflow

The source pipeline should become reproducible early.

```text
download gii-toc.xml
        ↓
discover requested law
        ↓
read XML archive URL
        ↓
download archive
        ↓
record archive checksum
        ↓
extract XML
        ↓
record XML checksum
        ↓
parse source metadata
        ↓
store snapshot
```

For example:

```text
just fetch-law gg
```

should eventually be sufficient to update the Grundgesetz reference snapshot.

---

# XML inspection before modeling

Before building a detailed Rust model, inspect the real source.

A small tool should answer questions such as:

```text
What element names occur?

What attributes occur?

How many <norm> elements are present?

Which values appear in <enbez>?

How are articles represented?

How are headings represented?

How is text mixed with inline markup?

Where do references occur?

How are footnotes represented?

What does the DTD permit?

Which structures occur only once?

Which apparently optional structures are common?
```

Possible usage:

```text
cargo run -- inspect reference/gesetze-im-internet/gg/source.xml
```

The first goal is empirical understanding of the corpus.

---

# Structural implementation phase

After inspecting the XML, model a very small amount of law.

For example:

```text
Law
Article
Paragraph
Heading
LegalText
Identifier
Reference
```

Manually represent one or two provisions.

Render them.

Compare the result with the source fragment.

Allow the first model to be inadequate.

Its failures are evidence about which distinctions the project actually needs.

---

# Semantic research phase

Semantic experimentation should begin much earlier than full-corpus semantic modeling.

Use a handful of deliberately simple propositions.

For example:

```text
N1:
if P, X must do Q

N2:
if P, X must not do Q
```

Then encode:

```text
source
  ↓
semantic norm
  ↓
constraint
  ↓
solver
```

Verify that:

```text
without P:
possibly satisfiable

with P:
unsatisfiable
```

Then add synthetic cases for:

```text
obligation + obligation
obligation + prohibition
permission + prohibition
general rule + exception
different subjects
different dates
deadline interaction
definition dependency
nested conditions
cross-reference dependency
sequential rules
```

These synthetic tests allow the semantic architecture to be explored without making premature claims about real statutory interpretation.

---

# Real-law semantic experiments

After synthetic cases work, select unusually explicit legal provisions.

The purpose is not initially to formalize the whole Grundgesetz.

The purpose is to discover:

```text
what information disappears during formalization?

what distinctions are required?

what assumptions must be made?

what cannot be faithfully represented?

where does natural language resist decomposition?
```

Every such experiment should retain a direct source link to the modeled provision.

---

# Candidate semantic representation

A possible direction:

```rust
struct LegalNorm {
    id: LegalNormId,
    source: SourceSpan,
    applicability: Condition,
    effect: LegalEffect,
}
```

with:

```rust
enum LegalEffect {
    Obligation(Action),
    Prohibition(Action),
    Permission(Action),
    Definition(Definition),
    Competence(Competence),
}
```

and perhaps:

```rust
struct Action {
    actor: Actor,
    predicate: ActionPredicate,
    object: Option<LegalObject>,
}
```

This is exploratory syntax.

It is not a commitment to a universal model of law.

---

# Constraint representation

The constraint layer may initially use a small intermediate representation.

For example, conceptually:

```rust
enum Constraint {
    Requires(Fact),
    Forbids(Fact),
    Implies {
        if_: Condition,
        then: Box<Constraint>,
    },
    All(Vec<Constraint>),
    Any(Vec<Constraint>),
}
```

This representation may prove too weak almost immediately.

That is acceptable.

The objective is to discover the necessary distinctions through concrete failures.

---

# Solvers

Do not choose the architecture around a solver prematurely.

Potential tools may eventually include:

```text
SAT solver
SMT solver
Datalog engine
custom graph algorithms
temporal solver
constraint programming
```

An SMT solver may be especially useful when constraints involve:

```text
integers
dates
ordering
quantities
equality
logical implications
```

But a solver should remain downstream of the semantic representation.

The legal domain model should not become a collection of solver-specific expressions unless evidence shows that this is genuinely useful.

---

# Explanation is part of correctness

A solver result without traceability is not sufficient.

The system should prefer:

```text
UNSAT because:
N12, N18 and N31 cannot all hold
```

over:

```text
false
```

Likewise:

```text
SAT with example:
...
```

is often more useful than merely:

```text
true
```

The developer should be able to navigate:

```text
solver result
    ↓
constraint
    ↓
semantic rule
    ↓
provision
    ↓
source text
```

Explanation is not merely a UI feature.

For this project it is part of making analysis trustworthy.

---

# Testing strategy

The project should develop several classes of tests.

## Tooling tests

```text
project compiles
formatting passes
Clippy passes
```

## Source tests

```text
reference XML exists
XML is well-formed
checksum matches metadata
expected law identifier is present
DTD is available
```

## Structural tests

```text
IDs are unique
hierarchy is valid
references resolve
ordering is coherent
```

## Rendering tests

```text
modeled structure
→ generated XML
→ normalization
→ reference comparison
```

## Semantic unit tests

```text
conditions compose correctly
exceptions apply correctly
modality distinctions survive lowering
assumptions are preserved
```

## Constraint tests

Synthetic examples should verify expected:

```text
SAT
UNSAT
implication
non-implication
conflict cores
```

## Provenance tests

Every semantic rule used for real-law analysis should be traceable to:

```text
source provision
source snapshot
modeling assumptions
```

## Regression tests

Whenever a bug or modeling failure is discovered, preserve a minimal example reproducing it.

---

# Useful invariants

Several invariants should guide the project.

## Source independence

If the reference fixture is deleted:

```text
can the internal model still generate its derived representations?
```

Eventually the desired answer is:

```text
yes
```

## Traceability

For every derived formal constraint:

```text
can we identify why it exists and where it came from?
```

The desired answer is:

```text
yes
```

## Explicit assumptions

For every analysis result:

```text
can we identify the assumptions required to obtain it?
```

The desired answer is:

```text
yes
```

## No hidden duplication

Ask:

> If this legal fact changes, how many places must be manually updated?

Normally the desired answer is:

```text
one
```

Everything else should either update mechanically or fail loudly.

---

# Non-goals

At least initially, this project is not intended to:

```text
replace the Bundesgesetzblatt
replace Gesetze im Internet
provide legal advice
declare the legally correct interpretation of disputed language
automatically decide constitutional questions
predict court decisions
turn every sentence into executable code
assume every legal proposition has a Boolean interpretation
claim that SAT means lawful
claim that UNSAT means German law is contradictory
formalize all German law at once
model every historical version
build a general legal AI system
build a GUI before the underlying model is useful
```

The system analyzes explicitly modeled representations.

It does not eliminate legal interpretation.

---

# Initial legal universe

Start with:

```text
current consolidated federal law
```

and primarily:

```text
Grundgesetz
```

Not initially:

```text
all historical versions
all Länder law
EU law
court decisions
administrative rules
private standards
legal commentary
```

Those may later become related but separate domains.

---

# Why the Grundgesetz?

The Grundgesetz is a useful initial source because:

```text
it is finite
important
well known
structurally non-trivial
available in machine-readable form
rich in cross-references
different from ordinary statutes in useful ways
```

It should not be assumed to represent all patterns found elsewhere in German legislation.

A second law should later be selected specifically to challenge abstractions learned from the GG.

---

# Additional laws

A model that perfectly represents one statute may simply be an elaborate encoding of that statute.

Therefore:

```text
first law
    ↓
initial abstractions
    ↓
second structurally different law
    ↓
break assumptions
    ↓
refine abstractions
```

Repeated distinctions across unrelated laws are stronger candidates for reusable domain concepts.

---

# Suggested implementation phases

## Phase 0 — repository and tooling

Create the smallest usable Rust project.

Set up:

```text
Cargo
rustfmt
Clippy
rust-analyzer
Git
just or equivalent
```

Do not design the complete architecture.

---

## Phase 1 — reproducible source acquisition

Implement:

```text
fetch gii-toc.xml
discover law XML archive
download archive
extract XML
record checksums
record provenance
download referenced DTD
```

The process should be repeatable.

---

## Phase 2 — source inspection

Build a tiny XML inspector.

Learn:

```text
document hierarchy
metadata
identifiers
text representation
inline markup
footnotes
cross-references
special cases
DTD structure
```

Do not map every XML element directly into the legal domain.

---

## Phase 3 — minimal structural representation

Represent one or two provisions.

Introduce only the types required by actual examples.

Render them back into external forms where useful.

---

## Phase 4 — structural validation

Add checks for:

```text
duplicate IDs
missing references
invalid hierarchy
ordering
required metadata
```

---

## Phase 5 — source traceability

Ensure internal objects can identify their source locations.

For example:

```text
Article
  ↓
source norm
  ↓
document ID
  ↓
snapshot
```

This should exist before serious semantic analysis.

---

## Phase 6 — synthetic semantic experiments

Create a tiny semantic model independent of the full GG.

Test:

```text
conditions
obligations
prohibitions
permissions
exceptions
temporal distinctions
```

Lower them into constraints.

---

## Phase 7 — first satisfiability analysis

Connect a solver or small custom reasoning engine.

Support something as small as:

```text
SAT
UNSAT
example assignment
conflicting subset
```

Do not optimize for scale.

---

## Phase 8 — one real semantic example

Choose a deliberately simple real provision.

Model it manually.

Document every interpretation choice.

Trace every generated constraint to its source.

---

## Phase 9 — conflicting and interacting provisions

Find or construct examples involving:

```text
multiple provisions
references
exceptions
deadlines
definitions
```

Determine whether the semantic model can explain their interaction.

---

## Phase 10 — challenge the architecture

Add another law.

Expect existing assumptions to fail.

Refactor only when concrete repetition justifies it.

---

# Developer experience

The project should become navigable using normal Rust tools.

Desired capabilities include:

```text
completion
go to definition
find all references
type information
diagnostics
refactoring
rustdoc
symbol search
Git history
```

Eventually, legal navigation should feel increasingly like software navigation.

For example:

```text
GG::Art20
```

could expose:

```text
source text
document metadata
incoming references
outgoing references
semantic norms
derived constraints
analysis results
```

---

# Possible future CLI

Eventually:

```text
cargo law fetch gg
cargo law inspect gg
cargo law check gg
cargo law render gg
cargo law diff gg
cargo law refs GG::Art20
cargo law semantics GG::Art20
cargo law constraints GG::Art20
cargo law sat scenario.toml
cargo law explain conflict-42
```

The exact CLI is not an initial priority.

---

# Scenario-based analysis

Formal analysis will often require facts not contained in legislation itself.

A scenario may eventually provide those facts.

For example:

```toml
[scenario]
date = "..."
jurisdiction = "..."

[[facts]]
predicate = "..."
value = true

[[assumptions]]
id = "A1"
description = "..."
```

Then:

```text
law model
+
semantic rules
+
scenario facts
+
interpretation assumptions
        ↓
constraint system
        ↓
analysis
```

This cleanly separates:

```text
what the source says
```

from:

```text
what facts are assumed about the world
```

---

# Source updates

When the external consolidated source changes:

```text
fetch new source
      ↓
record provenance + checksum
      ↓
compare source snapshots
      ↓
identify changed provisions
      ↓
update structural model
      ↓
identify affected semantic models
      ↓
rerun analyses
      ↓
inspect changed results
```

Eventually this creates an interesting form of dependency tracking:

```text
changed statutory text
        ↓
affected legal structures
        ↓
affected semantic norms
        ↓
affected constraints
        ↓
affected conclusions
```

---

# Git as another view

A useful future convention may be:

```text
one external legal update
≈
one focused commit
```

This can make Git history another inspectable representation of legal change.

Potential tooling:

```text
show provisions changed between commits
show semantic rules affected
show broken references
show analyses whose result changed
show dependency paths from source change to conclusion
```

---

# Licensing and external material

The repository should distinguish between:

```text
project-authored code
official statutory material
project-authored semantic annotations
third-party material
```

Do not assume one license automatically applies to everything in the repository.

A possible layout:

```text
LICENSE
LICENSES/
NOTICE.md
```

Semantic annotations produced by the project should also be clearly distinguishable from official legal text.

---

# Candidate dependencies

Dependencies should be introduced only when they solve an immediate problem.

Likely early candidates:

## `quick-xml`

For:

```text
XML inspection
XML parsing
XML generation
```

## `thiserror`

For explicit domain and transformation errors.

## `serde`

Potentially useful for:

```text
source metadata
scenarios
configuration
intermediate formats
```

Whether core legal-domain types should derive Serde serialization should be determined experimentally.

## Solver libraries

Do not add one until the first semantic experiment actually needs it.

Possible future choices may include SAT or SMT libraries.

The solver choice should not determine the domain architecture.

---

# Errors should preserve meaning

Avoid turning all failures into strings.

Potential categories may eventually include:

```text
SourceError
ParseError
StructuralError
ReferenceError
SemanticModelError
LoweringError
UnsupportedConstruct
AnalysisError
```

Particularly important is distinguishing:

```text
contradiction in the modeled constraints
```

from:

```text
failure to construct the constraints
```

and from:

```text
construct not supported by the current model
```

---

# A useful rule for abstractions

Whenever a new type, trait, macro or layer is proposed, ask:

> What repeated distinction in the actual law, semantics or workflow does this abstraction preserve?

If the answer is unclear, the abstraction may be premature.

---

# A useful rule for formalization

Whenever a piece of natural-language law is converted into semantics, ask:

```text
What information did we preserve?

What information did we discard?

What interpretation did we choose?

What alternative interpretations remain possible?

Can the formal object be traced back to its source?
```

If these questions cannot be answered, the formalization should not silently become authoritative.

---

# A useful rule for solver results

Never report:

```text
the law is inconsistent
```

merely because a solver returns `UNSAT`.

Prefer:

```text
Under assumptions A,
the modeled constraints derived from provisions X, Y and Z
are jointly unsatisfiable.
```

Similarly, `SAT` does not mean:

```text
the behavior is legally valid
```

It means only that the current formal model admits at least one satisfying assignment.

---

# Definition of done for the first infrastructure milestone

The first infrastructure milestone is complete when:

```text
cargo check
cargo test
cargo clippy
```

pass and:

* the Gesetze-im-Internet catalog can be fetched;
* the GG XML archive can be discovered automatically;
* the XML can be downloaded and extracted;
* provenance is recorded;
* archive and XML checksums are recorded;
* the DTD is available;
* the GG source XML can be parsed;
* a basic corpus inspector can describe its structure;
* the process is reproducible locally.

No substantial legal ontology is required yet.

---

# Definition of done for the first structural milestone

The first structural milestone is complete when:

* a small part of the Grundgesetz is represented through typed Rust structures;
* exact source text is not unnecessarily duplicated;
* identifiers are typed where useful;
* source provenance remains accessible;
* generated XML or another external representation can be compared with the reference source;
* basic structural validation works;
* the model can evolve without depending on the comparison fixture at runtime.

---

# Definition of done for the first semantic milestone

The first semantic milestone is complete when:

* a few synthetic legal norms can be represented;
* conditions and at least basic modalities are explicit;
* semantic norms retain source/provenance information;
* those norms can be lowered into formal constraints;
* the system can distinguish at least `SAT` and `UNSAT`;
* a satisfying example can be inspected;
* an unsatisfiable case can identify the relevant modeled norms;
* assumptions remain explicit;
* tests demonstrate obligation/prohibition and exception cases.

---

# Definition of done for the first real-law analysis milestone

A first real-law analysis is complete when:

* at least one real provision is modeled semantically;
* every semantic assertion points back to its exact source;
* interpretation choices are documented as assumptions;
* generated constraints can be inspected;
* the analysis result can be reproduced;
* the result does not overclaim beyond the model;
* changing an assumption predictably changes the analysis where appropriate.

---

# Current status

Exploratory solo project.

The architecture is intentionally not fixed.

The project should evolve through repeated contact with real legal material:

```text
real source
    ↓
initial representation
    ↓
awkwardness / failure / ambiguity
    ↓
identify missing distinction
    ↓
refine model
    ↓
repeat
```

The same applies to semantics:

```text
legal proposition
    ↓
initial semantic model
    ↓
formalization failure
    ↓
identify lost distinction
    ↓
refine semantics
    ↓
repeat
```

Failures are useful evidence.

---

# Guiding principle

The project should make legal structure increasingly explicit without pretending that explicit structure eliminates legal interpretation.

In short:

```text
source text
    ↓
typed structure
    ↓
explicit semantics
    ↓
explicit assumptions
    ↓
formal constraints
    ↓
mechanically inspectable consequences
```

while always retaining the reverse path:

```text
analysis result
    ↓
constraint
    ↓
semantic norm
    ↓
legal provision
    ↓
source snapshot
```

The objective is not to make the machine the legal authority.

The objective is to make the consequences of an explicit legal model inspectable.

Rust is the medium.

Law is the domain.

Formalization is an experiment.

Traceability is mandatory.

And the interesting question is increasingly not only:

```text
Can we represent this law?
```

but also:

```text
What does this modeled set of legal constraints allow,
require,
forbid,
leave open,
or make impossible?
```
