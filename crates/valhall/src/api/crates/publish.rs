use axum::{body::{Body, BodyDataStream, Bytes}, Json};
use http_body_util::BodyStream;
use serde::{Deserialize, Serialize};

use crate::error::ApiError;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublishResponse {}

pub async fn handler(bytes: Bytes) ->  Result<Json<PublishResponse>, ApiError> {
    std::fs::write("./test.crate", bytes).unwrap();
    Ok(Json(PublishResponse {}))
}
