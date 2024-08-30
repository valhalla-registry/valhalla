use axum::{response::Redirect, routing::get, Router};
use tower_http::services::ServeDir;

use crate::{app::App, config::FrontendConfig};

mod account;
mod crates;
mod index;
mod search;

pub fn router(config: &FrontendConfig) -> Router<App> {
    Router::new()
        .route("/", get(index::handler))
        // account routes
        .route("/account/login", get(account::login_handler))
        .route("/account/register", get(account::register_handler))
        .route("/account/profile", get(account::profile_handler))
        .route("/account/token", get(account::token_handler))
        .route("/me", get(|| async { Redirect::to("/account/token") }))
        // crate routes
        .route("/crates/:name", get(crates::handler))
        // search
        .route("/search", get(search::handler))
        // static files
        .nest_service(
            "/assets",
            ServeDir::new(&config.assets_dir).append_index_html_on_directories(false),
        )
}
