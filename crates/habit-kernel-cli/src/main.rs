use clap::Parser;
use clap_derive::Subcommand;
use habit_kernel_core::{db::DataBase, habit::Habit};

use crate::Commands::View;

#[derive(Parser, Debug)]
#[command(name = "HabitKernel CLI")]
#[command(version)]
#[command(about = "`HabitKernel` is an open-source habit manager.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Habit {
        #[arg(long, value_name = "HABIT NAME")]
        create: String,
    },

    View {
        #[arg(long)]
        all: bool
    }
}

fn main() -> anyhow::Result<()> {
    let mut db = DataBase::load()?.unwrap_or_else(|| DataBase::new() );
    db.save()?;

    let cli = Cli::parse();

    match cli.command {
        Commands::Habit { create } => {
            if !create.is_empty() {
                create_habit(&mut db, create);
            }
        },

        Commands::View { all } => {
            if all {
                let mut first_row = format!("| ID\t\t\t\t\t| NAME\t\t\t\t| CREATED\t| LIMITATION\t| DAYS\t|\n");
                db.habit_list.iter().for_each(|h| {
                    first_row.push_str(short_string_view_habit(h).as_str());
                });

                println!("{first_row}");
            }
        }
    }

    db.save()
}

fn create_habit(db: &mut DataBase, name: String) {
    db.add_habit(name);
}

fn short_string_view_habit(habit: &Habit) -> String {
    let (id, name, created, limit, count_days) = (
        habit.metadata().id(),
        habit.metadata().name(),
        habit.metadata().created(),
        habit.metadata().limitation_value(),
        habit.data().count_days()
    );


    format!("| {id}\t| {name}\t| {created}\t| {limit}\t| {count_days}\t|\n")
}
