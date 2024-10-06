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

    let docs_router = Router::new()
        // .route("/docs", get(docs::index))
        // .route("/docs/:name", get(docs::docs_by_name))
        // .route("/docs/:name/:version/:target/", get(docs::docs_by_version))
        // .route(
        //     "/docs/:name/:version/:target/*path",
        //     get(docs::docs_by_version),
        // )
        .route("/:name", get(docs::rustdoc_redirector_handler))
        .route("/:name/", get(docs::rustdoc_redirector_handler))
        .route("/:name/:version", get(docs::rustdoc_redirector_handler))
        .route("/:name/:version/", get(docs::rustdoc_redirector_handler))
        .route(
            "/:name/:version/all.html",
            get(docs::rustdoc_html_server_handler),
        )
        .route(
            "/:name/:version/help.html",
            get(docs::rustdoc_html_server_handler),
        )
        .route(
            "/:name/:version/settings.html",
            get(docs::rustdoc_html_server_handler),
        )
        .route(
            "/:name/:version/scrape-examples-help.html",
            get(docs::rustdoc_html_server_handler),
        )
        .route(
            "/:name/:version/:target",
            get(docs::rustdoc_redirector_handler),
        )
        .route(
            "/:name/:version/:target/",
            get(docs::rustdoc_html_server_handler),
        )
        .route(
            "/:name/:version/:target/*path",
            get(docs::rustdoc_html_server_handler),
        );
    // .nest_service(
    //     "/docs",
    //     ServeDir::new("/opt/valhall/docs/").append_index_html_on_directories(true),
    // );

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
    frontend_router
        .merge(auth_router)
        .nest_service(
            "/assets",
            ServeDir::new(&config.assets_dir).append_index_html_on_directories(false),
        )
        .nest_service(
            "/static/rustdoc",
            ServeDir::new("./static/rustdoc").append_index_html_on_directories(false),
        )
}
