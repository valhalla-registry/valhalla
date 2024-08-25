use semver::Version;

#[derive(Debug, Clone)]
pub enum Index {
    Git,
    Sparse,
}
