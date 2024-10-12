use semver::{Version, VersionReq};
use serde_with::{DeserializeFromStr, SerializeDisplay};

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

impl std::fmt::Display for ReqVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReqVersion::Exact(version) => version.fmt(f),
            ReqVersion::Semver(version_req) => version_req.fmt(f),
            ReqVersion::Latest => write!(f, "latest"),
        }
    }
}

impl std::str::FromStr for ReqVersion {
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
