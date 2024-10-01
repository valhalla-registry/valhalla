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

    let crate_id: i64 = sqlx::query_scalar("SELECT id FROM crates WHERE name = ?")
        .bind(&name)
        .fetch_optional(&app.db.pool)
        .await?
        .ok_or(ApiError(anyhow!("crate does not exist!")))?;

    // TODO: check if the author is an owner of this crate
    let is_owner: bool = sqlx::query_scalar("SELECT COUNT(1) FROM crate_owners WHERE user_id = ? AND crate_id = ?")
        .bind(&token.user_id)
        .bind(crate_id)
        .fetch_one(&app.db.pool)
        .await?;

    tracing::debug!("is owner: {}", is_owner);

    let version = version.parse::<Version>()?;

    // Check if crate (with version) exists.
    // This method returns Err if the record does not exist
    // and then does an early return from this function.
    // app.index.match_record(&name, version.clone().into())?;

    // yank the version of this crate on the index
    tracing::info!("Yanking crate '{} ({})'", name, version);
    app.index.yank_record(&name, version)?;

    Ok(Json(YankReponse { ok: true }))
}
