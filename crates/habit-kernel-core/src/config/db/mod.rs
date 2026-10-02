pub mod err;
#[cfg(test)]
pub mod tests;
pub mod traits;

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use uuid::Uuid;

use crate::config::db::err::DbConfigError;
pub use crate::config::db::traits::{DbEntry, DbEntryMut, DbStore, DbStoreMut};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataBaseConfig<P: AsRef<Path>> {
    path_dir: P,
    databases: Vec<CurrentDataBaseConfig<P>>,
    current_db: Option<uuid::Uuid>,
}

impl<P: AsRef<Path>> DataBaseConfig<P> {
    pub fn new(path_dir: P) -> Self {
        Self {
            path_dir,
            databases: Vec::new(),
            current_db: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrentDataBaseConfig<P: AsRef<Path>> {
    id: Uuid,
    name: String,
    path_file: P,
}

impl<P: AsRef<Path>> CurrentDataBaseConfig<P> {
    pub fn new(id: Uuid, name: impl Into<String>, path_file: P) -> Self {
        Self {
            id,
            name: name.into(),
            path_file,
        }
    }
}

// ---------------------------------------------------------------------
// Вся логика — в impl'ах трейтов; дублирующих inherent-методов (кроме
// конструкторов) нет. Так `DataBaseConfig`/`CurrentDataBaseConfig`
// описываются ровно одним набором контрактов.
// ---------------------------------------------------------------------

impl<P: AsRef<Path>> DbEntry for CurrentDataBaseConfig<P> {
    type Path = P;

    fn id(&self) -> Uuid {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn path_file(&self) -> &P {
        &self.path_file
    }
}

impl<P: AsRef<Path>> DbEntryMut for CurrentDataBaseConfig<P> {
    fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    fn set_path_file(&mut self, path: P) {
        self.path_file = path;
    }
}

impl<P: AsRef<Path>> DbStore for DataBaseConfig<P> {
    type Path = P;
    type Entry = CurrentDataBaseConfig<P>;

    fn path_dir(&self) -> &P {
        &self.path_dir
    }

    /// Список доступных БД — только для чтения. Мутабельный доступ идёт
    /// через [`DbStoreMut::databases_mut`].
    fn databases(&self) -> &[Self::Entry] {
        &self.databases
    }

    fn current_db_id(&self) -> Option<Uuid> {
        self.current_db
    }
}

impl<P: AsRef<Path>> DbStoreMut for DataBaseConfig<P> {
    /// Меняет каталог только для будущих `add_with_default_path`; пути уже
    /// существующих БД не перезаписываются — иначе это молчаливая потеря
    /// данных.
    fn set_path_dir(&mut self, path_dir: P) {
        self.path_dir = path_dir;
    }

    fn databases_mut(&mut self) -> &mut [Self::Entry] {
        &mut self.databases
    }

    /// Регистрирует БД с явным путём к файлу. Имя должно быть уникальным.
    fn add(&mut self, name: impl Into<String>, path_file: P) -> Result<Uuid, DbConfigError> {
        let name = name.into();
        if self.find_by_name(&name).is_some() {
            return Err(DbConfigError::NameAlreadyExists(name));
        }
        let id = Uuid::new_v4();
        self.databases.push(CurrentDataBaseConfig {
            id,
            name,
            path_file,
        });
        Ok(id)
    }

    /// Регистрирует БД с путём по умолчанию: `<path_dir>/<name>.db`.
    ///
    /// Требует `P: From<PathBuf>`, чтобы сконструировать значение нужного
    /// типа. Для `P = PathBuf` и `P = Box<Path>` подходит.
    fn add_with_default_path(&mut self, name: impl Into<String>) -> Result<Uuid, DbConfigError>
    where
        P: From<PathBuf>,
    {
        let name = name.into();
        let path_file = P::from(self.path_dir.as_ref().join(format!("{name}.db")));
        self.add(name, path_file)
    }

    /// Удаляет запись из конфига. **Файл БД не трогает** — разделение
    /// ответственности; при необходимости вызвать `fs::remove_file`
    /// отдельно по `removed.path_file()`.
    fn remove_by_id(&mut self, id: Uuid) -> Result<Self::Entry, DbConfigError> {
        let pos = self
            .databases
            .iter()
            .position(|d| d.id == id)
            .ok_or(DbConfigError::IdNotFound(id))?;
        let removed = self.databases.remove(pos);

        if self.current_db == Some(id) {
            self.current_db = None;
        }
        Ok(removed)
    }

    fn remove_by_name(&mut self, name: &str) -> Result<Self::Entry, DbConfigError> {
        let pos = self
            .databases
            .iter()
            .position(|d| d.name == name)
            .ok_or_else(|| DbConfigError::NameNotFound(name.to_string()))?;
        let removed = self.databases.remove(pos);

        if self.current_db == Some(removed.id) {
            self.current_db = None;
        }
        Ok(removed)
    }

    fn select_by_id(&mut self, id: Uuid) -> Result<(), DbConfigError> {
        if self.find_by_id(id).is_none() {
            return Err(DbConfigError::IdNotFound(id));
        }
        self.current_db = Some(id);
        Ok(())
    }

    fn select_by_name(&mut self, name: &str) -> Result<(), DbConfigError> {
        let id = self
            .find_by_name(name)
            .ok_or_else(|| DbConfigError::NameNotFound(name.to_string()))?
            .id;
        self.current_db = Some(id);
        Ok(())
    }

    fn clear_current(&mut self) {
        self.current_db = None;
    }
}
