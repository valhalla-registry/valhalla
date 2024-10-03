use axum::{middleware::from_extractor_with_state, response::Redirect, routing::get, Router};
use tower_http::services::ServeDir;

use crate::{app::App, auth::RequireAuth};
use valhall_config::FrontendConfig;

mod account;
mod crates;
mod docs;
mod index;
mod search;

pub fn router(config: &FrontendConfig, state: App) -> Router<App> {
    let auth_router = Router::new()
        .route("/account/login", get(account::login_handler))
        .route("/account/register", get(account::register_handler));

    let account_router = Router::new()
        .route("/account/profile", get(account::profile_handler))
        .route("/account/token", get(account::token_handler))
        .route("/me", get(|| async { Redirect::to("/account/token") }));

    let crates_router = Router::new()
        .route("/crates/:name", get(crates::handler))
        .route("/crates/:name/versions", get(crates::versions_handler))
        .route("/crates/:name/:version/dependencies", get(|| async {}))
        .route("/crates/:name/:version/dependents", get(|| async {}));

    let docs_router = Router::new().route("/docs", get(docs::index));

    let mut frontend_router = Router::new()
        .route("/", get(index::handler))
        .route("/search", get(search::handler))
        .merge(account_router)
        .merge(crates_router)
        .merge(docs_router);

    if config.require_auth {
        frontend_router =
            frontend_router.route_layer(from_extractor_with_state::<RequireAuth, App>(state))
    }

    // all other routes may not require authorization
    frontend_router.merge(auth_router).nest_service(
        "/assets",
        ServeDir::new(&config.assets_dir).append_index_html_on_directories(false),
    )
}
