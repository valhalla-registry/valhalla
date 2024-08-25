use axum::{routing::get, Router};

mod account;
mod crates;
mod index;

pub fn router() -> Router {
    Router::new()
        .route("/", get(index::handler))
        // account routes
        .route("/account/login", get(account::login_handler))
        .route("/account/register", get(account::register_handler))
    // .route("/me", get(handler))
}
