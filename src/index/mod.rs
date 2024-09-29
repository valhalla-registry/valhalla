use crate::models::crates::CrateVersion;
use error::Error;
use git::GitIndex;
use semver::{Version, VersionReq};
use sparse::SparseIndex;

pub mod error;
pub mod git;
pub mod models;
pub mod sparse;
pub mod tree;

pub trait IndexTrait {
    /// Adds a new crate record into the index.
    fn add_record(&self, record: CrateVersion) -> Result<(), Error>;
    /// Retrieves all the version records of a crate.
    fn all_records(&self, name: &str) -> Result<Vec<CrateVersion>, Error>;
    /// Retrieves the latest version record of a crate.
    fn latest_record(&self, name: &str) -> Result<CrateVersion, Error>;
    /// Retrieves the latest crate version record that matches the given name and version requirement.
    fn match_record(&self, name: &str, req: VersionReq) -> Result<CrateVersion, Error>;
    /// Alters an index's crate version record with the passed-in function.
    fn alter_record<F>(&self, name: &str, version: Version, func: F) -> Result<(), Error>
    where
        F: FnOnce(&mut CrateVersion);
    /// Yanks a crate version.
    fn yank_record(&self, name: &str, version: Version) -> Result<(), Error> {
        self.alter_record(name, version, |krate| krate.yanked = true)
    }
    /// Un-yanks a crate version.
    fn unyank_record(&self, name: &str, version: Version) -> Result<(), Error> {
        self.alter_record(name, version, |krate| krate.yanked = false)
    }
}

#[derive(Debug)]
pub enum Index
where
    GitIndex: IndexTrait,
    SparseIndex: IndexTrait,
{
    Git(GitIndex),
    Sparse(SparseIndex),
}

impl IndexTrait for Index {
    fn add_record(&self, record: CrateVersion) -> Result<(), Error> {
        tracing::debug!("adding record!");
        match self {
            Self::Git(idx) => idx.add_record(record),
            Self::Sparse(idx) => idx.add_record(record),
        }
    }

    fn all_records(&self, name: &str) -> Result<Vec<CrateVersion>, Error> {
        match self {
            Self::Git(idx) => idx.all_records(name),
            Self::Sparse(idx) => idx.all_records(name),
        }
    }

    fn latest_record(&self, name: &str) -> Result<CrateVersion, Error> {
        match self {
            Self::Git(idx) => idx.latest_record(name),
            Self::Sparse(idx) => idx.latest_record(name),
        }
    }

    fn match_record(&self, name: &str, req: VersionReq) -> Result<CrateVersion, Error> {
        match self {
            Self::Git(idx) => idx.match_record(name, req),
            Self::Sparse(idx) => idx.match_record(name, req),
        }
    }

    fn alter_record<F>(&self, name: &str, version: Version, func: F) -> Result<(), Error>
    where
        F: FnOnce(&mut CrateVersion),
    {
        match self {
            Self::Git(idx) => idx.alter_record(name, version, func),
            Self::Sparse(idx) => idx.alter_record(name, version, func),
        }
    }
}
