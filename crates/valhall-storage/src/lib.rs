use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Storage {
    pub path: PathBuf,
}

impl Storage {
    pub fn store_crate(self, name: String, version: String, bytes: &[u8]) {
        let path = self.path.join(format!("{}-{}.crate", name, version));
        std::fs::write(path, bytes).unwrap();
    }
}
