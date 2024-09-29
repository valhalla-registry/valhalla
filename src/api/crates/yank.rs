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

#[derive(Serialize)]
pub struct YankReponse {
    ok: bool,
}

pub async fn handler(
    Auth(token): Auth<Token>,
    State(app): State<App>,
    Path((name, version)): Path<(String, String)>,
) -> Result<Json<YankReponse>, ApiError> {
    tracing::debug!("token: {:?}", token);
    // check if the token contains the yank scope/permission
    if !token.scope.contains(Scope::YANK) {
        return Err(ApiError(anyhow!(
            "your api token does not contain the yank scope!"
        )));
    }

    // TODO: check if the author is an owner of this crate

    // TODO: check if crate (with version) exists

    // yank the version of this crate on the index
    tracing::info!("Yanking crate '{} ({})'", name, version);
    app.index.yank_record(&name, Version::parse(&version)?)?;

    Ok(Json(YankReponse { ok: true }))
}
