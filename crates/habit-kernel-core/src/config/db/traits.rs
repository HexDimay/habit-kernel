//! Общие трейты конфигов баз данных.
//!
//! Задача — позволить библиотеке работать с абстракцией «хранилище
//! записей о БД», не привязываясь к конкретной структуре.
//! Внутренние подсистемы (менеджер подключений, CLI, сериализаторы)
//! принимают `impl DbStore`/`impl DbStoreMut` и не знают, откуда взялся
//! список.

use std::path::{Path, PathBuf};
use uuid::Uuid;

use super::err::DbConfigError;

/// Read-only view of a single database entry.
pub trait DbEntry {
    /// Конкретный тип пути. `AsRef<Path>` — минимум, на который опирается
    /// любой потребитель (например, чтобы открыть файл БД).
    type Path: AsRef<Path>;

    fn id(&self) -> Uuid;
    fn name(&self) -> &str;
    fn path_file(&self) -> &Self::Path;
}

/// Mutable view of a single database entry.
///
/// Сеттера `id` здесь нет сознательно: id — первичный ключ, на который
/// опирается `current_db` хранилища. Если бы запись могла менять id на
/// месте, выбранная БД превратилась бы в висячую ссылку.
pub trait DbEntryMut: DbEntry {
    fn set_name(&mut self, name: impl Into<String>);
    fn set_path_file(&mut self, path: Self::Path);
}

/// Read-only view of a store of database entries.
pub trait DbStore {
    type Path: AsRef<Path>;
    type Entry: DbEntry<Path = Self::Path>;

    // --- обязательно к реализации ---

    fn path_dir(&self) -> &Self::Path;
    fn databases(&self) -> &[Self::Entry];
    fn current_db_id(&self) -> Option<Uuid>;

    // --- provided methods ---

    fn current_db(&self) -> Option<&Self::Entry> {
        self.current_db_id().and_then(|id| self.find_by_id(id))
    }

    fn len(&self) -> usize {
        self.databases().len()
    }

    fn is_empty(&self) -> bool {
        self.databases().is_empty()
    }

    fn find_by_id(&self, id: Uuid) -> Option<&Self::Entry> {
        self.databases().iter().find(|e| e.id() == id)
    }

    fn find_by_name(&self, name: &str) -> Option<&Self::Entry> {
        self.databases().iter().find(|e| e.name() == name)
    }
}

/// Mutable view of a store of database entries.
///
/// Контракт: реализация обязуется поддерживать инвариант «`current_db_id()`
/// указывает на существующую запись». Provided-методы его соблюдают;
/// required-методы — на совести реализации.
pub trait DbStoreMut: DbStore {
    // --- обязательно к реализации ---

    fn set_path_dir(&mut self, path_dir: Self::Path);

    /// Мутабельный срез записей. Слайс не умеет менять длину, поэтому
    /// добавление/удаление через него невозможно — инвариант числа записей
    /// сохраняется. Для мутации содержимого конкретной записи используйте
    /// [`DbEntryMut`].
    fn databases_mut(&mut self) -> &mut [Self::Entry];

    fn add(
        &mut self,
        name: impl Into<String>,
        path_file: Self::Path,
    ) -> Result<Uuid, DbConfigError>;

    fn add_with_default_path(&mut self, name: impl Into<String>) -> Result<Uuid, DbConfigError>
    where
        Self::Path: From<PathBuf>;

    fn remove_by_id(&mut self, id: Uuid) -> Result<Self::Entry, DbConfigError>;

    fn remove_by_name(&mut self, name: &str) -> Result<Self::Entry, DbConfigError>;

    fn select_by_id(&mut self, id: Uuid) -> Result<(), DbConfigError>;

    fn select_by_name(&mut self, name: &str) -> Result<(), DbConfigError>;

    fn clear_current(&mut self);

    // --- provided methods ---

    fn find_by_id_mut(&mut self, id: Uuid) -> Option<&mut Self::Entry> {
        self.databases_mut().iter_mut().find(|e| e.id() == id)
    }

    fn find_by_name_mut(&mut self, name: &str) -> Option<&mut Self::Entry> {
        self.databases_mut().iter_mut().find(|e| e.name() == name)
    }
}
