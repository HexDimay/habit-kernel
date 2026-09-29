use std::hash::Hash;

use crate::habit::Habit;

pub mod traits;

#[derive(Debug, serde::Serialize, serde::Deserialize, Hash)]
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
        let db = serde_json::to_string(self)?;

        std::fs::write(Self::save_load_path(), db)?;

        Ok(())
    }

    pub fn load() -> anyhow::Result<Option<Self>> {
        if !std::fs::exists(Self::save_load_path())? {
            return Ok(None);
        }

        let s = std::fs::read_to_string(Self::save_load_path().to_owned())?;

        Ok(Some(serde_json::from_str(&s)?))
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

    pub fn del_by_id(&mut self, id: uuid::Uuid) -> Option<Habit> {
        for (idx, habit) in self.iter().enumerate() {
            if habit.metadata().id() == id {
                return Some(self.habit_list.remove(idx));
            }
        }

        None
    }
}
