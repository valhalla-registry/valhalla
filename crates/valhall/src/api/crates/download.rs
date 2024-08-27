use axum::extract::Path;

use crate::error::ApiError;

pub async fn handler(Path((name, version)): Path<(String, String)>) -> Result<(), ApiError> {
    tracing::info!("Download request for crate '{} ({})'", name, version);
    Ok(())
}
