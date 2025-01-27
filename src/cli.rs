use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    // Add a new task
    Add { description: String },
    // Mark a task as completed
    Complete { id: usize },
    // Remove a task
    Remove { id: usize },
    // List all tasks
    List,
}
