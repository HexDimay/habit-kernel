use chrono::NaiveDate;
use clap_derive::Args;
use cli_table::{Style, Table, print_stdout};
use habit_kernel_core::{
    db::{DataBase, traits::DataBaseMetadata},
    habit::traits::{DataContainer, HabitEntity, ValueEntity},
};

#[derive(Debug, Args)]
pub struct ChartArgs {
    #[arg(long)]
    weekly: bool,
}

impl ChartArgs {
    pub fn weekly(&self) -> bool {
        self.weekly
    }
}

pub fn chart_habit(db: &mut DataBase, args: &ChartArgs) {
    if args.weekly() {
        create_and_print_weekly_table(db);
    }
}

fn create_and_print_weekly_table(db: &mut DataBase) {
    let mut title = vec!["HABIT NAME".to_owned(), "LIMITATION".to_owned()];
    let today = chrono::Local::now().date_naive();
    let days: Vec<NaiveDate> = (0..7)
        .into_iter()
        .map(|d| {
            let day = today.checked_sub_days(chrono::Days::new(d)).unwrap();
            title.push(day.to_string());

            day
        })
        .rev()
        .collect();
    let table: Vec<Vec<_>> = db
        .iter()
        .map(|h| {
            let mut data = vec![
                h.metadata().name().to_owned(),
                h.metadata().limitation_value().to_string(),
            ];

            let dones: Vec<String> = days
                .iter()
                .map(|d| {
                    if let Some(v) = h.data().get(*d) {
                        return v.get().to_string();
                    }

                    "0".to_string()
                })
                .collect();

            data.extend(dones);

            data
        })
        .collect();

    let table = table.table().title(title).bold(true);
    print_stdout(table).unwrap();
}
