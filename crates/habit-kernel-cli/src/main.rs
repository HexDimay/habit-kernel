use clap::Parser;
use habit_kernel_cli::commands::{
    Cli, Command, HabitCommand, create::create_habit, delete::delete_habit, view::view_all_habits,
};
use habit_kernel_core::db::DataBase;

fn main() -> anyhow::Result<()> {
    let mut db = DataBase::load()?.unwrap_or_else(|| DataBase::new());
    db.save()?;

    let cli = Cli::parse();

    match cli.command() {
        Command::Habit { command } => match command {
            HabitCommand::Create(create_args) => create_habit(&mut db, create_args),
            HabitCommand::Delete(delete_args) => delete_habit(&mut db, delete_args),
        },

        Command::View { all } => {
            if *all {
                view_all_habits(&mut db);
            }
        }
    }

    db.update()
}
