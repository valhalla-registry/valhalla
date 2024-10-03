use axum::Router;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use valhall_config::Config;

mod api;
mod app;
mod auth;
mod db;
mod error;
mod frontend;

use crate::app::AppState;

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=trace", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting {} (v{})", APP_NAME, APP_VERSION);

    let config = Config::load("valhall.toml");
    let state = Arc::new(AppState::from_config(&config).await);
    let app = Router::new()
        .nest("/", frontend::router(&config.frontend, state.clone()))
        .nest("/api/v1", api::router())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

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
