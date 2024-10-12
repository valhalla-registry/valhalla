use axum::{response::IntoResponse, Json};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),
    #[error(transparent)]
    MigrationError(#[from] sqlx::migrate::MigrateError),
    #[error(transparent)]
    SemverError(#[from] semver::Error),

    #[error("Error: {0}")]
    Other(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        Json(serde_json::json!({
            "errors": [{
                "detail": self.to_string()
            }]
        }))
        .into_response()
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct ApiError(pub anyhow::Error);

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        Json(serde_json::json!({
            "errors": [{
                "detail": self.0.to_string()
            }]
        }))
        .into_response()
    }
}

impl<E> From<E> for ApiError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
