use axum::{
    extract::Path,
    routing::{delete, get, post, put},
    Router,
};

use crate::app::App;

mod account;
mod crates;
mod index;

/// creates the router for all api endpoints
pub fn router() -> Router<App> {
    Router::new()
        // account api
        .route("/account/login", post(account::login::handler))
        .route("/account/register", post(account::register::handler))
        // crates api
        .route("/crates", get(crates::search::handler))
        .route("/crates/new", put(crates::new::handler))
        // .route("/crates/suggest", get(handler))
        .route("/crates/:name", get(crates::info::handler))
        .route(
            "/crates/:name/owners",
            get(crates::owners::get_handler)
                .put(crates::owners::put_handler)
                .delete(crates::owners::delete_handler),
        )
        .route("/crates/:name/:version/yank", delete(crates::yank::handler))
        .route(
            "/crates/:name/:version/unyank",
            put(crates::unyank::handler),
        )
        .route(
            "/crates/:name/:version/download",
            get(crates::download::handler),
        )
        // index api
        .route("/index/:path", get(index::handler))
    // .route("/index/git", get(index::git::handler))
    // .route("/index/sparse", get(index::sparse::handler))
}
