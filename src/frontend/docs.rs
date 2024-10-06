use std::{
    fmt::{self, Display},
    str::FromStr,
};

use askama_axum::IntoResponse;
use axum::{extract::Path, http::Uri, response::Redirect};
use semver::{Version, VersionReq};
use serde::Deserialize;
use serde_with::{DeserializeFromStr, SerializeDisplay};
use sqlx::{FromRow, SqlitePool};

use crate::error::Error;

// pub async fn index() -> &'static str {
//     "docs"
// }

#[derive(Clone, Deserialize, Debug)]
pub(crate) struct RustdocHtmlParams {
    pub(crate) name: String,
    pub(crate) version: ReqVersion,
    pub(crate) target: Option<String>,
    pub(crate) path: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, SerializeDisplay, DeserializeFromStr)]
pub(crate) enum ReqVersion {
    Exact(Version),
    Semver(VersionReq),
    #[default]
    Latest,
}

impl ReqVersion {
    pub(crate) fn is_latest(&self) -> bool {
        matches!(self, ReqVersion::Latest)
    }
}

impl Display for ReqVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReqVersion::Exact(version) => version.fmt(f),
            ReqVersion::Semver(version_req) => version_req.fmt(f),
            ReqVersion::Latest => write!(f, "latest"),
        }
    }
}

impl FromStr for ReqVersion {
    type Err = semver::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "latest" {
            Ok(ReqVersion::Latest)
        } else if let Ok(version) = Version::parse(s) {
            Ok(ReqVersion::Exact(version))
        } else if s.is_empty() || s == "newest" {
            Ok(ReqVersion::Semver(VersionReq::STAR))
        } else {
            VersionReq::parse(s).map(ReqVersion::Semver)
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct RustdocRedirectorParams {
    name: String,
    version: Option<ReqVersion>,
    target: Option<String>,
}

// ----------------------------------------------------------------------------

/// redirects to the full path
pub async fn rustdoc_redirector_handler(Path(params): Path<RustdocRedirectorParams>) -> Redirect {
    todo!()
}

// ----------------------------------------------------------------------------

/// serves the .html .js and .css files
pub async fn rustdoc_html_server_handler(
    Path(params): Path<RustdocHtmlParams>,
) -> impl IntoResponse {
    todo!()
}

// ----------------------------------------------------------------------------

async fn match_version(pool: &SqlitePool, name: &str, input_version: &ReqVersion) {}

async fn get_releases_for_crate(
    pool: &SqlitePool,
    crate_id: i64,
) -> Result<Vec<CrateRelease>, Error> {
    let releases = sqlx::query_as("SELECT ")
        .bind(crate_id)
        .fetch_all(pool)
        .await?;

    Ok(releases)
}

#[derive(Debug, FromRow)]
pub(crate) struct CrateRelease {
    pub id: i32,
    pub version: String,
}

// pub async fn docs_by_name(Path(name): Path<String>) -> Redirect {
//     Redirect::permanent(&format!("/docs/{name}/latest/{name}"))
// }

// pub async fn docs_by_version(
//     Path(params): Path<RustdocHtmlParams>,
//     uri: Uri,
// ) -> Result<String, String> {
//     let original_path = percent_encoding::percent_decode(uri.path().as_bytes())
//         .decode_utf8()
//         .map_err(|err| err.to_string())?;

//     let mut req_path: Vec<&str> = original_path.split('/').collect();
//     // // Remove the empty start, the name and the version from the path
//     // req_path.drain(..3).for_each(drop);

//     // remove empty start
//     req_path.drain(..1).for_each(drop);

//     // TODO: match release
//     // check if crate and its version exist

//     // prepare storage path
//     let mut storage_path = req_path.join("/");
//     if storage_path.ends_with('/') {
//         req_path.pop(); // get rid of empty string
//         storage_path.push_str("index.html");
//         req_path.push("index.html");
//     }

//     Ok(format!("{params:?}"))
// }
