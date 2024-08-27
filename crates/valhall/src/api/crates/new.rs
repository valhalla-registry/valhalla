use std::io::Cursor;

use axum::{body::Bytes, extract::State, Json};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, BufReader};
use valhall_index::IndexTrait;

use crate::{app::App, error::ApiError, models::CrateMetadata};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublishResponse {}

pub async fn handler(
    State(state): State<App>,
    bytes: Bytes,
) -> Result<Json<PublishResponse>, ApiError> {
    let mut reader = BufReader::new(Cursor::new(bytes));

    // extract metadata from the request body
    let metadata_size = reader.read_u32_le().await?;
    let mut metadata_bytes = vec![0u8; metadata_size as usize];
    reader.read_exact(&mut metadata_bytes).await?;
    let msg = String::from_utf8(metadata_bytes.clone()).unwrap();
    tracing::debug!("{}", msg);
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
        .store_crate(&metadata.name, &metadata.vers, &crate_bytes)?;

    tracing::debug!("adding record");

    state.index.add_record(metadata.into())?;

    Ok(Json(PublishResponse {}))
}
