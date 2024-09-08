use std::io::Cursor;

use crate::{
    auth::{
        backend::{Scope, Token},
        Auth,
    },
    index::{models::CrateVersion, IndexTrait},
};
use anyhow::anyhow;
use axum::{body::Bytes, extract::State, Json};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, BufReader};

use crate::{app::App, error::ApiError, models::CrateMetadata};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublishResponse {}

pub async fn handler(
    Auth(token): Auth<Token>,
    State(state): State<App>,
    bytes: Bytes,
) -> Result<Json<PublishResponse>, ApiError> {
    if !token.scopes.intersects(Scope::PUBLISH) {
        return Err(ApiError(anyhow!(
            "your api token does not contain the publish-new and publish-update scope!"
        )));
    }

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
    let hash = sha256::digest(&crate_bytes);

    tracing::debug!(
        "Size of crate '{}': {:.02} KB",
        metadata.name,
        crate_size as f64 / 1024.0
    );

    // save the raw bytes as .crate file in the storage directory
    state
        .storage
        .store_crate(&metadata.name, &metadata.vers, &crate_bytes)?;

    let mut record: CrateVersion = metadata.into();
    record.cksum = hash;
    state.index.add_record(record)?;

    Ok(Json(PublishResponse {}))
}
