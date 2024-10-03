pub mod backend;
pub mod frontend;

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{header, request::Parts, StatusCode},
    response::Redirect,
};

use crate::app::App;

pub struct RequireAuth;

#[async_trait]
impl FromRequestParts<App> for RequireAuth {
    type Rejection = Redirect;

    async fn from_request_parts(parts: &mut Parts, state: &App) -> Result<Self, Self::Rejection> {
        let session_token = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or(Redirect::to("/account/login"))?;

        let session_timestamp: i64 =
            sqlx::query_scalar("SELECT expires FROM sessions WHERE token = ?")
                .bind(session_token)
                .fetch_optional(&state.db.pool)
                .await
                // some database error occured
                .map_err(|_| Redirect::to("/account/login"))? // FIXME
                // if the query returns None, the session token does
                // not exist and we return early because the user is unauthorized
                .ok_or(Redirect::to("/account/login"))?;

        if session_timestamp <= chrono::Utc::now().timestamp() {
            // the token the user used is
            // remove the database entry for this session
            sqlx::query("DELETE FROM sessions WHERE token = ?")
                .bind(session_token)
                .execute(&state.db.pool)
                .await
                .map_err(|_| Redirect::to("/account/login"))?; //FIXME

            // return 401 unauthorized
            return Err(Redirect::to("/account/login"));
        }

        Ok(Self)
    }
}

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
