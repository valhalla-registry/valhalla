use std::io::Cursor;

use axum::{body::Bytes, extract::State, Json};
use hashbrown::HashMap;
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, BufReader};

use crate::{app::App, error::ApiError};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublishResponse {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct CrateMetadata {
    pub name: String,
    pub vers: Version,
    pub deps: Vec<CrateDependency>,
    pub features: HashMap<String, Vec<String>>,
    pub authors: Vec<String>,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub documentation: Option<String>,
    pub readme: Option<String>,
    pub readme_file: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    pub license: Option<String>,
    pub license_file: Option<String>,
    pub repository: Option<String>,
    pub badges: Option<HashMap<String, HashMap<String, String>>>,
    pub links: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct CrateDependency {
    pub name: String,
    pub version_req: VersionReq,
    pub features: Vec<String>,
    pub optional: bool,
    pub default_features: bool,
    pub target: Option<String>,
    pub kind: Option<CrateDependencyKind>,
    pub registry: Option<String>,
    #[serde(rename = "explicit_name_in_toml")]
    pub explicit_name: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CrateDependencyKind {
    /// A normal dependency.
    Normal,
    /// A build dependency.
    Build,
    /// A developement dependency.
    Dev,
}

pub async fn handler(
    State(state): State<App>,
    bytes: Bytes,
) -> Result<Json<PublishResponse>, ApiError> {
    let mut reader = BufReader::new(Cursor::new(bytes));

    // extract metadata from the request body
    let metadata_size = reader.read_u32_le().await?;
    let mut metadata_bytes = vec![0u8; metadata_size as usize];
    reader.read_exact(&mut metadata_bytes).await?;
    let metadata: CrateMetadata = serde_json::from_slice(&metadata_bytes).unwrap();

    tracing::info!(
        "Registering new crate: '{}' (version {})",
        metadata.name,
        metadata.vers
    );

    // TODO: check if crate already exists (maybe not
    // necessary because cargo could already check if
    // the index does contain this crate/version)

    // extract crate bytes from the request body
    let crate_size = reader.read_u32_le().await?;
    let mut crate_bytes = vec![0u8; crate_size as usize];
    reader.read_exact(&mut crate_bytes).await?;

    tracing::debug!(
        "Size of crate '{}': {:.02} KB",
        metadata.name,
        crate_size as f64 / 1024.0
    );

    // save the raw bytes as .crate file in the storage directory
    state
        .storage
        .store_crate(metadata.name, metadata.vers, &crate_bytes)?;

    Ok(Json(PublishResponse {}))
}
