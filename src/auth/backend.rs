use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use bitflags::bitflags;

use crate::app::App;

bitflags! {
    /// Bitflags for the different scope variants of a token.
    ///
    /// This is not an enum because we need to store it in
    /// the database as a single field and arrays are not supported
    #[derive(Debug, PartialEq)]
    pub struct Scope: u8 {
        const CHANGE_OWNERS  = 0b0000_0001;
        const PUBLISH_NEW    = 0b0000_0010;
        const PUBLISH_UPDATE = 0b0000_0100;
        const YANK           = 0b0000_1000;

        /// union of publish-new and publish-update
        const PUBLISH = Self::PUBLISH_NEW.bits() | Self::PUBLISH_UPDATE.bits();
    }
}

#[derive(Debug)]
pub struct Token {
    pub(crate) user_id: String,
    pub(crate) scopes: Scope,
}

#[async_trait]
impl FromRequestParts<App> for Token {
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &App) -> Result<Self, Self::Rejection> {
        let auth = parts
            .headers
            .get("Authorization")
            .ok_or(StatusCode::UNAUTHORIZED)?;

        // FIXME: get author and scopes from the database
        let api_token = Self {
            user_id: "user_id".into(),
            scopes: Scope::PUBLISH,
        };

        Ok(api_token)
    }
}

#[cfg(test)]
mod tests {
    use super::Scope;

    #[test]
    fn scopes_intersect() {
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
