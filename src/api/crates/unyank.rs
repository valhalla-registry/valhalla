use crate::{
    auth::{
        backend::{Scope, Token},
        Auth,
    },
    index::IndexTrait,
};
use anyhow::anyhow;
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
    Auth(token): Auth<Token>,
    State(app): State<App>,
    Path((name, version)): Path<(String, String)>,
) -> Result<Json<UnyankResponse>, ApiError> {
    if !token.scope.contains(Scope::YANK) {
        return Err(ApiError(anyhow!(
            "your api token does not contain the yank scope!"
        )));
    }
    tracing::info!("Unyanking crate '{} ({})'", name, version);
    app.index.unyank_record(&name, Version::parse(&version)?)?;
    Ok(Json(UnyankResponse { ok: true }))
}
