#![allow(unused)]

use super::{error::Error, IndexTrait};
use semver::{Version, VersionReq};
use valhall_models::crates::CrateVersion;

#[derive(Debug)]
pub struct SparseIndex {}

impl IndexTrait for SparseIndex {
    fn add_record(&self, record: CrateVersion) -> Result<(), Error> {
        todo!()
    }

    fn all_records(&self, name: &str) -> Result<Vec<CrateVersion>, Error> {
        todo!()
    }

    fn latest_record(&self, name: &str) -> Result<CrateVersion, Error> {
        todo!()
    }

    fn match_record(&self, name: &str, req: VersionReq) -> Result<CrateVersion, Error> {
        todo!()
    }

    fn alter_record<F>(&self, name: &str, version: Version, func: F) -> Result<(), Error>
    where
        F: FnOnce(&mut CrateVersion),
    {
        todo!()
    }
}
