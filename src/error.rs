use axum::{response::IntoResponse, Json};

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
