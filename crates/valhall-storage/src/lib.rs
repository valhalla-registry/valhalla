use std::path::PathBuf;

use semver::Version;

#[derive(Debug, Clone)]
pub struct Storage {
    pub path: PathBuf,
}

pub struct Crate {
    pub name: String,
}

impl Storage {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn store_crate(&self, name: &str, version: &Version, bytes: &[u8]) -> std::io::Result<()> {
        let path = self
            .path
            .join(&name)
            .join(format!("{}-{}.crate", name, version));

        std::fs::create_dir_all(&path)?;
        std::fs::write(path, bytes)
    }

    pub fn get_all_crates(&self) -> Vec<Crate> {
        self.path
            .read_dir()
            .unwrap()
            .into_iter()
            .map(|e| {
                e.unwrap()
                    .path()
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_owned()
            })
            .map(|n| Crate { name: n })
            .collect()
    }
}
