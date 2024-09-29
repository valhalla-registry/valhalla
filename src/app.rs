use crate::config::Config;
use crate::db::Database;
use crate::index::{git::GitIndex, Index};
use crate::storage::Storage;
use std::sync::Arc;

pub type App = Arc<AppState>;

#[derive(Debug)]
pub struct AppState {
    pub index: Index,
    pub storage: Storage,
    pub db: Database,
}

impl AppState {
    pub async fn from_config(config: &Config) -> Self {
        AppState {
            index: Index::Git(GitIndex::new(config.index.path.clone())),
            storage: Storage::new(config.storage.path.clone()),
            db: Database::init(&config).await.unwrap(),
        }
    }
}
