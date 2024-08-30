use askama::Template;
use axum::extract::Query;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SearchQueryRequest {
    q: String,
}

#[derive(Template)]
#[template(path = "search.html")]
pub struct SearchTemplate {
    query: String,
    results: Vec<SearchResult>,
}

#[derive(Serialize)]
pub struct SearchResult {
    name: String,
    version: String,
    description: String,
}

pub async fn handler(Query(search): Query<SearchQueryRequest>) -> SearchTemplate {
    SearchTemplate {
        query: search.q,
        results: vec![SearchResult {
            name: "test".into(),
            version: "1.2.3-a.1".into(),
            description: "This crates does not actually exist!".into(),
        }],
    }
}
