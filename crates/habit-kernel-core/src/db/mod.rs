use std::hash::Hash;

use anyhow::bail;

use crate::habit::Habit;

pub mod traits;

#[derive(Debug, serde::Serialize, serde::Deserialize, Hash)]
pub struct DataBase {
    pub current_habit: Option<Habit>,
    pub habit_list: Vec<Habit>,
}

impl DataBase {
    pub fn new() -> Self {
        Self {
            current_habit: None,
            habit_list: Vec::new(),
        }
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

    // pub fn update(&mut self) -> anyhow::Result<()> {
    //     let mut old_hasher = DefaultHasher::new();
    //     let mut current_hasher = DefaultHasher::new();
    //     let old = Self::load()?.unwrap();

    //     old.hash(&mut old_hasher);
    //     self.hash(&mut current_hasher);

    //     if old_hasher.finish() != current_hasher.finish() {
    //         self.save()?;
    //     }

    //     Ok(())
    // }

    pub fn iter(&self) -> std::slice::Iter<'_, Habit> {
        self.habit_list.iter()
    }

    pub fn add_habit(&mut self, name: &str) {
        let mut habit = Habit::new();
        habit.mut_metadata().set_name(name);

        self.habit_list.push(habit);
    }

    /// Это действие равносильно удалению из основного списка.
    pub fn select_habit(&mut self, id: uuid::Uuid) -> anyhow::Result<()> {
        if let Some(_) = self.current_habit {
            self.habit_list.push(self.current_habit.take().unwrap());
        }

        self.current_habit = self.del_by_id(id);

        if self.current_habit.is_none() {
            bail!("Couldn't choose a habit.");
        }

        Ok(())
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
