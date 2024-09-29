use axum::Router;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use valhall::app::AppState;
use valhall::{api, config::Config, frontend, APP_NAME, APP_VERSION};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        // .with_max_level(Level::DEBUG)
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting {} (Version: {})", APP_NAME, APP_VERSION);

    let config = Config::load("valhall.toml");

    // create app router
    let app = Router::new()
        .nest("/", frontend::router(&config.frontend))
        .nest("/api/v1", api::router())
        .layer(TraceLayer::new_for_http())
        .with_state(Arc::new(AppState::from_config(&config).await));

    tracing::info!(
        "Starting listener on {}:{}",
        config.server.ip,
        config.server.port
    );

    let listener = TcpListener::bind((config.server.ip, config.server.port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
