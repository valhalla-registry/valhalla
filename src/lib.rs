pub mod api;
pub mod app;
mod auth;
// pub mod config;
pub mod db;
mod error;
pub mod frontend;
// mod index;
// pub mod models;

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
