use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub(crate) struct IndexTemplate {}

pub async fn handler() -> IndexTemplate {
    IndexTemplate {}
}
