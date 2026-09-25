#[derive(Debug, Clone)]
pub struct Metadata {
    id: uuid::Uuid,
    name: String,
    created: chrono::NaiveDate,
    limitation_value: LimitationValue,
}

impl Metadata {
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            name: String::new(),
            created: chrono::Local::now().date_naive(),
            limitation_value: LimitationValue::Unlimited,
        }
    }

    pub fn id(&self) -> uuid::Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    pub fn created(&self) -> &chrono::NaiveDate {
        &self.created
    }

    pub fn limitation_value(&self) -> LimitationValue {
        self.limitation_value
    }

    pub fn set_limitation_value(&mut self, new_limit: LimitationValue) {
        self.limitation_value = new_limit;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LimitationValue {
    Max(usize),
    Unlimited,
}
