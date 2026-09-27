use clap::Parser;
use clap_derive::Subcommand;

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
    Hello { name: String }
}


fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Hello { name } => println!("Hello, {name}")
    }
}
