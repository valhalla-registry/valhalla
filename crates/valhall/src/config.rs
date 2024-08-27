#![allow(unused)] // FIXME: remove this whene functionality is implemented

use std::{
    net::Ipv4Addr,
    path::{Path, PathBuf},
};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub index: IndexConfig,
    pub storage: StorageConfig,
    pub frontend: FrontendConfig,
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
}

#[derive(Debug, Deserialize)]
pub struct IndexConfig {
    pub path: PathBuf,
    pub git: bool,
    pub sparse: bool,
}

#[derive(Debug, Deserialize)]
pub struct StorageConfig {
    #[serde(rename = "type")]
    pub kind: String,
    pub path: PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct FrontendConfig {
    pub assets_dir: PathBuf,
}
