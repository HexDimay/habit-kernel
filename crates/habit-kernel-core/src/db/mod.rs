use crate::{
    db::traits::{DataBaseMetadata, DataBaseMetadataMut, QueryExecutor},
    habit::{Habit, traits::HabitEntity},
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

    /// Путь к файлу БД. Трейтом не описан, но на него опирается [`GetPath`].
    pub const fn save_load_path() -> &'static str {
        "./db.json"
    }
}

// ---------------------------------------------------------------------
// Логика доступа и изменений живёт в трейтах. Дублирующих inherent-методов
// нет: единственный источник правды — эти impl'ы.
// ---------------------------------------------------------------------
impl DataBaseMetadata for DataBase {
    fn get_current_habit(&self) -> Option<&Habit> {
        let id = self.current_habit?;
        self.get_by_id(id)
    }

    fn get_by_idx(&self, index: usize) -> Option<&Habit> {
        self.habit_list.get(index)
    }

    fn get_by_id(&self, id: uuid::Uuid) -> Option<&Habit> {
        self.iter().find(|habit| habit.id() == id)
    }

    fn iter(&self) -> std::slice::Iter<'_, Habit> {
        self.habit_list.iter()
    }
}

impl DataBaseMetadataMut for DataBase {
    fn get_mut_current_habit(&mut self) -> Option<&mut Habit> {
        let id = self.current_habit?;
        self.get_mut_by_id(id)
    }

    fn get_mut_by_idx(&mut self, index: usize) -> Option<&mut Habit> {
        self.habit_list.get_mut(index)
    }

    fn get_mut_by_id(&mut self, id: uuid::Uuid) -> Option<&mut Habit> {
        self.iter_mut().find(|habit| habit.id() == id)
    }

    fn iter_mut(&mut self) -> std::slice::IterMut<'_, Habit> {
        self.habit_list.iter_mut()
    }
}

impl QueryExecutor<&'static str> for DataBase {
    fn add_habit(&mut self, name: &str) {
        let mut habit = Habit::new();
        habit.set_name(name);

        self.habit_list.push(habit);
    }

    fn select_habit(&mut self, id: uuid::Uuid) {
        self.current_habit = if self.get_by_id(id).is_some() {
            Some(id)
        } else {
            None
        };
    }

    fn current_done(&mut self) {
        if let Some(habit) = self.get_mut_current_habit() {
            habit.increment();
        }
    }

    fn del_by_id(&mut self, id: uuid::Uuid) -> Option<Habit> {
        let idx = self.iter().position(|habit| habit.id() == id)?;
        Some(self.habit_list.remove(idx))
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
