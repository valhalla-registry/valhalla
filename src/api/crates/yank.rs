use crate::index::IndexTrait;
use axum::{
    extract::{Path, State},
    Json,
};
use semver::Version;
use serde::Serialize;

use crate::{app::App, auth::Auth, error::ApiError};

#[derive(Serialize)]
pub struct YankReponse {
    ok: bool,
}

pub async fn handler(
    Auth(author): Auth,
    State(app): State<App>,
    Path((name, version)): Path<(String, String)>,
) -> Result<Json<YankReponse>, ApiError> {
    tracing::info!("Yanking crate '{} ({})'", name, version);
    app.index.yank_record(&name, Version::parse(&version)?)?;
    Ok(Json(YankReponse { ok: true }))
}
