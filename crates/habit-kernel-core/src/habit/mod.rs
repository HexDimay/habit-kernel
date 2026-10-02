use chrono::NaiveDate;

use crate::habit::{
    data::{Data, Value},
    metadata::Metadata,
    traits::{DataContainer, HabitEntity, Limitation, ValueEntity},
};

pub mod data;
pub mod metadata;
pub mod traits;

#[cfg(test)]
pub mod tests;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Habit {
    metadata: Metadata,
    data: Data,
}

impl Habit {
    pub fn new() -> Self {
        Self {
            metadata: Metadata::new(),
            data: Data::new(),
        }
    }

    /// Прямой доступ к метаданным. `Metadata` не описан ни одним трейтом —
    /// в `HabitEntity` попали только его проекции, — поэтому оставляем
    /// read-only доступ к конкретному типу (нужен, например, для
    /// `LimitationValue`). Все изменения идут через [`HabitEntity`].
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

// ---------------------------------------------------------------------
// Логика привычки живёт в трейте `HabitEntity`. Параметры выбраны по
// факту: счётчик — `usize`, значения — `Value`, контейнер — `Data`.
// ---------------------------------------------------------------------
impl HabitEntity<usize, Value, Data> for Habit {
    fn id(&self) -> uuid::Uuid {
        self.metadata.id()
    }

    fn name(&self) -> &str {
        self.metadata.name()
    }

    fn set_name(&mut self, new_name: &str) {
        self.metadata.set_name(new_name);
    }

    fn created(&self) -> &chrono::NaiveDate {
        self.metadata.created()
    }

    fn limitation_value(&self) -> impl Limitation<usize> {
        self.metadata.limitation_value()
    }

    fn set_limitation_value(&mut self, new_limit: impl Limitation<usize>) {
        // `LimitationValue` хранит только верхнюю границу, поэтому нижнюю
        // границу (`min`) сохранить некуда — при конвертации она теряется.
        let value = if new_limit.is_lim() {
            metadata::LimitationValue::Max(new_limit.max())
        } else {
            metadata::LimitationValue::Unlimited
        };

        self.metadata.set_limitation_value(value);
    }

    fn current_time(&self) -> NaiveDate {
        self.metadata.current_time()
    }

    fn update_current_time(&mut self) {
        self.metadata.update_current_time();
    }

    fn increment(&mut self) {
        let date = self.metadata.current_time();
        self.data.add(date, Value(1));

        if let Some(v) = self.data.get(date) {
            if let metadata::LimitationValue::Max(max) = self.metadata.limitation_value() {
                if v.get() > max {
                    self.data.zeroing_by_date(date);
                }
            }
        }
    }

    fn increment_by_date(&mut self, date: NaiveDate) {
        self.data.add(date, Value(1));

        if let Some(v) = self.data.get(date) {
            if let metadata::LimitationValue::Max(max) = self.metadata.limitation_value() {
                if v.get() > max {
                    self.data.zeroing_by_date(date);
                }
            }
        }
    }

    fn decrement(&mut self) {
        let date = self.metadata.current_time();
        self.data.decrement(date);
    }

    fn decrement_by_date(&mut self, date: NaiveDate) {
        self.data.decrement(date);
    }

    fn data(&self) -> &Data {
        &self.data
    }

    fn mut_data(&mut self) -> &mut Data {
        &mut self.data
    }
}
