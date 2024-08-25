use askama::Template;
use axum::extract::State;

use crate::{app::App, storage::Crate};

#[derive(Template)]
#[template(path = "index.html")]
pub(crate) struct IndexTemplate {
    pub(crate) crates: Vec<Crate>,
}

pub async fn handler(State(state): State<App>) -> IndexTemplate {
    IndexTemplate {
        crates: state.storage.get_all_crates(),
    }
}
