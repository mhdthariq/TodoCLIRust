mod cli;
mod error;
mod storage;
mod task;

use clap::Parser;
use error::{Result, TodoError};
use storage::{load_tasks, save_tasks};
use chrono::NaiveDate;

fn main() -> Result<()> {
    let args = cli::Cli::parse();
    let mut tasks = load_tasks()?;

    match args.command {
        cli::Command::Add { description, due_date } => {
            // Parse the due date if provided
            let parsed_due_date = due_date
                .as_deref()
                .map(|date_str| NaiveDate::parse_from_str(date_str, "%Y-%m-%d"))
                .transpose()
                .map_err(|_| TodoError::InvalidTaskId)?;  // Using InvalidTaskId for simplicity

            let id = tasks.add_task(description.clone(), parsed_due_date);
            println!("Task #{} added: {}{}", id, description,
                if let Some(date) = parsed_due_date {
                    format!(" (Due: {})", date)
                } else {
                    "".to_string()
                }
            );
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
