use crate::habit::traits::Limitation;

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

    pub fn set_name(&mut self, new_name: &str) {
        self.name = new_name.to_owned();
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

impl From<String> for LimitationValue {
    fn from(value: String) -> Self {
        if let Ok(n) = value.parse() {
            return LimitationValue::Max(n);
        }

        LimitationValue::Unlimited
    }
}

impl std::fmt::Display for LimitationValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            LimitationValue::Unlimited => "Unlimited".to_owned(),
            LimitationValue::Max(num) => format!("Max: {num}"),
        };
        write!(f, "{res}")
    }
}

// ---------------------------------------------------------------------
// Реализация трейта `Limitation`.
//
// `LimitationValue` моделирует только верхнюю границу (`Max`/`Unlimited`),
// поэтому нижняя граница вырождена: `min` всегда 0, а задать её нельзя.
// ---------------------------------------------------------------------
impl Limitation<usize> for LimitationValue {
    /// Нижняя граница не хранится, поэтому минимум всегда равен 0.
    fn min(&self) -> usize {
        0
    }

    /// Нижняя граница в `LimitationValue` не представима: установить можно
    /// только 0. Для ненулевого значения подходящего состояния нет, поэтому
    /// вызов игнорируется (см. `min`).
    fn set_mut(&mut self, new_min: usize) {
        debug_assert_eq!(
            new_min, 0,
            "LimitationValue задаёт только верхнюю границу (min всегда 0)"
        );
    }

    /// Верхняя граница: значение `Max(n)` или `usize::MAX` для `Unlimited`
    /// (последнее означает отсутствие эффективного потолка).
    fn max(&self) -> usize {
        match self {
            LimitationValue::Max(n) => *n,
            LimitationValue::Unlimited => usize::MAX,
        }
    }

    fn set_max(&mut self, new_max: usize) {
        *self = LimitationValue::Max(new_max);
    }

    /// `true` только для явно заданного потолка (`Max`).
    fn is_lim(&self) -> bool {
        matches!(self, LimitationValue::Max(_))
    }
}
