use axum::{body::Bytes, extract::State, Json};
use semver::VersionReq;
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use tokio::io::{AsyncReadExt, BufReader};

use crate::{
    api::error::ApiError2,
    app::App,
    auth::{
        backend::{Scope, Token},
        Auth,
    },
};
use valhall_index::IndexTrait;
use valhall_models::crates::{CrateMetadata, CrateVersion};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublishResponse {}

pub async fn handler(
    // The auth token used by Cargo
    Auth(token): Auth<Token>,
    // The App-State
    State(state): State<App>,
    // The body of the request as a byte stream
    bytes: Bytes,
) -> Result<Json<PublishResponse>, ApiError2> {
    if !token.scope.intersects(Scope::PUBLISH) {
        return Err(ApiError2::MissingTokenScope(Scope::PUBLISH));
    }

    let mut reader = BufReader::new(Cursor::new(bytes));

    // extract metadata from the request body
    let metadata_size = reader.read_u32_le().await?;
    let mut metadata_bytes = vec![0u8; metadata_size as usize];
    reader.read_exact(&mut metadata_bytes).await?;
    let metadata: CrateMetadata = serde_json::from_slice(&metadata_bytes)?;

    // extract crate tarball bytes from the request body
    let crate_size = reader.read_u32_le().await?;
    let mut crate_bytes = vec![0u8; crate_size as usize];
    reader.read_exact(&mut crate_bytes).await?;

    tracing::info!(
        "Publishing crate: '{}' (version {})",
        metadata.name,
        metadata.version
    );

    // get the id of the crate (Some(_) if it already exists, otherwise None)
    let crate_id: Option<i64> = sqlx::query_scalar("SELECT id FROM crates WHERE name = $1")
        .bind(&metadata.name)
        .fetch_optional(&state.db.pool)
        .await?;

    match crate_id {
        Some(id) if token.scope.contains(Scope::PUBLISH_UPDATE) => {
            // crate exists, token has correct scope
            publish_update(&state, &token, id, &metadata, crate_bytes).await?;
        }
        Some(_) => {
            // crate exists, token has wrong scope
            tracing::trace!("Token does not have the publish-update scope!");
            return Err(ApiError2::MissingTokenScope(Scope::PUBLISH_UPDATE));
        }
        None if token.scope.contains(Scope::PUBLISH_NEW) => {
            // crate does not exist, token has correct scope
            publish_new(&state, &token, &metadata, crate_bytes).await?;
        }
        None => {
            // crate does not exist, token has wrong scope
            tracing::trace!("Token does not have the publish-new scope!");
            return Err(ApiError2::MissingTokenScope(Scope::PUBLISH_NEW));
        }
    }

    Ok(Json(PublishResponse {}))
}

/// Publish a new crate
async fn publish_new(
    // The App-State
    state: &App,
    // The API token
    token: &Token,
    // The crate metadata sent by the client
    metadata: &CrateMetadata,
    // The crate tarball bytes sent by the client
    crate_bytes: Vec<u8>,
) -> Result<(), ApiError2> {
    tracing::trace!(
        "Publishing new crate '{}' (v{})",
        metadata.name,
        metadata.version
    );
    // store crate on the disk
    state
        .storage
        .store_crate(&metadata.name, &metadata.version, &crate_bytes)?;

    // create index record for the new crate
    let mut record: CrateVersion = metadata.clone().into(); // FIXME
    record.checksum = sha256::digest(&crate_bytes);
    state.index.add_record(record).unwrap(); // FIXME: remove unwrap

    let mut tx = state.db.pool.begin().await?;

    // create crate entry
    let crate_id: i64 = sqlx::query_scalar(
        "INSERT INTO crates (name, description, documentation, repository) VALUES ($1, $2, $3, $4) RETURNING id",
    )
    .bind(&metadata.name)
    .bind(&metadata.description)
    .bind(&metadata.documentation)
    .bind(&metadata.repository)
    .fetch_one(&mut *tx)
    .await?;

    // insert user as an owner for this new crate
    let _ = sqlx::query("INSERT INTO crate_owners (user_id, crate_id) VALUES ($1, $2)")
        .bind(&token.user_id)
        .bind(&crate_id)
        .execute(&mut *tx)
        .await?;

    // Insert a version entry for the crate (regardless if it is new or an update)
    sqlx::query("INSERT INTO crate_versions (name, version, created_at) VALUES ($1, $2, $3)")
        .bind(&metadata.name)
        .bind(&metadata.version.to_string())
        .bind(chrono::Utc::now().timestamp())
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(())
}

/// Publish a new version of a crate
async fn publish_update(
    // The App-State
    state: &App,
    // The API token
    token: &Token,
    // The ID of the crate
    crate_id: i64,
    // The crate metadata sent by the client
    metadata: &CrateMetadata,
    // The crate tarball bytes sent by the client
    crate_bytes: Vec<u8>,
) -> Result<(), ApiError2> {
    tracing::trace!(
        "publishing update of crate {} (v{})",
        &metadata.name,
        metadata.version
    );

    // check if the user is an owner of this crate
    let owners: Vec<i64> =
        sqlx::query_scalar("SELECT user_id FROM crate_owners WHERE crate_id = $1")
            .bind(crate_id)
            .fetch_all(&state.db.pool)
            .await?;

    if !owners.contains(&token.user_id) {
        return Err(ApiError2::CrateNotOwned);
    }

    // check if the crate already has this or a newer version
    let requirement = VersionReq::parse(&format!("^{}", &metadata.version))?;
    if let Ok(available) = state.index.match_record(&metadata.name, requirement) {
        return Err(ApiError2::VersionTooLow {
            available: available.version,
            provided: metadata.version.clone(),
        });
    }

    tracing::debug!(
        "Publishing update for crate '{}' (version: {})",
        metadata.name,
        metadata.version
    );

    // store crate on the disk
    state
        .storage
        .store_crate(&metadata.name, &metadata.version, &crate_bytes)?;

    // TODO: render + store readme on the disk

    // create index record for the new crate
    let mut record: CrateVersion = metadata.clone().into(); // FIXME
    record.checksum = sha256::digest(&crate_bytes);
    state.index.add_record(record).unwrap(); // FIXME: remove unwrap

    // Insert a version entry for the crate (regardless if it is new or an update)
    sqlx::query("INSERT INTO crate_versions (name, version, created_at) VALUES ($1, $2, $3)")
        .bind(&metadata.name)
        .bind(&metadata.version.to_string())
        .bind(chrono::Utc::now().timestamp())
        .execute(&state.db.pool)
        .await?;

    Ok(())
}
