use habit_kernel_core::{db::DataBase, habit::Habit};

use cli_table::{Style, Table, print_stdout};

use crate::check_current_habit;

pub fn view_selected_habit(db: &DataBase) {
    check_current_habit!(db);
    let habit = db.get_current_habit().unwrap();

    let table = vec![get_col_habit_for_table(habit)]
        .table()
        .title(vec!["ID", "NAME", "CREATED", "LIMITATION", "DAYS"])
        .bold(true);

    print_stdout(table).unwrap();
}

pub fn view_all_habits(db: &DataBase) {
    let mut table = vec![];

    db.iter().for_each(|habit| {
        table.push(get_col_habit_for_table(habit));
    });

    let table = table
        .table()
        .title(vec!["ID", "NAME", "CREATED", "LIMITATION", "DAYS"])
        .bold(true);

    print_stdout(table).unwrap();
}

fn get_col_habit_for_table(habit: &Habit) -> Vec<String> {
    vec![
        habit.metadata().id().to_string(),
        habit.metadata().name().to_owned(),
        habit.metadata().created().to_string(),
        habit.metadata().limitation_value().to_string(),
        habit.data().count_days().to_string(),
    ]
}
