use askama::Template;
use axum::extract::{Path, State};

use crate::{app::App, models::CrateMetadata};

#[derive(Template)]
#[template(path = "crates/index.html")]
pub(crate) struct IndexTemplate {
    /// the name of the crate
    pub name: String,
    pub version: String,
    pub readme: String,
}

pub async fn index_handler(Path(name): Path<String>, State(state): State<App>) -> IndexTemplate {
    IndexTemplate {
        name,
        version: "1.2.3-abc.1".into(),
        readme: "TODO: render and display readme.md".into(),
    }
}
