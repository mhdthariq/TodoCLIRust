mod cli;
mod error;
mod storage;
mod task;

use clap::Parser;
use error::{Result, TodoError};
use storage::{load_tasks, save_tasks};

fn main() -> Result<()> {
    let args = cli::Cli::parse();
    let mut tasks = load_tasks()?;

    match args.command {
        cli::Command::Add { description } => {
            let id = tasks.add_task(description.clone());
            println!("Task #{} added: {}", id, description);
        }
        cli::Command::Complete { id } => {
            tasks.complete_task(id).map_err(|_| TodoError::InvalidTaskId)?;
            println!("Task #{} marked as completed!", id);
        }
        cli::Command::Remove { id } => {
            tasks.remove_task(id).map_err(|_| TodoError::InvalidTaskId)?;
            println!("Task #{} removed!", id);
        }
        cli::Command::List => {
            if tasks.is_empty() {
                println!("No tasks found.");
            } else {
                tasks.list_tasks();
            }
        }
    }

    save_tasks(&tasks)?;
    Ok(())
}
