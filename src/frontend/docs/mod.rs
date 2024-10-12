use std::{
    fmt::{self, Display},
    str::FromStr,
};

use askama_axum::IntoResponse;
use axum::{
    extract::{Path, State},
    http::Uri,
    response::Redirect,
};
use semver::{Version, VersionReq};
use serde::Deserialize;
use serde_with::{DeserializeFromStr, SerializeDisplay};
use sqlx::{FromRow, SqlitePool};
use version::ReqVersion;

use crate::{app::App, error::Error};

mod file;
pub mod routes;
mod version;

// pub async fn index() -> &'static str {
//     "docs"
// }

// ----------------------------------------------------------------------------

struct Blob {}

async fn fetch_rustdoc_file(name: &str, version: &str, path: &str) -> Result<Blob, Error> {
    todo!()
}

// ----------------------------------------------------------------------------

// ----------------------------------------------------------------------------

#[derive(Debug)]
struct MatchedRelease {
    pub name: String,
    // pub corrected_name: Option<String>,
    pub req_version: ReqVersion,
    pub release: CrateRelease,
    // pub(crate) all_releases: Vec<CrateRelease>,
}

async fn match_version(
    pool: &SqlitePool,
    name: &str,
    input_version: &ReqVersion,
) -> Result<MatchedRelease, Error> {
    // let crate_id: i32 = sqlx::query_scalar("SELECT id FROM crates WHERE name = ?")
    //     .bind(&name)
    //     .fetch_one(pool)
    //     .await?;

    // tracing::debug!(?crate_id);

    let releases = get_releases_for_crate(pool, name).await?;

    if releases.is_empty() {
        return Err(Error::Other("no release found".into()));
    }

    let req_semver = match input_version {
        ReqVersion::Exact(parsed_req_version) => {
            if let Some(release) = releases
                .iter()
                .find(|release| &Version::parse(&release.version).unwrap() == parsed_req_version)
            {
                return Ok(MatchedRelease {
                    name: name.to_owned(),
                    // corrected_name,
                    req_version: input_version.clone(),
                    release: release.clone(),
                    // all_releases: releases,
                });
            }

            if let Ok(version_req) = VersionReq::parse(&parsed_req_version.to_string()) {
                // when we don't find a release with exact version,
                // we try to interpret it as a semver requirement.
                // A normal semver version ("1.2.3") is equivalent to a caret semver requirement.
                version_req
            } else {
                return Err(Error::Other("VersionNotFound".into()));
            }
        }
        ReqVersion::Latest => VersionReq::STAR,
        ReqVersion::Semver(version_req) => version_req.clone(),
    };

    let (_, latest_matching) = releases
        .into_iter()
        .map(|r| (Version::parse(&r.version).unwrap(), r))
        .filter(|(v, _)| req_semver.matches(v))
        .max_by(|(v, _), (v2, _)| v.cmp(v2))
        .ok_or(Error::Other(format!(
            "no matching version found for {}",
            input_version
        )))?;

    Ok(MatchedRelease {
        name: name.to_owned(),
        req_version: input_version.to_owned(),
        release: latest_matching,
    })
}

async fn get_releases_for_crate(pool: &SqlitePool, name: &str) -> Result<Vec<CrateRelease>, Error> {
    let releases = sqlx::query_as("SELECT id, version FROM crate_versions WHERE name = ?")
        .bind(name)
        .fetch_all(pool)
        .await?;

    Ok(releases)
}

#[derive(Debug, Clone, FromRow)]
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
