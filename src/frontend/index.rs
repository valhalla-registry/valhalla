use crate::app::App;
use askama::Template;
use axum::extract::State;
use sqlx::FromRow;

#[derive(Template)]
#[template(path = "index.html")]
pub(crate) struct IndexTemplate {
    pub(crate) crates: Vec<Crate>,
}

#[allow(unused)]
#[derive(Debug, FromRow)]
pub struct Crate {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub documentation: String,
    pub repository: String,
}

pub async fn handler(State(state): State<App>) -> IndexTemplate {
    let crates: Vec<Crate> = sqlx::query_as("SELECT * FROM crates")
        .fetch_all(&state.db.pool)
        .await
        .unwrap();

    IndexTemplate { crates }
}
