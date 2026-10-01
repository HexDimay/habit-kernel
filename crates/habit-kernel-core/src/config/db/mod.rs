pub mod err;
#[cfg(test)]
pub mod tests;

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::db::err::DbConfigError;

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

    // getters

    pub fn path_dir(&self) -> &P {
        &self.path_dir
    }

    /// Список доступных БД — только для чтения. Мутабельный доступ не
    /// отдаём: он ломает инвариант «`current_db` ссылается на элемент из
    /// `databases`».
    pub fn databases(&self) -> &[CurrentDataBaseConfig<P>] {
        &self.databases
    }

    pub fn current_db_id(&self) -> Option<Uuid> {
        self.current_db
    }

    /// Выбранная БД целиком (или `None`, если ничего не выбрано).
    pub fn current_db(&self) -> Option<&CurrentDataBaseConfig<P>> {
        self.current_db.and_then(|id| self.find_by_id(id))
    }

    pub fn len(&self) -> usize {
        self.databases.len()
    }
    pub fn is_empty(&self) -> bool {
        self.databases.is_empty()
    }

    // setters

    /// Меняет каталог только для будущих `add_with_default_path`; пути уже
    /// существующих БД не перезаписываются — иначе это молчаливая потеря
    /// данных.
    pub fn set_path_dir(&mut self, path_dir: P) {
        self.path_dir = path_dir;
    }

    // finds

    pub fn find_by_id(&self, id: Uuid) -> Option<&CurrentDataBaseConfig<P>> {
        self.databases.iter().find(|d| d.id == id)
    }

    pub fn find_by_id_mut(&mut self, id: Uuid) -> Option<&mut CurrentDataBaseConfig<P>> {
        self.databases.iter_mut().find(|d| d.id == id)
    }

    pub fn find_by_name(&self, name: &str) -> Option<&CurrentDataBaseConfig<P>> {
        self.databases.iter().find(|d| d.name == name)
    }

    pub fn find_by_name_mut(&mut self, name: &str) -> Option<&mut CurrentDataBaseConfig<P>> {
        self.databases.iter_mut().find(|d| d.name == name)
    }

    // add

    /// Регистрирует БД с явным путём к файлу. Имя должно быть уникальным.
    pub fn add(&mut self, name: impl Into<String>, path_file: P) -> Result<Uuid, DbConfigError> {
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
    pub fn add_with_default_path(&mut self, name: impl Into<String>) -> Result<Uuid, DbConfigError>
    where
        P: From<PathBuf>,
    {
        let name = name.into();
        let path_file = P::from(self.path_dir.as_ref().join(format!("{name}.db")));
        self.add(name, path_file)
    }

    // del and remove

    /// Удаляет запись из конфига. **Файл БД не трогает** — разделение
    /// ответственности; при необходимости вызвать `fs::remove_file`
    /// отдельно по `removed.path_file()`.
    pub fn remove_by_id(&mut self, id: Uuid) -> Result<CurrentDataBaseConfig<P>, DbConfigError> {
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

    pub fn remove_by_name(
        &mut self,
        name: &str,
    ) -> Result<CurrentDataBaseConfig<P>, DbConfigError> {
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

    // select

    pub fn select_by_id(&mut self, id: Uuid) -> Result<(), DbConfigError> {
        if self.find_by_id(id).is_none() {
            return Err(DbConfigError::IdNotFound(id));
        }
        self.current_db = Some(id);
        Ok(())
    }

    pub fn select_by_name(&mut self, name: &str) -> Result<(), DbConfigError> {
        let id = self
            .find_by_name(name)
            .ok_or_else(|| DbConfigError::NameNotFound(name.to_string()))?
            .id;
        self.current_db = Some(id);
        Ok(())
    }

    pub fn clear_current(&mut self) {
        self.current_db = None;
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

    // getters

    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn path_file(&self) -> &P {
        &self.path_file
    }

    // setters

    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }
    pub fn set_path_file(&mut self, path: P) {
        self.path_file = path;
    }
}
