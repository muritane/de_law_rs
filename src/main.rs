use std::{fs, path::Path};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use de_law_rs::{
    fetch::{fetch_snapshot, law_dir, verify_snapshot},
    xml::{inspect, print_inspection},
};

#[derive(Debug, Parser)]
#[command(name = "de-law")]
#[command(about = "Source/provenance tooling for de-law-rs")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Fetch a law snapshot from Gesetze im Internet.
    Fetch {
        /// URL/path slug used by Gesetze im Internet, e.g. "gg".
        law: String,
    },

    /// Inspect the structure of an already-fetched source.xml.
    Inspect {
        /// URL/path slug used by Gesetze im Internet, e.g. "gg".
        law: String,
    },

    /// Verify checksums, archive extraction, metadata and XML parsing offline.
    Verify {
        /// URL/path slug used by Gesetze im Internet, e.g. "gg".
        law: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"));

    match cli.command {
        Command::Fetch { law } => {
            let metadata = fetch_snapshot(repo_root, &law)?;
            println!("\nsnapshot written successfully");
            println!("  law:       {}", metadata.document.law_slug);
            println!("  title:     {}", metadata.source.catalog_title);
            println!(
                "  doknr:     {}",
                metadata.document.doknr.as_deref().unwrap_or("<unknown>")
            );
            println!(
                "  builddate: {}",
                metadata
                    .document
                    .builddate
                    .as_deref()
                    .unwrap_or("<unknown>")
            );
            println!("  archive:   {}", metadata.snapshot.archive_sha256);
            println!("  XML:       {}", metadata.snapshot.xml_sha256);
            println!("  DTD:       {}", metadata.snapshot.dtd_sha256);
        }
        Command::Inspect { law } => {
            let path = law_dir(repo_root, &law).join("source.xml");
            let xml = fs::read_to_string(&path).with_context(|| {
                format!(
                    "reading {}; run `cargo run -- fetch {law}` first",
                    path.display()
                )
            })?;
            let inspection = inspect(&xml)?;
            print_inspection(&inspection);
        }
        Command::Verify { law } => {
            let metadata = verify_snapshot(repo_root, &law)?;
            println!("snapshot verified: {}", metadata.document.law_slug);
            println!("retrieved at: {}", metadata.snapshot.retrieved_at);
            println!("XML SHA-256: {}", metadata.snapshot.xml_sha256);
        }
    }

    Ok(())
}
