use chrono::NaiveDate;

use crate::habit::{
    data::{Data, Value},
    metadata::Metadata,
};

#[macro_use]
pub mod data;
pub mod metadata;

#[derive(Debug, serde::Serialize, serde::Deserialize, Hash)]
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

    pub fn update(&mut self) {
        self.metadata.update_current_time();
    }

    pub fn increment(&mut self) {
        let current_date = self.metadata().current_time();
        self.data.add(current_date, Value(1));

        if let Some(v) = self.data.get(current_date) {
            match self.metadata().limitation_value() {
                metadata::LimitationValue::Max(max) => {
                    if v.0 > max {
                        self.data.zeroing_by_date(current_date);
                    }
                }
                _ => {}
            }
        }
    }

    pub fn increment_by_date(&mut self, date: NaiveDate) {
        self.data.add(date, Value(1));

        if let Some(v) = self.data.get(date) {
            match self.metadata().limitation_value() {
                metadata::LimitationValue::Max(max) => {
                    if v.0 > max {
                        self.data.zeroing_by_date(date);
                    }
                }
                _ => {}
            }
        }
    }

    pub fn decrement(&mut self) {
        let current_date = self.metadata().current_time();
        self.data.decrement(current_date);
    }

    pub fn decrement_by_date(&mut self, date: NaiveDate) {
        self.data.decrement(date);
    }

    pub fn data(&self) -> &Data {
        &self.data
    }

    pub fn mut_data(&mut self) -> &mut Data {
        &mut self.data
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    pub fn mut_metadata(&mut self) -> &mut Metadata {
        &mut self.metadata
    }
}
