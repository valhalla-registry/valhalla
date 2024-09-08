pub mod backend;
pub mod frontend;

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

use crate::app::App;

pub struct Auth<T>(pub(crate) T)
where
    T: FromRequestParts<App>;

#[async_trait]
impl<T> FromRequestParts<App> for Auth<T>
where
    T: FromRequestParts<App, Rejection = StatusCode>,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &App) -> Result<Self, Self::Rejection> {
        Ok(Self(T::from_request_parts(parts, state).await?))
    }
}
