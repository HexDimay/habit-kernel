use std::collections::LinkedList;

#[derive(Debug)]
pub struct Data {
    pub(crate) done: LinkedList<Value>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            done: LinkedList::new(),
        }
    }

    /// Поиск `Value` по дате и возвращение его индекса.
    /// Если элементов нет, то возвращает `IndexDone::IndexNone`.
    /// Если поиск успешен, то возвращается позиция элемента `IndexExact(usize)`
    /// Если элемент не найден, но при этом есть элементы, то возвращается диапозон индексов,
    /// в котором могут распологаться данные, если индексы одинаковые, то это конец массива данных.
    pub fn find_index_by_date(&self, date: chrono::NaiveDate) -> IndexDone {
        if self.done.is_empty() {
            return IndexDone::IndexNone;
        }

        let mut min_idx = 0;
        let mut max_idx = 0;
        for (i, v) in self.done.iter().enumerate() {
            if v.date == date {
                return IndexDone::IndexExact(i);
            }

            if v.date < date {
                min_idx = i;

                // Наш массив всегда отсортирован и сохраняется инвариант диапазонов индексов.
                max_idx = i + 1;

                if max_idx >= self.done.len() {
                    max_idx = min_idx;
                }
            }
        }

        IndexDone::IndexBetween(min_idx, max_idx)
    }
}

#[derive(Debug, Clone)]
pub struct Value {
    date: chrono::NaiveDate,
    value: usize,
}

impl Value {
    pub fn new(date: chrono::NaiveDate, value: usize) -> Self {
        Self { date, value }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IndexDone {
    IndexBetween(usize, usize),
    IndexExact(usize),
    IndexNone,
}

#[cfg(test)]
mod tests {
    use crate::habit::data::{Data, IndexDone, Value};

    #[test]
    fn test_find_index_by_date() {
        let mut data = Data::new();
        data.done.push_back(Value::new(
            chrono::NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
            1,
        ));

        data.done.push_back(Value::new(
            chrono::NaiveDate::from_ymd_opt(2026, 1, 2).unwrap(),
            1,
        ));

        data.done.push_back(Value::new(
            chrono::NaiveDate::from_ymd_opt(2026, 1, 6).unwrap(),
            1,
        ));

        data.done.push_back(Value::new(
            chrono::NaiveDate::from_ymd_opt(2026, 1, 8).unwrap(),
            1,
        ));

        assert_eq!(data.find_index_by_date(chrono::NaiveDate::from_ymd_opt(2026, 1, 1).unwrap()), IndexDone::IndexExact(0));
        assert_eq!(data.find_index_by_date(chrono::NaiveDate::from_ymd_opt(2026, 1, 2).unwrap()), IndexDone::IndexExact(1));
        assert_eq!(data.find_index_by_date(chrono::NaiveDate::from_ymd_opt(2026, 1, 3).unwrap()), IndexDone::IndexBetween(1, 2));
        assert_eq!(data.find_index_by_date(chrono::NaiveDate::from_ymd_opt(2026, 1, 7).unwrap()), IndexDone::IndexBetween(2, 3));
    }
}
