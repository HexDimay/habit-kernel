use std::hash::{DefaultHasher, Hash, Hasher};

use crate::habit::Habit;

pub mod traits;

#[derive(Debug, serde::Serialize, serde::Deserialize, Hash)]
pub struct DataBase {
    pub habit_list: Vec<Habit>,
}

impl DataBase {
    pub fn new() -> Self {
        Self {
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

    pub fn update(&mut self) -> anyhow::Result<()> {
        let mut old_hasher = DefaultHasher::new();
        let mut current_hasher = DefaultHasher::new();
        let old = Self::load()?.unwrap();

        old.hash(&mut old_hasher);
        self.hash(&mut current_hasher);

        if old_hasher.finish() != current_hasher.finish() {
            self.save()?;
        }

        Ok(())
    }

    pub fn add_habit(&mut self, name: String) {
        let mut habit = Habit::new();
        habit.mut_metadata().set_name(name);

        self.habit_list.push(habit);
    }
}