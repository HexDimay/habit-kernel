use chrono::NaiveDate;
use habit_kernel_core::habit::{
    Habit,
    metadata::LimitationValue,
    traits::{DataContainer, HabitEntity},
};

#[derive(cli_table::Table)]
pub struct TableHabit {
    #[table(title = "ID")]
    id: uuid::Uuid,
    #[table(title = "Habit Name")]
    name: String,
    #[table(title = "Created")]
    created: NaiveDate,
    #[table(title = "Limitation")]
    limitation: LimitationValue,
    #[table(title = "Today done")]
    today_done: usize,
}

impl From<&Habit> for TableHabit {
    fn from(value: &Habit) -> Self {
        Self {
            id: value.id(),
            name: value.name().to_string(),
            created: *value.created(),
            limitation: value.metadata().limitation_value(),
            today_done: value.data().count_current_done(),
        }
    }
}
