#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Hash)]
pub struct Metadata {
    id: uuid::Uuid,
    name: String,
    created: chrono::NaiveDate,
    limitation_value: LimitationValue,
    current_time: chrono::NaiveDate,
}

impl Metadata {
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            name: String::new(),
            created: chrono::Local::now().date_naive(),
            limitation_value: LimitationValue::Unlimited,
            current_time: chrono::Local::now().date_naive(),
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

    pub fn current_time(&self) -> chrono::NaiveDate {
        self.current_time
    }

    pub fn update_current_time(&mut self) {
        self.current_time = chrono::Local::now().date_naive();
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize, Hash,
)]
pub enum LimitationValue {
    Max(usize),
    Unlimited,
}
