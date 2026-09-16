# Bootstrap source tooling

## Requirements

- Rust stable via rustup
- Git
- Optional: `just`

No `curl` or `unzip` command is required; HTTP, ZIP extraction, hashing and XML parsing are done in Rust.

## First run

```bash
cargo test
cargo run -- fetch gg
cargo run -- verify gg
cargo run -- inspect gg
```

With `just` installed, the equivalent convenience commands are:

```bash
just check
just fetch gg
just verify gg
just inspect gg
```

## Files created by `fetch gg`

```text
reference/
└── gesetze-im-internet/
    ├── gii-toc.xml
    ├── dtd/
    │   └── 1.01/
    │       └── gii-norm.dtd
    └── gg/
        ├── source.zip
        ├── source.xml
        └── source.toml
```

`source.toml` contains the catalog title and URLs, retrieval timestamp, original XML filename,
document metadata, and SHA-256 values for the downloaded archive, extracted XML and DTD.

## What each command means

`fetch gg` is the only networked operation. It fetches the current catalog, discovers the GG XML archive from the catalog, downloads it, requires exactly one XML file in the archive, parses it, discovers its referenced DTD, downloads that DTD, hashes everything, and then writes the snapshot.

`verify gg` is offline. It verifies all stored hashes, re-extracts the ZIP in memory and proves that `source.xml` is byte-for-byte the extracted XML, reparses document metadata, and parses the XML structure.

`inspect gg` is offline. It prints root metadata, the DOCTYPE, counts for XML elements/attributes and the first `enbez` values. It is intentionally an inspector rather than a legal-domain parser.

## Important naming detail

`gg` is currently a Gesetze-im-Internet URL/path slug, not yet a typed legal identifier or `jurabk`. Keeping those concepts separate avoids accidentally treating an external website identifier as a fundamental legal-domain identifier.
