use habit_kernel_core::{db::DataBase, habit::Habit};

pub fn view_all_habits(db: &mut DataBase) {
    let mut first_row = format!("| ID\t\t\t\t\t| NAME\t\t\t\t| CREATED\t| LIMITATION\t| DAYS\t|\n");
    db.iter().for_each(|h| {
        first_row.push_str(short_string_view_habit(h).as_str());
    });

    println!("{first_row}");
}

pub fn short_string_view_habit(habit: &Habit) -> String {
    let (id, name, created, limit, count_days) = (
        habit.metadata().id(),
        habit.metadata().name(),
        habit.metadata().created(),
        habit.metadata().limitation_value(),
        habit.data().count_days(),
    );

    format!("| {id}\t| {name}\t| {created}\t| {limit}\t| {count_days}\t|\n")
}
