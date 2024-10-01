use crate::app::App;
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use bitflags::bitflags;
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Row};
use std::fmt::Display;

bitflags! {
    /// Bitflags for the different scope variants of a token.
    ///
    /// This is not an enum because we need to store it in
    /// the database as a single field and arrays are not supported
    #[derive(Debug, PartialEq)]
    pub struct Scope: u32 {
        const CHANGE_OWNERS  = 0b0001;
        const PUBLISH_NEW    = 0b0010;
        const PUBLISH_UPDATE = 0b0100;
        const YANK           = 0b1000;

        /// union of publish-new and publish-update
        const PUBLISH = Self::PUBLISH_NEW.bits() | Self::PUBLISH_UPDATE.bits();
    }
}

impl Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = match self {
            &Self::CHANGE_OWNERS => "change-owners".into(),
            &Self::PUBLISH => "publish-(new|update)".into(),
            &Self::PUBLISH_NEW => "publish-new".into(),
            &Self::PUBLISH_UPDATE => "publish-update".into(),
            &Self::YANK => "yank".into(),
            _ => "unknown".into(),
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub(crate) struct Token {
    id: u64,
    token: String,
    name: String,
    pub(crate) scope: Scope,
    pub(crate) user_id: i64,
}

impl<'r> FromRow<'r, SqliteRow> for Token {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Token {
            id: row.try_get("id")?,
            token: row.try_get("token")?,
            name: row.try_get("name")?,
            scope: Scope::from_bits(row.try_get::<u32, _>("scope")?).ok_or(
                sqlx::Error::ColumnDecode {
                    index: "scope".to_string(),
                    source: Box::new(std::fmt::Error),
                },
            )?,
            user_id: row.try_get("user_id")?,
        })
    }
}

#[async_trait]
impl FromRequestParts<App> for Token {
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &App) -> Result<Self, Self::Rejection> {
        let auth = parts
            .headers
            .get("Authorization")
            .ok_or(StatusCode::UNAUTHORIZED)?
            .to_str()
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let token: Option<Token> = sqlx::query_as("SELECT * FROM tokens WHERE token = $1")
            .bind(auth)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        token.ok_or_else(|| StatusCode::UNAUTHORIZED)
    }
}

#[cfg(test)]
mod tests {
    use super::Scope;

    #[test]
    fn scope_intersect() {
        let scope = Scope::from_bits(Scope::YANK.bits() | Scope::PUBLISH_NEW.bits()).unwrap();
        assert!(scope.intersects(Scope::PUBLISH));

        let scope = Scope::from_bits(Scope::YANK.bits() | Scope::PUBLISH_UPDATE.bits()).unwrap();
        assert!(scope.intersects(Scope::PUBLISH));

        let scope =
            Scope::from_bits(Scope::PUBLISH_NEW.bits() | Scope::PUBLISH_UPDATE.bits()).unwrap();
        assert!(scope.intersects(Scope::PUBLISH));

        let scope = Scope::from_bits(Scope::YANK.bits() | Scope::CHANGE_OWNERS.bits()).unwrap();
        assert!(!scope.intersects(Scope::PUBLISH))
    }

    #[test]
    fn scope_contains() {
        let scope = Scope::from_bits(Scope::YANK.bits() | Scope::CHANGE_OWNERS.bits()).unwrap();
        assert!(scope.contains(Scope::YANK));

        let scope = Scope::from_bits(Scope::YANK.bits() | Scope::CHANGE_OWNERS.bits()).unwrap();
        assert!(!scope.contains(Scope::PUBLISH_NEW))
    }

    #[test]
    fn scope_publish() {
        let scope =
            Scope::from_bits(Scope::PUBLISH_NEW.bits() | Scope::PUBLISH_UPDATE.bits()).unwrap();
        assert_eq!(scope, Scope::PUBLISH);
    }
}
