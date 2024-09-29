use axum::{
    body::Bytes,
    extract::{Path, State},
};
use semver::Version;

use crate::{app::App, error::ApiError};

pub async fn handler(
    State(app): State<App>,
    Path((name, version)): Path<(String, String)>,
) -> Result<Bytes, ApiError> {
    tracing::info!("Download request for crate '{} ({})'", name, version);

    let crate_bytes = app.storage.get_crate(&name, Version::parse(&version)?)?;

    // update download counter for the crate+version
    sqlx::query(
        "UPDATE crate_versions SET downloads = downloads + 1 WHERE name = $1 AND version = $2",
    )
    .bind(&name)
    .bind(&version)
    .execute(&app.db.pool)
    .await?;

    Ok(Bytes::from(crate_bytes))
}
