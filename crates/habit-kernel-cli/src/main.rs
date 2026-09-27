use clap::Parser;
use clap_derive::Subcommand;
use habit_kernel_core::db::DataBase;

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
    Create {
        #[arg(long, value_name = "NAME HABIT")]
        habit: Option<String>,
    }
}

fn main() -> anyhow::Result<()> {
    let mut db = DataBase::load()?.unwrap_or_else(|| DataBase::new() );
    db.save()?;

    let cli = Cli::parse();

    match cli.command {
        Commands::Create { habit } => {
            if let Some(habit_name) = habit {
                create_habit(&mut db, habit_name);
            }
        }
    }

    db.save()
}

fn create_habit(db: &mut DataBase, name: String) {
    db.add_habit(name);
}
