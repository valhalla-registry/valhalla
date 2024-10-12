use askama_axum::IntoResponse;
use axum::{
    extract::{Path, State},
    http::Uri,
    response::Redirect,
};
use serde::Deserialize;

use crate::{
    app::App,
    error::Error,
    frontend::docs::{fetch_rustdoc_file, match_version},
};

use super::version::ReqVersion;

#[derive(Clone, Deserialize, Debug)]
pub(crate) struct RustdocHtmlParams {
    pub(crate) name: String,
    pub(crate) version: ReqVersion,
    pub(crate) target: Option<String>,
    pub(crate) path: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct RustdocRedirectorParams {
    name: String,
    version: Option<ReqVersion>,
    target: Option<String>,
}

/// redirects to the full path
pub async fn rustdoc_redirector_handler(
    State(state): State<App>,
    Path(params): Path<RustdocRedirectorParams>,
) -> Result<Redirect, Error> {
    let (crate_name, _path_in_crate) = match params.name.split_once("::") {
        Some((krate, path)) => (krate.to_string(), Some(path.to_string())),
        None => (params.name.to_string(), None),
    };

    let matched_release = match_version(
        &state.db.pool,
        &crate_name,
        &params.version.unwrap_or(ReqVersion::Latest),
    )
    .await?;
    tracing::trace!(?matched_release, "matched version");
    let crate_name = matched_release.name.clone();

    // let file = fetch_rustdoc_file(&crate_name, &matched_release.release.version, path);

    // TODO: allow different targets
    Ok(Redirect::to(&format!(
        "/docs/{crate_name}/{}/{}/",
        matched_release.release.version, matched_release.name
    )))
}

/// serves the .html .js and .css files
pub async fn rustdoc_html_server_handler(
    State(state): State<App>,
    Path(params): Path<RustdocHtmlParams>,
    uri: Uri,
) -> Result<impl IntoResponse, Error> {
    let original_path = percent_encoding::percent_decode(uri.path().as_bytes())
        .decode_utf8()
        .map_err(|err| Error::Other(err.to_string()))?;

    let mut req_path: Vec<&str> = original_path.split('/').collect();
    // // Remove the empty start, the name and the version from the path
    // req_path.drain(..3).for_each(drop);

    // remove empty start
    req_path.drain(..3).for_each(drop);

    // TODO: match release
    // check if crate and its version exist
    let (crate_name, _path_in_crate) = match params.name.split_once("::") {
        Some((krate, path)) => (krate.to_string(), Some(path.to_string())),
        None => (params.name.to_string(), None),
    };

    let matched_release = match_version(&state.db.pool, &crate_name, &params.version).await?;

    // prepare storage path
    let mut storage_path = req_path.join("/");
    if storage_path.ends_with('/') {
        req_path.pop(); // get rid of empty string
        storage_path.push_str("index.html");
        req_path.push("index.html");
    }

    let file = fetch_rustdoc_file(
        &matched_release.name,
        &matched_release.release.version,
        &storage_path,
    )
    .await?;

    Ok(format!("{storage_path:?}"))
}
