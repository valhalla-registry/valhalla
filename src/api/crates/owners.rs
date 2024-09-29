use crate::{
    app::App,
    auth::{
        backend::{Scope, Token},
        Auth,
    },
    error::ApiError,
};
use anyhow::anyhow;
use axum::extract::State;
use axum::{extract::Path, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, SqlitePool};

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnersListResponse {
    pub users: Vec<OwnerEntry>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct OwnerEntry {
    pub id: u32,
    #[serde(rename = "login")]
    pub email: String,
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

#[derive(Serialize)]
pub struct ErrorResponse {
    error: String,
}

pub async fn get_handler(
    State(app): State<App>,
    Path(name): Path<String>,
) -> Result<Json<OwnersListResponse>, ApiError> {
    tracing::trace!("Retrieving owners of crate '{}'", name);
    let crate_id: i64 = sqlx::query_scalar("SELECT id FROM crates WHERE name = $1")
        .bind(&name)
        .fetch_optional(&app.db.pool)
        .await?
        .ok_or(ApiError(anyhow!("Crate does not exist")))?;

    let owners: Vec<OwnerEntry> = sqlx::query_as(
        "
        SELECT id, name, email FROM users WHERE id IN (
            SELECT user_id FROM crate_owners WHERE crate_id = $1
        )",
    )
    .bind(&crate_id)
    .fetch_all(&app.db.pool)
    .await?;

    Ok(Json(OwnersListResponse { users: owners }))
}

pub async fn put_handler(
    Auth(token): Auth<Token>,
    State(app): State<App>,
    Path(name): Path<String>,
    Json(body): Json<OwnerAddBody>,
) -> Result<Json<OwnerAddResponse>, ApiError> {
    // check if the token-user has the correct permissions
    if !token.scope.contains(Scope::CHANGE_OWNERS) {
        return Err(ApiError(anyhow!(
            "your api token does not contain the change-owners scope!"
        )));
    }

    tracing::trace!("Adding owners to crate '{}': {:?}", name, body.users);

    // get id of the current crate
    let crate_id: i64 = sqlx::query_scalar("SELECT id FROM crates WHERE name = $1")
        .bind(&name)
        .fetch_optional(&app.db.pool)
        .await?
        .ok_or(ApiError(anyhow!("crate does not exist")))?;

    // get all user ids who are owner of this crate
    let existing_owners: Vec<i64> =
        sqlx::query_scalar("SELECT user_id FROM crate_owners WHERE crate_id = $1")
            .bind(&crate_id)
            .fetch_all(&app.db.pool)
            .await?;

    // check if the token-user is an owner of the current crate
    if !existing_owners.contains(&token.user_id) {
        return Err(ApiError(anyhow!(
            "Unauthorized: only owners have permission to modify owners!"
        )));
    }

    let new_owners = get_user_ids_by_emails(&app.db.pool, body.users)
        .await?
        // filter out new owners which are already existing owners
        .into_iter()
        .filter(|id| !existing_owners.contains(id))
        .collect::<Vec<_>>();

    // insert new owners
    if !new_owners.is_empty() {
        let sql = format!(
            "INSERT INTO crate_owners (crate_id, user_id) VALUES {}",
            new_owners
                .iter()
                .map(|_| "(?,?)")
                .collect::<Vec<_>>()
                .join(",")
        );
        let mut query = sqlx::query(&sql);
        for user_id in new_owners {
            query = query.bind(&crate_id).bind(user_id);
        }
        query.execute(&app.db.pool).await?;
    }

    // successfully added owners -> return success message
    Ok(Json(OwnerAddResponse {
        ok: true,
        msg: "added specified owners to crate".into(),
    }))
}

pub async fn delete_handler(
    Auth(token): Auth<Token>,
    State(app): State<App>,
    Path(name): Path<String>,
    Json(body): Json<OwnerDeleteBody>,
) -> Result<Json<OwnerDeleteResponse>, ApiError> {
    if !token.scope.contains(Scope::CHANGE_OWNERS) {
        return Err(ApiError(anyhow!(
            "your api token does not contain the change-owners scope!"
        )));
    }

    tracing::trace!("Removing owners from crate '{}': {:?}", name, body.users);

    // get id of the current crate
    let crate_id: i64 = sqlx::query_scalar("SELECT id FROM crates WHERE name = $1")
        .bind(&name)
        .fetch_optional(&app.db.pool)
        .await?
        .ok_or(ApiError(anyhow!("crate does not exist")))?;

    // Get user ids for the provided emails
    let user_ids = get_user_ids_by_emails(&app.db.pool, body.users).await?;

    // Get number of current owners of crate
    let current_owners_count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM crate_owners WHERE crate_id = ?")
            .bind(crate_id)
            .fetch_one(&app.db.pool)
            .await?;

    // Each crate MUST have an owner
    if current_owners_count == 1 {
        return Err(ApiError(anyhow!("Cannot remove the only owner!")));
    }

    // Remove user_ids as owners from crate_id
    remove_owners_if_exist(&app.db.pool, crate_id, user_ids).await?;

    // success
    Ok(Json(OwnerDeleteResponse {
        ok: true,
        msg: "removed specified owners from crate".into(),
    }))
}

/// Gets the `user_id`s for all existing user entries with an email in the list.
///
/// Potential error: if a user with a specified email does not exist, it is ignored, i.e. there
/// is no error if a user with email xyz does not exist.
async fn get_user_ids_by_emails(
    pool: &SqlitePool,
    emails: Vec<String>,
) -> Result<Vec<i64>, ApiError> {
    if emails.is_empty() {
        return Ok(vec![]);
    }

    let sql = format!(
        "SELECT id FROM users WHERE email IN ({})",
        emails.iter().map(|_| "?").collect::<Vec<_>>().join(",")
    );

    let mut query = sqlx::query_scalar(&sql);

    for email in &emails {
        query = query.bind(email);
    }

    let user_ids = query.fetch_all(pool).await?;

    Ok(user_ids)
}

/// Remove `crate_owners` entries for the given `crate_id` and the `user_id`s.
///
/// If a `user_id` is not an owner of that crate, it is just skipped.
async fn remove_owners_if_exist(
    pool: &SqlitePool,
    crate_id: i64,
    user_ids: Vec<i64>,
) -> Result<(), ApiError> {
    // if no ids are provided we return early
    if user_ids.is_empty() {
        return Ok(());
    }

    // construct sql query with placeholder for user ids
    let sql = format!(
        "DELETE FROM crate_owners WHERE crate_id = ? AND user_id IN ({})",
        user_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",")
    );

    // create query + bind crate id
    let mut query = sqlx::query(&sql).bind(crate_id);

    // bind each user id
    for user_id in user_ids {
        query = query.bind(user_id);
    }

    query.execute(pool).await?;

    Ok(())
}
