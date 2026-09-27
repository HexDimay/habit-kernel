use anyhow::bail;
use clap::Parser;
use clap_derive::{Args, Subcommand};
use habit_kernel_core::{db::DataBase, habit::Habit};

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
        #[command(subcommand)]
        command: HabitCommand,
    },

    View {
        #[arg(long)]
        all: bool,
    },
}
#[derive(Debug, Subcommand)]
enum HabitCommand {
    Create(CreateArgs),
    Delete(DeleteArgs),
}

#[derive(Debug, Args)]
struct CreateArgs {
    name: Option<String>,
}

#[derive(Debug, Args)]
struct DeleteArgs {
    id: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let mut db = DataBase::load()?.unwrap_or_else(|| DataBase::new());
    db.save()?;

    let cli = Cli::parse();

    match cli.command {
        Commands::Habit { command } => match command {
            HabitCommand::Create(create_args) => create_habit(&mut db, create_args.name.unwrap()),
            HabitCommand::Delete(delete_args) => {
                match delete_habit(&mut db, delete_args.id.unwrap()) {
                    Ok(h) => println!(
                        "The habit was successfully removed. [ id: {}; name: {}; ]",
                        h.metadata().id(),
                        h.metadata().name()
                    ),
                    Err(e) => println!("The habit was successfully removed. Error: {e}"),
                }
            }
        },

        Commands::View { all } => {
            if all {
                let mut first_row =
                    format!("| ID\t\t\t\t\t| NAME\t\t\t\t| CREATED\t| LIMITATION\t| DAYS\t|\n");
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

fn delete_habit(db: &mut DataBase, id: String) -> anyhow::Result<Habit> {
    if let Some(habit) = db.del_by_id(uuid::Uuid::parse_str(id.as_str())?) {
        return Ok(habit);
    }

    bail!(id)
}

fn short_string_view_habit(habit: &Habit) -> String {
    let (id, name, created, limit, count_days) = (
        habit.metadata().id(),
        habit.metadata().name(),
        habit.metadata().created(),
        habit.metadata().limitation_value(),
        habit.data().count_days(),
    );

    format!("| {id}\t| {name}\t| {created}\t| {limit}\t| {count_days}\t|\n")
}
