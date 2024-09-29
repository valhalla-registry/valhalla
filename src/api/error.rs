use crate::auth::backend::Scope;
use askama_axum::Response;
use axum::response::IntoResponse;
use axum::Json;
use semver::Version;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError2 {
    /// SQLX database error
    #[error("An internal database error occurred")]
    DatabaseError(#[from] sqlx::Error),

    /// IO error
    #[error("Encountered an internal IO error")]
    IoError(#[from] std::io::Error),

    /// Serde error
    #[error("Encountered an internal error 1: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("Encountered an internal error 2: {0}")]
    SemverError(#[from] semver::Error),

    /// An error saying that the token does not contain the required scope
    #[error("The api token does not contain the `{0}` scope")]
    MissingTokenScope(Scope),

    #[error("You are not an owner of this crate!")]
    CrateNotOwned,

    #[error("Version too low! Available {available} but provided {provided}!")]
    VersionTooLow {
        provided: Version,
        available: Version,
    },

    /// Wildcard error: remove for production
    #[error("Other error: {0}")]
    Other(String),
}

impl IntoResponse for ApiError2 {
    fn into_response(self) -> Response {
        Json(serde_json::json!({
            "errors": [{
                "detail": self.to_string()
            }]
        }))
        .into_response()
    }
}
