use anyhow::anyhow;
use axum::extract::Path;

use crate::error::ApiError;

pub async fn handler(Path(path): Path<String>) -> Result<&'static str, ApiError> {
    tracing::info!("index: {}", path);

    if path == "config.json" {
        Ok(r#"{
  "dl": "http://192.168.188.32:3000/api/v1/crates/{crate}/{version}/download",
  "api": "http://192.168.188.32:3000",
  "allowed-registries": [
    "https://github.com/rust-lang/crates.io-index",
    "sparse+http://localhost:3000/api/v1/index/"
  ]
}"#)
    } else {
        Err(ApiError(anyhow!("not implemented!")))
    }
}
