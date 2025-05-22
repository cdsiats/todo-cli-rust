mod commands;
mod storage;
mod todo;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todo-cli", version = "0.1", author = "cdsiats")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds a new task to the TODO list.
    Add {
        task: String,
    },
    /// Lists all tasks in the TODO list.
    List,
    /// Removes a task from the TODO list.
    Rm {
        id: u32,
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command  {
        Commands::Add { task } => commands::add(task),
        Commands::List => commands::list(),
        Commands::Rm { id } => commands::remove(*id),
    }
}
