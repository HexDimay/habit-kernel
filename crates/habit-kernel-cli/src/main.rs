use clap::Parser;
use habit_kernel_cli::commands::{
    Cli, Command, HabitCommand,
    chart::chart_habit,
    create::create_habit,
    delete::delete_habit,
    done::done_habit,
    edit::edit_habit,
    select::select_habit,
    view::{view_all_habits, view_selected_habit},
};
use habit_kernel_core::{
    db::DataBase,
    io::{GetPath, Load, Save},
};

fn main() -> anyhow::Result<()> {
    let mut db = if std::fs::exists(<DataBase as GetPath<&'static str>>::get_path())? {
        <DataBase as Load<&'static str>>::load::<DataBase>()?
    } else {
        DataBase::new()
    };

    db.save()?;

    let cli = Cli::parse();

    match cli.command() {
        Command::Habit { command } => match command {
            HabitCommand::Create(args) => create_habit(&mut db, args),
            HabitCommand::Delete(args) => delete_habit(&mut db, args),
            HabitCommand::Select(args) => select_habit(&mut db, args),
            HabitCommand::Edit(args) => edit_habit(&mut db, args),
            HabitCommand::Done => done_habit(&mut db),
            HabitCommand::Chart(args) => chart_habit(&mut db, args),
        },

        Command::View { all, selected } => {
            if *all {
                view_all_habits(&db);
            }

            if *selected {
                view_selected_habit(&db);
            }
        }
    }

    db.save()
}
