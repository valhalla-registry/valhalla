use std::sync::Arc;

use crate::config::Config;
use crate::index::{git::GitIndex, Index};
use crate::storage::Storage;

pub type App = Arc<AppState>;

#[derive(Debug)]
pub struct AppState {
    pub index: Index,
    pub storage: Storage,
}

impl From<&Config> for App {
    fn from(config: &Config) -> Self {
        Self::new(AppState {
            index: Index::Git(GitIndex::new(config.index.path.clone())),
            storage: Storage::new(config.storage.path.clone()),
        })
    }
}
