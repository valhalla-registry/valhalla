use axum::{
    async_trait,
    extract::{FromRequest, FromRequestParts, Request},
    http::{request::Parts, StatusCode},
    RequestPartsExt,
};

use crate::app::App;

pub struct Auth(pub String);

#[async_trait]
impl FromRequestParts<App> for Auth {
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &App) -> Result<Self, Self::Rejection> {
        let auth = parts
            .headers
            .get("Authorization")
            .ok_or(StatusCode::UNAUTHORIZED)?;

        tracing::debug!(bearer =? &auth);

        Ok(Auth("".into()))
    }
}
