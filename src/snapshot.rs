use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SnapshotMetadata {
    pub source: SourceMetadata,
    pub document: DocumentMetadata,
    pub snapshot: SnapshotInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SourceMetadata {
    pub provider: String,
    pub catalog_url: String,
    pub catalog_title: String,
    pub archive_url: String,
    pub dtd_url: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct DocumentMetadata {
    pub law_slug: String,
    pub xml_filename: String,
    pub doknr: Option<String>,
    pub builddate: Option<String>,
    pub jurabk: Option<String>,
    pub ausfertigung_datum: Option<String>,
    pub title: Option<String>,
    pub stand: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SnapshotInfo {
    pub retrieved_at: String,
    pub archive_sha256: String,
    pub xml_sha256: String,
    pub dtd_sha256: String,
    pub modified: bool,
}
