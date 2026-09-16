use std::{
    fs,
    io::{Cursor, Read},
    path::{Path, PathBuf},
    time::Duration,
};

use anyhow::{Context, Result, bail, ensure};
use chrono::{SecondsFormat, Utc};
use reqwest::blocking::Client;
use zip::ZipArchive;

use crate::{
    catalog::find_law,
    hash::sha256_hex,
    snapshot::{SnapshotInfo, SnapshotMetadata, SourceMetadata},
    xml::{inspect, parse_document_metadata, referenced_dtd_url},
};

pub const CATALOG_URL: &str = "https://www.gesetze-im-internet.de/gii-toc.xml";
const PROVIDER: &str = "Gesetze im Internet";

pub fn fetch_snapshot(repo_root: &Path, law_slug: &str) -> Result<SnapshotMetadata> {
    validate_slug(law_slug)?;

    let reference_root = reference_root(repo_root);
    let law_dir = law_dir(repo_root, law_slug);
    let client = http_client()?;

    println!("fetching catalog: {CATALOG_URL}");
    let catalog_bytes = get_bytes(&client, CATALOG_URL)?;
    let catalog_xml =
        std::str::from_utf8(&catalog_bytes).context("Gesetze-im-Internet catalog is not UTF-8")?;

    let item = find_law(catalog_xml, law_slug)?;
    println!("found: {}", item.title);
    println!("archive: {}", item.link);

    let archive_bytes = get_bytes(&client, &item.link)
        .with_context(|| format!("downloading XML archive for '{law_slug}'"))?;
    let archive_sha256 = sha256_hex(&archive_bytes);

    let (xml_filename, xml_bytes) = extract_single_xml(&archive_bytes)?;
    let xml_sha256 = sha256_hex(&xml_bytes);
    let xml = std::str::from_utf8(&xml_bytes).context("source XML is not UTF-8")?;

    // Parsing here is intentional: fetch only commits a snapshot after the XML
    // is at least well-formed enough for our inspector and metadata reader.
    let _ = inspect(xml)?;
    let document = parse_document_metadata(xml, law_slug, &xml_filename)?;

    let dtd_url = referenced_dtd_url(xml)?;
    println!("DTD: {dtd_url}");
    let dtd_bytes = get_bytes(&client, &dtd_url).context("downloading referenced DTD")?;
    let dtd_sha256 = sha256_hex(&dtd_bytes);

    let metadata = SnapshotMetadata {
        source: SourceMetadata {
            provider: PROVIDER.to_owned(),
            catalog_url: CATALOG_URL.to_owned(),
            catalog_title: item.title,
            archive_url: item.link,
            dtd_url: dtd_url.clone(),
        },
        document,
        snapshot: SnapshotInfo {
            retrieved_at: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
            archive_sha256,
            xml_sha256,
            dtd_sha256,
            modified: false,
        },
    };

    // Only write after all downloads and parsing have succeeded.
    fs::create_dir_all(&law_dir).with_context(|| format!("creating {}", law_dir.display()))?;

    write_bytes(&reference_root.join("gii-toc.xml"), &catalog_bytes)?;
    write_bytes(&law_dir.join("source.zip"), &archive_bytes)?;
    write_bytes(&law_dir.join("source.xml"), &xml_bytes)?;

    let dtd_path = dtd_destination(&reference_root, &dtd_url)?;
    write_bytes(&dtd_path, &dtd_bytes)?;

    let toml = toml::to_string_pretty(&metadata).context("serializing source.toml")?;
    write_bytes(&law_dir.join("source.toml"), toml.as_bytes())?;

    Ok(metadata)
}

pub fn verify_snapshot(repo_root: &Path, law_slug: &str) -> Result<SnapshotMetadata> {
    validate_slug(law_slug)?;

    let reference_root = reference_root(repo_root);
    let law_dir = law_dir(repo_root, law_slug);

    let metadata_path = law_dir.join("source.toml");
    let metadata_text = fs::read_to_string(&metadata_path)
        .with_context(|| format!("reading {}", metadata_path.display()))?;
    let metadata: SnapshotMetadata =
        toml::from_str(&metadata_text).context("parsing source.toml")?;

    ensure!(
        !metadata.snapshot.modified,
        "source.toml says the extracted source was modified"
    );
    ensure!(
        metadata.document.law_slug == law_slug,
        "source.toml belongs to law slug '{}', not '{law_slug}'",
        metadata.document.law_slug
    );

    let archive_path = law_dir.join("source.zip");
    let archive_bytes =
        fs::read(&archive_path).with_context(|| format!("reading {}", archive_path.display()))?;
    verify_hash("archive", &metadata.snapshot.archive_sha256, &archive_bytes)?;

    let xml_path = law_dir.join("source.xml");
    let xml_bytes =
        fs::read(&xml_path).with_context(|| format!("reading {}", xml_path.display()))?;
    verify_hash("XML", &metadata.snapshot.xml_sha256, &xml_bytes)?;

    // Prove that source.xml is exactly the XML extracted from source.zip.
    let (archived_name, archived_xml) = extract_single_xml(&archive_bytes)?;
    ensure!(
        archived_name == metadata.document.xml_filename,
        "archive XML filename changed: metadata has '{}', archive has '{}'",
        metadata.document.xml_filename,
        archived_name
    );
    ensure!(
        archived_xml == xml_bytes,
        "source.xml is not byte-for-byte identical to the XML in source.zip"
    );

    let dtd_path = dtd_destination(&reference_root, &metadata.source.dtd_url)?;
    let dtd_bytes =
        fs::read(&dtd_path).with_context(|| format!("reading {}", dtd_path.display()))?;
    verify_hash("DTD", &metadata.snapshot.dtd_sha256, &dtd_bytes)?;

    let xml = std::str::from_utf8(&xml_bytes).context("source XML is not UTF-8")?;
    let _inspection = inspect(xml)?;
    let parsed = parse_document_metadata(xml, law_slug, &archived_name)?;
    ensure!(
        parsed == metadata.document,
        "metadata parsed from source.xml does not match source.toml"
    );

    Ok(metadata)
}

pub fn reference_root(repo_root: &Path) -> PathBuf {
    repo_root.join("reference").join("gesetze-im-internet")
}

pub fn law_dir(repo_root: &Path, law_slug: &str) -> PathBuf {
    reference_root(repo_root).join(law_slug)
}

pub fn dtd_destination(reference_root: &Path, dtd_url: &str) -> Result<PathBuf> {
    let marker = "/dtd/";
    let start = dtd_url
        .find(marker)
        .context("DTD URL does not contain a /dtd/ path")?;
    let relative = &dtd_url[start + 1..];

    ensure!(
        !relative.split('/').any(|component| component == ".."),
        "DTD URL contains an unsafe path"
    );

    Ok(reference_root.join(relative))
}

fn http_client() -> Result<Client> {
    Client::builder()
        .user_agent(concat!("de-law-rs/", env!("CARGO_PKG_VERSION")))
        .timeout(Duration::from_secs(60))
        .build()
        .context("building HTTP client")
}

fn get_bytes(client: &Client, url: &str) -> Result<Vec<u8>> {
    let response = client
        .get(url)
        .send()
        .with_context(|| format!("GET {url}"))?
        .error_for_status()
        .with_context(|| format!("GET {url} returned an error status"))?;

    Ok(response
        .bytes()
        .with_context(|| format!("reading response body from {url}"))?
        .to_vec())
}

fn extract_single_xml(archive_bytes: &[u8]) -> Result<(String, Vec<u8>)> {
    let cursor = Cursor::new(archive_bytes);
    let mut archive = ZipArchive::new(cursor).context("opening downloaded ZIP archive")?;
    let mut xml_entries = Vec::new();

    for index in 0..archive.len() {
        let file = archive.by_index(index).context("reading ZIP directory")?;
        if !file.is_dir() && file.name().to_ascii_lowercase().ends_with(".xml") {
            xml_entries.push((index, file.name().to_owned()));
        }
    }

    if xml_entries.is_empty() {
        bail!("downloaded ZIP contains no XML file");
    }
    if xml_entries.len() != 1 {
        let names = xml_entries
            .iter()
            .map(|(_, name)| name.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        bail!("expected exactly one XML file in archive, found: {names}");
    }

    let (index, name) = xml_entries.remove(0);
    let mut file = archive
        .by_index(index)
        .with_context(|| format!("opening '{name}' in ZIP archive"))?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)
        .with_context(|| format!("extracting '{name}'"))?;

    Ok((name, bytes))
}

fn verify_hash(label: &str, expected: &str, bytes: &[u8]) -> Result<()> {
    let actual = sha256_hex(bytes);
    ensure!(
        actual == expected,
        "{label} SHA-256 mismatch: expected {expected}, got {actual}"
    );
    Ok(())
}

fn write_bytes(path: &Path, bytes: &[u8]) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("creating {}", parent.display()))?;
    }
    fs::write(path, bytes).with_context(|| format!("writing {}", path.display()))
}

fn validate_slug(law_slug: &str) -> Result<()> {
    ensure!(!law_slug.is_empty(), "law slug cannot be empty");
    ensure!(
        law_slug
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_'),
        "law slug may contain only ASCII letters, digits, '-' and '_'"
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_versioned_gii_dtd_to_reference_tree() {
        let root = Path::new("reference/gesetze-im-internet");
        let result = dtd_destination(
            root,
            "https://www.gesetze-im-internet.de/dtd/1.01/gii-norm.dtd",
        )
        .unwrap();
        assert_eq!(
            result,
            Path::new("reference/gesetze-im-internet/dtd/1.01/gii-norm.dtd")
        );
    }
}
