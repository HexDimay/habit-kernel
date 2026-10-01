use crate::{
    config::db::DataBaseConfig,
    io::{
        GetPath, Load, Save,
        atomic::{read_to_string, write_atomic},
    },
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::path::PathBuf;

pub mod db;
#[cfg(test)]
pub mod tests;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    database: DataBaseConfig<PathBuf>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            database: DataBaseConfig::new("./db".into()),
        }
    }

    /// Пытается загрузить конфиг с диска; если не удалось - создаёт новый
    /// и сохраняет его.
    pub fn init() -> Self {
        match <Self as Load<&str>>::load::<Self>() {
            Ok(cfg) => cfg,
            Err(err) => {
                eprintln!("config load failed: {err:#}, using defaults");
                let cfg = Self::new();
                // сохраняем best-effort, не роняем приложение из-за этого
                if let Err(e) = cfg.save() {
                    eprintln!("config save failed: {e:#}");
                }
                cfg
            }
        }
    }
}

impl GetPath<&str> for Config {
    fn get_path() -> &'static str {
        "./app.toml"
    }
}

impl Save<&str> for Config {
    fn save(&self) -> anyhow::Result<()> {
        let path = <Self as GetPath<&str>>::get_path();
        let data = basic_toml::to_string(self)?;
        write_atomic(path, &data)?;
        Ok(())
    }
}

impl Load<&'static str> for Config {
    fn load<T>() -> anyhow::Result<T>
    where
        T: DeserializeOwned + GetPath<&'static str>,
    {
        let path = T::get_path();
        let content = read_to_string(path)?;
        Ok(basic_toml::from_str(&content)?)
    }
}
