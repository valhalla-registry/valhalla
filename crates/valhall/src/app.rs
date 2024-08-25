use std::sync::Arc;

use crate::{config::Config, index::Index, storage::Storage};

pub type App = Arc<AppState>;

#[derive(Debug, Clone)]
pub struct AppState {
    pub index: Index,
    pub storage: Storage,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            index: Index::Git,
            storage: Storage {
                path: "./storage".into(),
            },
        }
    }
}

impl From<Config> for AppState {
    fn from(config: Config) -> Self {
        Self {
            index: Index::Git,
            storage: Storage {
                path: config.storage.path,
            },
        }
    }
}
