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

    pub fn get_current_habit(&self) -> Option<&Habit> {
        self.current_habit.as_ref()
    }

    pub fn get_mut_current_habit(&mut self) -> Option<&mut Habit> {
        self.current_habit.as_mut()
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

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Habit> {
        self.habit_list.iter_mut()
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
