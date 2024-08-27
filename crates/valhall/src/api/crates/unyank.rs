use axum::extract::{Path, State};

use crate::app::App;

pub async fn handler(State(app): State<App>, Path((name, version)): Path<(String, String)>) {
    tracing::info!("Unyanking crate '{} ({})'", name, version);
}
