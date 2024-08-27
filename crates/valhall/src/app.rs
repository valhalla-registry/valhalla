use std::sync::Arc;

use valhall_index::{git::GitIndex, Index};
use valhall_storage::Storage;

use crate::config::Config;

pub type App = Arc<AppState>;

#[derive(Debug)]
pub struct AppState {
    pub index: Index,
    pub storage: Storage,
}

impl From<&Config> for App {
    fn from(config: &Config) -> Self {
        Arc::new(AppState {
            index: Index::Git(GitIndex::new(config.index.path.clone())),
            storage: Storage::new(config.storage.path.clone()),
        })
    }
}
