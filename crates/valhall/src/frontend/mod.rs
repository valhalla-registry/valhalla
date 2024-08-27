use axum::{extract::Path, routing::get, Router};
use tower_http::services::ServeDir;

use crate::{app::App, config::FrontendConfig};

mod account;
mod crates;
mod index;

pub fn router(config: &FrontendConfig) -> Router<App> {
    Router::new()
        .route("/", get(index::handler))
        // account routes
        .route("/account/login", get(account::login_handler))
        .route("/account/register", get(account::register_handler))
        // .route("/me", get(handler))
        // crate routes
        .route("/crate/:name", get(crates::index_handler))
        // static files
        .nest_service(
            "/assets",
            ServeDir::new(&config.assets_dir).append_index_html_on_directories(false),
        )
}
