mod commands;
mod storage;
mod todo;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todo", version = "1.0", author = "cdsiats")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        task: String,
    },
    List
}

fn main() {
    let cli = Cli::parse();

    match &cli.command  {
        Commands::Add { task } => commands::add(task),
        Commands::List => commands::list(),
    }
}
