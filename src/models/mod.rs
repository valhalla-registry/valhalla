use crate::index::models::{CrateDependency, CrateVersion};
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Author {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrateMetadata {
    pub name: String,
    pub vers: Version,
    pub deps: Vec<CrateDependency>,
    pub features: HashMap<String, Vec<String>>,
    pub authors: Vec<String>,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub documentation: Option<String>,
    pub readme: Option<String>,
    pub readme_file: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    pub license: Option<String>,
    pub license_file: Option<String>,
    pub repository: Option<String>,
    pub badges: Option<HashMap<String, HashMap<String, String>>>,
    pub links: Option<String>,
}

// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct CrateDependency {
//     pub name: String,
//     pub version_req: VersionReq,
//     pub features: Vec<String>,
//     pub optional: bool,
//     pub default_features: bool,
//     pub target: Option<String>,
//     pub kind: Option<CrateDependencyKind>,
//     pub registry: Option<String>,
//     #[serde(rename = "explicit_name_in_toml")]
//     pub explicit_name: Option<String>,
// }

// #[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "snake_case")]
// pub enum CrateDependencyKind {
//     /// A normal dependency.
//     Normal,
//     /// A build dependency.
//     Build,
//     /// A developement dependency.
//     Dev,
// }

impl From<CrateMetadata> for CrateVersion {
    fn from(value: CrateMetadata) -> Self {
        Self {
            name: value.name,
            vers: value.vers,
            deps: value.deps,
            cksum: "FIXME".into(),
            features: value.features,
            yanked: None,
            links: value.links,
        }
    }
}
