use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    // Add a new task
    Add {
        description: String,
        #[arg(short, long)]
        due_date: Option<String>,
        #[arg(short, long, default_value = "medium")]
        priority: String,
        #[arg(short, long, value_delimiter = ',')]
        tags: Option<Vec<String>>,
    },
    // Mark a task as completed
    Complete { id: usize },
    // Remove a task
    Remove { id: usize },
    // List all tasks
    List,
    AddTags {
        id: usize,
        #[arg(value_delimiter = ',')]
        tags: Vec<String>,
    },
    RemoveTags {
        id: usize,
        #[arg(value_delimiter = ',')]
        tags: Vec<String>,
    },
    ListByTag {
        tag: String,
    },
}
