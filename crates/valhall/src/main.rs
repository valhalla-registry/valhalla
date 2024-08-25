mod api;
mod config;
mod docs;
mod error;
mod frontend;
mod storage;

use axum::Router;
use config::Config;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct AppState {}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    tracing::info!("Starting {} (Version: {})", APP_NAME, APP_VERSION);

    let config = Config::load("valhall.toml");

    let app = Router::new()
        .nest("/", frontend::router())
        .nest("/api/v1", api::router())
        .nest_service(
            "/assets",
            ServeDir::new(&config.server.assets_dir).append_index_html_on_directories(false),
        );

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
