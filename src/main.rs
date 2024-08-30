use axum::Router;
use tokio::net::TcpListener;
use tracing::Level;
use valhall::{api, app::App, config::Config, frontend, APP_NAME, APP_VERSION};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    tracing::info!("Starting {} (Version: {})", APP_NAME, APP_VERSION);

    let config = Config::load("valhall.toml");

    // create app router
    let app = Router::new()
        .nest("/", frontend::router(&config.frontend))
        .nest("/api/v1", api::router())
        .with_state(App::from(&config));

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