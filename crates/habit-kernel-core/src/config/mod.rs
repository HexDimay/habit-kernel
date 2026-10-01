use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{config::db::DataBaseConfig, io::GetPath};

pub mod db;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    database: DataBaseConfig<PathBuf>
}

impl Config {
    pub fn new() -> Self {
        Self {
            database: DataBaseConfig::new("./db".into())
        }
    }

    pub fn init() -> Self {
        todo!()
    }
}

impl GetPath<&str> for Config {
    fn get_path() -> &'static str {
        "./app.toml"
    }
}