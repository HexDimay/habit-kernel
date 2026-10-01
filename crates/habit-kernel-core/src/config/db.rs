use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DataBaseConfig<P: AsRef<Path>> {
    pub path_dir: P,
    pub current_db: Option<CurrentDataBaseConfig<P>>,
}

impl<P: AsRef<Path>> DataBaseConfig<P> {
    pub fn new(path_dir: P) -> Self {
        Self {
            path_dir,
            current_db: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentDataBaseConfig<P: AsRef<Path>> {
    pub name: String,
    pub path_file: P,
}
