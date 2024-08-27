use anyhow::anyhow;
use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{app::App, error::ApiError};

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnersListResponse {
    pub users: Vec<OwnerEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerEntry {
    pub id: i64,
    pub login: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerAddBody {
    pub users: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerAddResponse {
    pub ok: bool,
    pub msg: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerDeleteBody {
    pub users: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerDeleteResponse {
    pub ok: bool,
    pub msg: String,
}

pub async fn get_handler(
    State(app): State<App>,
    Path(name): Path<String>,
) -> Result<Json<OwnersListResponse>, ApiError> {
    tracing::info!("Retrieving owners of crate '{}'", name);
    Ok(Json(OwnersListResponse {
        users: vec![OwnerEntry {
            id: 1,
            login: "test@example.com".into(),
            name: "test".into(),
        }],
    }))
}

pub async fn put_handler(
    State(app): State<App>,
    Path(name): Path<String>,
    Json(body): Json<OwnerAddBody>,
) -> Result<Json<OwnerAddResponse>, ApiError> {
    tracing::info!("Adding owners to crate '{}': {:?}", name, body.users);
    Err(ApiError(anyhow!("not implemented yet")))
    // Ok(Json(OwnerAddResponse {
    //     ok: true,
    //     msg: "added specified owners to crate".into(),
    // }))
}

pub async fn delete_handler(
    State(app): State<App>,
    Path(name): Path<String>,
    Json(body): Json<OwnerDeleteBody>,
) -> Result<Json<OwnerDeleteResponse>, ApiError> {
    tracing::info!("Removing owners from crate '{}': {:?}", name, body.users);
    Err(ApiError(anyhow!("not implemented yet")))
    // Ok(Json(OwnerDeleteResponse {
    //     ok: true,
    //     msg: "removed specified owners from crate".into(),
    // }))
}
