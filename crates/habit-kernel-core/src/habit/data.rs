use std::collections::LinkedList;

#[derive(Debug)]
pub struct Data {
    done: LinkedList<Value>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            done: LinkedList::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Value {
    date: chrono::NaiveDate,
    value: usize,
}
