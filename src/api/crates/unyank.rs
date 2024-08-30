use crate::index::IndexTrait;
use axum::{
    extract::{Path, State},
    Json,
};
use semver::Version;
use serde::Serialize;

use crate::{app::App, error::ApiError};

#[derive(Debug, Serialize)]
pub struct UnyankResponse {
    ok: bool,
}

pub async fn handler(
    State(app): State<App>,
    Path((name, version)): Path<(String, String)>,
) -> Result<Json<UnyankResponse>, ApiError> {
    tracing::info!("Unyanking crate '{} ({})'", name, version);
    app.index.unyank_record(&name, Version::parse(&version)?)?;
    Ok(Json(UnyankResponse { ok: true }))
}
