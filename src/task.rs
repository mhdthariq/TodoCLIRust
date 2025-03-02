use serde::{Deserialize, Serialize};
use chrono::NaiveDate;  // Import chrono's date type
use std::result::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub description: String,
    pub completed: bool,
    pub due_date: Option<NaiveDate>,  // New field for due date
    pub priority: Priority,  // New field for priority
}

#[derive(Serialize, Deserialize, Default)]
pub struct TaskList {
    tasks: Vec<Task>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Priority {
    High,
    Medium,
    Low,
}

impl TaskList {
    pub fn add_task(&mut self, description: String, due_date: Option<NaiveDate>, priority: Priority) -> usize {
        let id = self.tasks.len();
        self.tasks.push(Task {
            description,
            completed: false,
            due_date,
            priority,
        });
        id
    }

    pub fn complete_task(&mut self, id: usize) -> Result<(), String> {
        self.tasks
            .get_mut(id)
            .map(|task| {
                task.completed = true;
            })
            .ok_or_else(|| "Invalid task ID".to_string())
    }

    pub fn remove_task(&mut self, id: usize) -> Result<(), String> {
        if id < self.tasks.len() {
            self.tasks.remove(id);
            Ok(())
        } else {
            Err("Invalid task ID".to_string())
        }
    }

    pub fn list_tasks(&self) {
        for (id, task) in self.tasks.iter().enumerate() {
            let due_date_display = match task.due_date {
                Some(date) => format!("Due: {}", date),
                None => "No due date".to_string(),
            };
            println!(
                "#{}: [{}] {} (Priority: {}, {})",
                id,
                if task.completed { "✓" } else { " " },
                task.description,
                task.priority,
                due_date_display
            );
        }
    }

    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
}

impl Default for Priority {
    fn default() -> Self {
        Priority::Medium // Default priority is Medium
    }
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::High => write!(f, "High"),
            Priority::Medium => write!(f, "Medium"),
            Priority::Low => write!(f, "Low"),
        }
    }
}
