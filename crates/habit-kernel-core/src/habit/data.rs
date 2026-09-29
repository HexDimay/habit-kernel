use std::{collections::HashMap, hash::Hash};

use chrono::NaiveDate;

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

    pub fn get(&self, date: NaiveDate) -> Option<&Value> {
        self.done.get(&date)
    }

    pub fn get_mut(&mut self, date: NaiveDate) -> Option<&mut Value> {
        self.done.get_mut(&date)
    }

    pub fn add_now(&mut self, value: Value) {
        let date = chrono::Local::now().date_naive();
        self.add(date, value);
    }

    pub fn add(&mut self, date: NaiveDate, value: Value) {
        if let Some(v) = self.done.get_mut(&date) {
            *v += value;
            return;
        }

        self.done.insert(date, value);
    }

    pub fn decrement(&mut self, date: NaiveDate) {
        if let Some(v) = self.done.get_mut(&date) {
            *v -= Value(1);
        }
    }

    pub fn zeroing_by_date(&mut self, date: NaiveDate) {
        if let Some(v) = self.done.get_mut(&date) {
            v.zeroing();
        }
    }

    pub fn count_current_done(&self) -> usize {
        if let Some(n) = self.done.get(&chrono::Local::now().date_naive()) {
            return n.0;
        }

        0
    }
}

impl Hash for Data {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.done
            .iter()
            .map(|c| (c.0.clone(), c.1.clone()))
            .collect::<Vec<(NaiveDate, Value)>>()
            .hash(state);
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Hash)]
pub struct Value(pub(crate) usize);

impl Value {
    pub fn new(value: usize) -> Self {
        Self(value)
    }

    pub fn zeroing(&mut self) {
        self.0 = 0;
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

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::{
        habit::data::{Data, Value},
        value_list,
    };

    #[test]
    fn test_create_list() {
        let data = value_list!(@DMY:
            (1,1,2026) <= 1,
            (2,1,2026) <= 1,
            (6,1,2026) <= 1,
            (8,1,2026) <= 7
        );

        assert_eq!(
            data.get(NaiveDate::from_ymd_opt(2026, 1, 2).unwrap())
                .unwrap()
                .0,
            1
        );
        assert_eq!(
            data.get(NaiveDate::from_ymd_opt(2026, 1, 8).unwrap())
                .unwrap()
                .0,
            7
        );
    }

    #[test]
    fn test_add_value() {
        let data = value_list!(@DMY:
            (1,1,2026) <= 1,
            (2,1,2026) <= 1,
            (3,1,2026) <= 1,
            (3,1,2026) <= 1
        );

        assert_eq!(
            data.get(NaiveDate::from_ymd_opt(2026, 1, 2).unwrap())
                .unwrap()
                .0,
            1
        );

        assert_eq!(
            data.get(NaiveDate::from_ymd_opt(2026, 1, 3).unwrap())
                .unwrap()
                .0,
            2
        );
    }

    #[test]
    fn test_del_value() {
        let mut data = value_list!(@DMY:
            (1,1,2026) <= 1,
            (2,1,2026) <= 1,
            (3,1,2026) <= 1,
            (3,1,2026) <= 1
        );

        data.zeroing_by_date(NaiveDate::from_ymd_opt(2026, 1, 3).unwrap());

        assert_eq!(
            data.done
                .get(&NaiveDate::from_ymd_opt(2026, 1, 2).unwrap())
                .unwrap()
                .0,
            1
        );

        assert_eq!(
            data.done
                .get(&NaiveDate::from_ymd_opt(2026, 1, 3).unwrap())
                .unwrap()
                .0,
            0
        );
    }
}

#[macro_export]
macro_rules! value_list {
    (@DMY: $(($day:expr,$month:expr,$year:expr) <= $value:expr),*) => {
        {
            let mut data = Data::new();

            $(
                data.add(
                    chrono::NaiveDate::from_ymd_opt($year, $month, $day).unwrap(),
                    Value::new($value)
                );
            )*

            data
        }
    };

    (@YMD: $(($year:expr,$month:expr,$day:expr) <= $value:expr),*) => {
        {
            let mut data = Data::new();

            $(
                data.add(
                    chrono::NaiveDate::from_ymd_opt($year, $month, $day).unwrap(),
                    Value::new($value)
                );
            )*

            data
        }
    };
}
