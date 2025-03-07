mod cli;
mod error;
mod storage;
mod task;

use clap::Parser;
use error::{Result, TodoError};
use storage::{load_tasks, save_tasks};
use chrono::NaiveDate;
use task::Priority;

fn parse_priority(priority: &str) -> Result<Priority> {
    match priority.to_lowercase().as_str() {
        "low" => Ok(Priority::Low),
        "medium" => Ok(Priority::Medium),
        "high" => Ok(Priority::High),
        _ => Err(TodoError::InvalidPriority),
    }
}

fn main() -> Result<()> {
    let args = cli::Cli::parse();
    let mut tasks = load_tasks()?;

    match args.command {
        cli::Command::Add { description, due_date, priority, tags } => {
            // Parse the due date if provided
            let parsed_due_date = due_date
                .as_deref()
                .map(|date_str| NaiveDate::parse_from_str(date_str, "%Y-%m-%d"))
                .transpose()
                .map_err(|_| TodoError::InvalidTaskId)?;  // Using InvalidTaskId for simplicity

            let parsed_priority = parse_priority(&priority)?;
            let tags = tags.unwrap_or_default();

            let id = tasks.add_task(description.clone(), parsed_due_date, parsed_priority, tags.clone());
            println!("Task #{} added: {} (Priority: {}){} Tags: [{}]", id, description, priority,
                if let Some(date) = parsed_due_date {
                    format!(" (Due: {})", date)
                } else {
                    "".to_string()
                },
                tags.join(", ")
            );
        }
        cli::Command::AddTags { id, tags } => {
            let tags_clone = tags.clone();
            tasks.add_tags(id, tags).map_err(|_| TodoError::InvalidTaskId)?;
            println!("Added tags [{}] to task #{}", tags_clone.join(", "), id);
        }
        cli::Command::RemoveTags { id, tags } => {
            tasks.remove_tags(id, tags.clone()).map_err(|_| TodoError::InvalidTaskId)?;
            println!("Removed tags [{}] from task #{}", tags.join(", "), id);
        }
        cli::Command::ListByTag { tag } => {
            tasks.list_tasks_by_tag(&tag);
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
