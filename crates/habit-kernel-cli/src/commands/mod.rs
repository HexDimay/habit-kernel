use clap_derive::{Parser, Subcommand};

use crate::commands::{create::CreateArgs, delete::DeleteArgs, edit::EditArgs, select::SelectArgs};

pub mod create;
pub mod delete;
pub mod done;
pub mod edit;
pub mod select;
pub mod view;

#[derive(Parser, Debug)]
#[command(name = "HabitKernel CLI")]
#[command(version)]
#[command(about = "`HabitKernel` is an open-source habit manager.", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

impl Cli {
    pub fn command(&self) -> &Command {
        &self.command
    }
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Habit {
        #[command(subcommand)]
        command: HabitCommand,
    },

    View {
        #[arg(long)]
        all: bool,

        #[arg(short, long)]
        selected: bool,
    },
}
#[derive(Debug, Subcommand)]
pub enum HabitCommand {
    Create(CreateArgs),
    Delete(DeleteArgs),
    Select(SelectArgs),
    Edit(EditArgs),
    Done,
}
