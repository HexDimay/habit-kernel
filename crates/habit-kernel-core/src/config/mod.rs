use crate::config::db::{
    CurrentDataBaseConfig,
    err::DbConfigError,
    traits::{DbStore, DbStoreMut},
};
use crate::{
    config::db::DataBaseConfig,
    io::{
        GetPath, Load, Save,
        atomic::{read_to_string, write_atomic},
    },
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::path::PathBuf;
use uuid::Uuid;

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

    /// Доступ к конфигу БД целиком.
    pub fn database(&self) -> &DataBaseConfig<PathBuf> {
        &self.database
    }

    /// Мутабельный доступ к конфигу БД.
    ///
    /// В отличие от `&mut Config as DbStoreMut`, этот метод обходит
    /// инвариант «`current_db` указывает на существующую запись» — при
    /// прямых манипуляциях с `databases` его легко нарушить. Предпочитайте
    /// трейт `DbStoreMut`; этот метод — для случаев, когда тип нужен
    /// явно (сериализация, тесты, миграции).
    pub fn database_mut(&mut self) -> &mut DataBaseConfig<PathBuf> {
        &mut self.database
    }
}

// ---------------------------------------------------------------------
// Делегирование общих трейтов во внутренний DataBaseConfig.
//
// Config здесь — фасад: он не добавляет семантики поверх DataBaseConfig,
// а лишь даёт единый тип верхнего уровня. Любое поведение (add/remove/
// select/find/…) живёт в трейтах и обеспечивается impl'ом для
// DataBaseConfig; Config просто переадресует.
// ---------------------------------------------------------------------
impl DbStore for Config {
    type Path = PathBuf;
    type Entry = CurrentDataBaseConfig<PathBuf>;

    fn path_dir(&self) -> &PathBuf {
        self.database.path_dir()
    }

    fn databases(&self) -> &[Self::Entry] {
        self.database.databases()
    }

    fn current_db_id(&self) -> Option<Uuid> {
        self.database.current_db_id()
    }
}

impl DbStoreMut for Config {
    fn set_path_dir(&mut self, path_dir: PathBuf) {
        self.database.set_path_dir(path_dir);
    }

    fn databases_mut(&mut self) -> &mut [Self::Entry] {
        self.database.databases_mut()
    }

    fn add(&mut self, name: impl Into<String>, path_file: PathBuf) -> Result<Uuid, DbConfigError> {
        self.database.add(name, path_file)
    }

    fn add_with_default_path(&mut self, name: impl Into<String>) -> Result<Uuid, DbConfigError>
    where
        PathBuf: From<PathBuf>,
    {
        self.database.add_with_default_path(name)
    }

    fn remove_by_id(&mut self, id: Uuid) -> Result<Self::Entry, DbConfigError> {
        self.database.remove_by_id(id)
    }

    fn remove_by_name(&mut self, name: &str) -> Result<Self::Entry, DbConfigError> {
        self.database.remove_by_name(name)
    }

    fn select_by_id(&mut self, id: Uuid) -> Result<(), DbConfigError> {
        self.database.select_by_id(id)
    }

    fn select_by_name(&mut self, name: &str) -> Result<(), DbConfigError> {
        self.database.select_by_name(name)
    }

    fn clear_current(&mut self) {
        self.database.clear_current();
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
