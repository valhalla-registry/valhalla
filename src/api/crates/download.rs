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
    Ok(Bytes::from(crate_bytes))
}
