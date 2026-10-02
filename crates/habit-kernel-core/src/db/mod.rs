use crate::{
    db::traits::{DataBaseMetadata, DataBaseMetadataMut, QueryExecutor},
    habit::Habit,
    io::{
        GetPath, Load, Save,
        atomic::{read_to_string, write_atomic},
    },
};
use serde::de::DeserializeOwned;

pub mod traits;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DataBase {
    current_habit: Option<uuid::Uuid>,
    habit_list: Vec<Habit>,
}

impl DataBase {
    pub fn new() -> Self {
        Self {
            current_habit: None,
            habit_list: Vec::new(),
        }
    }

    pub fn get_current_habit(&self) -> Option<&Habit> {
        if let Some(id) = self.current_habit {
            return self.get_by_id(id);
        }

        None
    }

    pub fn get_mut_current_habit(&mut self) -> Option<&mut Habit> {
        if let Some(id) = self.current_habit {
            return self.get_mut_by_id(id);
        }

        None
    }

    pub fn get_by_idx(&self, index: usize) -> Option<&Habit> {
        self.habit_list.get(index)
    }

    pub fn get_mut_by_idx(&mut self, index: usize) -> Option<&mut Habit> {
        self.habit_list.get_mut(index)
    }

    pub fn get_by_id(&self, id: uuid::Uuid) -> Option<&Habit> {
        self.iter().find(|h| h.metadata().id() == id)
    }

    pub fn get_mut_by_id(&mut self, id: uuid::Uuid) -> Option<&mut Habit> {
        self.iter_mut().find(|h| h.metadata().id() == id)
    }

    /// Save to db.json file and load from db.json file
    pub const fn save_load_path() -> &'static str {
        "./db.json"
    }

    pub fn save(&self) -> anyhow::Result<()> {
        <Self as Save<&'static str>>::save(self)
    }

    pub fn load() -> anyhow::Result<Option<Self>> {
        if !std::fs::exists(Self::save_load_path())? {
            return Ok(None);
        }

        Ok(Some(<Self as Load<&'static str>>::load::<Self>()?))
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Habit> {
        self.habit_list.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Habit> {
        self.habit_list.iter_mut()
    }

    pub fn add_habit(&mut self, name: &str) {
        let mut habit = Habit::new();
        habit.mut_metadata().set_name(name);

        self.habit_list.push(habit);
    }

    pub fn select_habit(&mut self, id: uuid::Uuid) {
        if self.get_by_id(id).is_some() {
            self.current_habit = Some(id);
            return;
        }

        self.current_habit = None;
    }

    pub fn current_done(&mut self) {
        if let Some(c_h) = self.get_mut_current_habit() {
            c_h.increment();
        }
    }

    pub fn del_by_id(&mut self, id: uuid::Uuid) -> Option<Habit> {
        for (idx, habit) in self.iter().enumerate() {
            if habit.metadata().id() == id {
                return Some(self.habit_list.remove(idx));
            }
        }

        None
    }
}

// ---------------------------------------------------------------------
// Делегирование трейтовых операций во внутренние inherent-методы.
//
// Inherent-методы остаются публичным API (работают без импорта трейтов),
// а impl'ы трейтов переадресуют в них. Вызовы внутри impl'ов
// тип-квалифицированы (`DataBase::get_by_id(self, id)`), чтобы Rust
// гарантированно выбрал inherent-метод, а не рекурсивно вернулся в трейт.
// ---------------------------------------------------------------------
impl DataBaseMetadata for DataBase {
    fn get_current_habit(&self) -> Option<&Habit> {
        DataBase::get_current_habit(self)
    }

    fn get_by_idx(&self, index: usize) -> Option<&Habit> {
        DataBase::get_by_idx(self, index)
    }

    fn get_by_id(&self, id: uuid::Uuid) -> Option<&Habit> {
        DataBase::get_by_id(self, id)
    }

    fn iter(&self) -> std::slice::Iter<'_, Habit> {
        DataBase::iter(self)
    }
}

impl DataBaseMetadataMut for DataBase {
    fn get_mut_current_habit(&mut self) -> Option<&mut Habit> {
        DataBase::get_mut_current_habit(self)
    }

    fn get_mut_by_idx(&mut self, index: usize) -> Option<&mut Habit> {
        DataBase::get_mut_by_idx(self, index)
    }

    fn get_mut_by_id(&mut self, id: uuid::Uuid) -> Option<&mut Habit> {
        DataBase::get_mut_by_id(self, id)
    }

    fn iter_mut(&mut self) -> std::slice::IterMut<'_, Habit> {
        DataBase::iter_mut(self)
    }
}

impl QueryExecutor<&'static str> for DataBase {
    fn add_habit(&mut self, name: &str) {
        DataBase::add_habit(self, name)
    }

    fn select_habit(&mut self, id: uuid::Uuid) {
        DataBase::select_habit(self, id)
    }

    fn current_done(&mut self) {
        DataBase::current_done(self)
    }

    fn del_by_id(&mut self, id: uuid::Uuid) {
        // Inherent-версия возвращает удалённый `Habit`; трейтовая по
        // контракту ничего не возвращает, поэтому результат отбрасываем.
        let _ = DataBase::del_by_id(self, id);
    }
}

// ---------------------------------------------------------------------
// Файловый IO. `DataBase` сериализуется в JSON по пути `./db.json`;
// запись идёт атомарно (см. `io::atomic`).
// ---------------------------------------------------------------------
impl GetPath<&'static str> for DataBase {
    fn get_path() -> &'static str {
        DataBase::save_load_path()
    }
}

impl Save<&'static str> for DataBase {
    fn save(&self) -> anyhow::Result<()> {
        let path = <Self as GetPath<&'static str>>::get_path();
        let db = serde_json::to_string(self)?;

        write_atomic(path, &db)?;

        Ok(())
    }
}

impl Load<&'static str> for DataBase {
    fn load<T>() -> anyhow::Result<T>
    where
        T: DeserializeOwned + GetPath<&'static str>,
    {
        let path = T::get_path();
        let content = read_to_string(path)?;

        Ok(serde_json::from_str(&content)?)
    }
}
