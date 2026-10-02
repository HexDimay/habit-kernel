use std::collections::HashMap;

use chrono::NaiveDate;

use crate::habit::traits::{DataContainer, ValueEntity};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Data {
    pub(crate) done: HashMap<NaiveDate, Value>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            done: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Value(pub(crate) usize);

impl Value {
    pub fn new(value: usize) -> Self {
        Self(value)
    }
}

impl std::ops::Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Value {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0.saturating_sub(rhs.0))
    }
}

impl std::ops::AddAssign for Value {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl std::ops::SubAssign for Value {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = self.0.saturating_sub(rhs.0);
    }
}

// ---------------------------------------------------------------------
// Логика `Value` живёт в трейте `ValueEntity`: дублирующих inherent-методов
// нет, чтобы не было двух источников правды.
// ---------------------------------------------------------------------
impl ValueEntity<usize> for Value {
    fn get(&self) -> usize {
        self.0
    }

    fn set(&mut self, new_v: usize) {
        self.0 = new_v;
    }

    fn zeroing(&mut self) {
        self.0 = 0;
    }
}

// ---------------------------------------------------------------------
// Логика `Data` живёт в трейте `DataContainer`.
// ---------------------------------------------------------------------
impl DataContainer<Value, usize> for Data {
    fn get(&self, date: NaiveDate) -> Option<&Value> {
        self.done.get(&date)
    }

    fn get_mut(&mut self, date: NaiveDate) -> Option<&mut Value> {
        self.done.get_mut(&date)
    }

    fn add_now(&mut self, value: Value) {
        let date = chrono::Local::now().date_naive();
        self.add(date, value);
    }

    fn add(&mut self, date: NaiveDate, value: Value) {
        if let Some(v) = self.done.get_mut(&date) {
            *v += value;
            return;
        }

        self.done.insert(date, value);
    }

    fn decrement(&mut self, date: NaiveDate) {
        if let Some(v) = self.done.get_mut(&date) {
            *v -= Value(1);
        }
    }

    fn zeroing_by_date(&mut self, date: NaiveDate) {
        if let Some(v) = self.done.get_mut(&date) {
            v.zeroing();
        }
    }

    fn count_current_done(&self) -> usize {
        if let Some(n) = self.done.get(&chrono::Local::now().date_naive()) {
            return n.get();
        }

        0
    }
}
