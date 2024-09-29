use std::fs::Metadata;
use std::io::Cursor;

use crate::{
    auth::{
        backend::{Scope, Token},
        Auth,
    },
    index::IndexTrait,
};
use axum::{body::Bytes, extract::State, Json};
use semver::VersionReq;
use serde::{Deserialize, Serialize};
use sqlx::{Sqlite, SqlitePool, Transaction};
use tokio::io::{AsyncReadExt, BufReader};

use crate::api::error::ApiError2;
use crate::app::App;
use crate::models::crates::{CrateMetadata, CrateVersion};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublishResponse {}

pub async fn handler(
    Auth(token): Auth<Token>,
    State(state): State<App>,
    bytes: Bytes,
) -> Result<Json<PublishResponse>, ApiError2> {
    if !token.scope.intersects(Scope::PUBLISH) {
        // return Err(ApiError(anyhow!(
        //     "your api token does not contain the publish-new and publish-update scope!"
        // )));
        return Err(ApiError2::MissingTokenScope(Scope::PUBLISH));
    }

    let mut reader = BufReader::new(Cursor::new(bytes));

    // extract metadata from the request body
    let metadata_size = reader.read_u32_le().await?;
    let mut metadata_bytes = vec![0u8; metadata_size as usize];
    reader.read_exact(&mut metadata_bytes).await?;
    let metadata: CrateMetadata = serde_json::from_slice(&metadata_bytes)?;

    tracing::info!(
        "Publishing crate: '{}' (version {})",
        metadata.name,
        metadata.version
    );

    // extract crate bytes from the request body
    let crate_size = reader.read_u32_le().await?;
    let mut crate_bytes = vec![0u8; crate_size as usize];
    reader.read_exact(&mut crate_bytes).await?;
    // let hash = sha256::digest(&crate_bytes);

    let mut transaction = state.db.pool.begin().await?;

    let crate_id: Option<i64> = sqlx::query_scalar("SELECT id FROM crates WHERE name = $1")
        .bind(&metadata.name)
        .fetch_optional(&mut *transaction)
        .await?;

    match crate_id {
        // crate exists, token has correct scope
        Some(id) if token.scope.contains(Scope::PUBLISH_UPDATE) => {
            publish_update(&mut transaction, &state, &token, id, metadata, crate_bytes).await?;
        }
        // crate exists, token has wrong scope
        Some(_) => {
            tracing::info!("Token does not have the publish-update scope!");
            return Err(ApiError2::MissingTokenScope(Scope::PUBLISH_UPDATE));
        }
        // crate does not exist, token has correct scope
        None if token.scope.contains(Scope::PUBLISH_NEW) => {
            publish_new(&mut transaction, &state, &token, metadata, crate_bytes).await?;
            // tracing::debug!(
            //     "Publishing new crate '{}' (version: {})",
            //     metadata.name,
            //     metadata.version
            // );
            // // store crate on the disk
            // state
            //     .storage
            //     .store_crate(&metadata.name, &metadata.version, &crate_bytes)?;
            //
            // // create index record for the new crate
            // let mut record: CrateVersion = metadata.clone().into();
            // record.checksum = hash; // FIXME
            // state.index.add_record(record).unwrap(); // FIXME: remove unwrap
            //
            // // create crate entry
            // // FIXME: get id with `RETURNING`
            // let _ = sqlx::query("INSERT INTO crates (name, description, documentation, repository) VALUES ($1, $2, $3, $4)")
            //     .bind(&metadata.name)
            //     .bind(&metadata.description)
            //     .bind(&metadata.documentation)
            //     .bind(&metadata.repository)
            //     .execute(&mut *transaction)
            //     .await?;
            //
            // // get id of newly created entry
            // let id: Option<i64> = sqlx::query_scalar("SELECT id FROM crates WHERE name = $1")
            //     .bind(&metadata.name)
            //     .fetch_optional(&mut *transaction)
            //     .await?;
            //
            // // if the previous query returned none, rollback the transaction and return an error
            // let Some(crate_id) = id else {
            //     transaction.rollback().await?;
            //     // TODO: delete crate from disk + remove index record
            //     // return Err(ApiError(anyhow!("internal database error")));
            //     return Err(ApiError2::Other("internal database error".into()));
            // };
            //
            // // insert user as an owner for this new crate
            // let _ = sqlx::query("INSERT INTO crate_owners (user_id, crate_id) VALUES ($1, $2)")
            //     .bind(&token.user_id)
            //     .bind(&crate_id)
            //     .execute(&mut *transaction)
            //     .await?;
        }
        // crate does not exist, token has wrong scope
        None => {
            return Err(ApiError2::MissingTokenScope(Scope::PUBLISH_NEW));
        }
    }

    sqlx::query("INSERT INTO crate_versions (name, version, created_at) VALUES ($1, $2, $3)")
        .bind(&metadata.name)
        .bind(&metadata.version.to_string())
        .bind(chrono::Utc::now().timestamp())
        .execute(&mut *transaction)
        .await?;

    transaction.commit().await?;

    Ok(Json(PublishResponse {}))
}

/// Publish a new crate
async fn publish_new(
    transaction: &mut Transaction<Sqlite>,
    /// The App-State
    state: &App,
    /// The API token
    token: &Token,
    /// The crate metadata sent by the client
    metadata: CrateMetadata,
    /// The crate tarball bytes sent by the client
    crate_bytes: Vec<u8>,
) -> Result<(), ApiError2> {
    tracing::debug!(
        "Publishing new crate '{}' (version: {})",
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

    // create crate entry
    // FIXME: get id with `RETURNING`
    let _ = sqlx::query(
        "INSERT INTO crates (name, description, documentation, repository) VALUES ($1, $2, $3, $4)",
    )
    .bind(&metadata.name)
    .bind(&metadata.description)
    .bind(&metadata.documentation)
    .bind(&metadata.repository)
    .execute(&mut *transaction)
    .await?;

    // get id of newly created entry
    let id: Option<i64> = sqlx::query_scalar("SELECT id FROM crates WHERE name = $1")
        .bind(&metadata.name)
        .fetch_optional(&mut *transaction)
        .await?;

    // if the previous query returned none, rollback the transaction and return an error
    let Some(crate_id) = id else {
        // transaction.rollback().await?;
        // TODO: delete crate from disk + remove index record
        // return Err(ApiError(anyhow!("internal database error")));
        return Err(ApiError2::Other("internal database error".into()));
    };

    // insert user as an owner for this new crate
    let _ = sqlx::query("INSERT INTO crate_owners (user_id, crate_id) VALUES ($1, $2)")
        .bind(&token.user_id)
        .bind(&crate_id)
        .execute(&mut *transaction)
        .await?;

    Ok(())
}

async fn publish_update(
    transaction: &mut Transaction<Sqlite>,
    /// The App-State
    state: &App,
    /// The API token
    token: &Token,
    /// The ID of the crate
    crate_id: i64,
    /// The crate metadata sent by the client
    metadata: CrateMetadata,
    /// The crate tarball bytes sent by the client
    crate_bytes: Vec<u8>,
) -> Result<(), ApiError2> {
    // check if the user is an owner of this crate
    let owners: Vec<i64> =
        sqlx::query_scalar("SELECT user_id FROM crate_owners WHERE crate_id = $1")
            .bind(crate_id)
            .fetch_all(&mut *transaction)
            .await?;

    if !owners.contains(&token.user_id) {
        return Err(ApiError2::CrateNotOwned);
    }

    // check if the crate already has this or a newer version
    let requirement = VersionReq::parse(&format!("^{}", &metadata.version))?;
    if let Ok(available) = state.index.match_record(&metadata.name, requirement) {
        return Err(ApiError2::VersionTooLow {
            available: available.version,
            provided: metadata.version,
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

    // create index record for the new crate
    let mut record: CrateVersion = metadata.clone().into(); // FIXME
    record.checksum = sha256::digest(&crate_bytes);
    state.index.add_record(record).unwrap(); // FIXME: remove unwrap

    // TODO: add version entry for crate in database
    Ok(())
}
