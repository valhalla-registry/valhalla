use std::{
    net::Ipv4Addr,
    path::{Path, PathBuf},
};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
}

impl Config {
    pub fn load(path: impl AsRef<Path>) -> Self {
        let content = std::fs::read_to_string(path).unwrap();
        toml::from_str(&content).unwrap()
    }
}

#[derive(Deserialize)]
pub struct ServerConfig {
    pub ip: Ipv4Addr,
    pub port: u16,
    pub assets_dir: PathBuf,
}
