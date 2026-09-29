use habit_kernel_core::db::DataBase;

use cli_table::{WithTitle, print_stdout};

use crate::{check_current_habit, tables::TableHabit};

pub fn view_selected_habit(db: &DataBase) {
    check_current_habit!(db);
    let habit = db.get_current_habit().unwrap();

    print_stdout(vec![TableHabit::from(habit)].with_title()).unwrap();
}

pub fn view_all_habits(db: &DataBase) {
    let mut table = vec![];

    db.iter().for_each(|habit| {
        table.push(TableHabit::from(habit));
    });

    print_stdout(table.with_title()).unwrap();
}
