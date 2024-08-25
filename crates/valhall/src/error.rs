use axum::{response::IntoResponse, Json};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {

}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        Json(serde_json::json!({
            "error": [{
                "detail": self.to_string()
            }]
        }))
        .into_response()
    }
}
