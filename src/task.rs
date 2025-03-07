use serde::{Deserialize, Serialize};
use chrono::NaiveDate;  // Import chrono's date type
use std::result::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub description: String,
    pub completed: bool,
    pub due_date: Option<NaiveDate>,  // New field for due date
    pub priority: Priority,  // New field for priority
    pub tags: Vec<String>,  // New field for tags
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
    pub fn add_task(&mut self, description: String, due_date: Option<NaiveDate>, priority: Priority, tags: Vec<String>) -> usize {
        let id = self.tasks.len();
        self.tasks.push(Task {
            description,
            completed: false,
            due_date,
            priority,
            tags,
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
                "#{}: [{}] {} (Priority: {}, {}, Tags: [{}])",  // Fixed format string
                id,
                if task.completed { "✓" } else { " " },
                task.description,
                task.priority,
                due_date_display,
                task.tags.join(", ")
            );
        }
    }

    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    pub fn add_tags(&mut self, id: usize, new_tags: Vec<String>) -> Result<(), String> {
        if let Some(task) = self.tasks.get_mut(id) {
            for tag in new_tags {
                if !task.tags.contains(&tag) {
                    task.tags.push(tag);
                }
            }
            Ok(())
        } else {
            Err("Invalid task ID".to_string())
        }
    }

    pub fn remove_tags(&mut self, id: usize, tags_to_remove: Vec<String>) -> Result<(), String> {
        if let Some(task) = self.tasks.get_mut(id) {
            task.tags.retain(|tag| !tags_to_remove.contains(tag));
            Ok(())
        } else {
            Err("Invalid task ID".to_string())
        }
    }

    pub fn list_tasks_by_tag(&self, tag: &str) {
        let filtered_task: Vec<_> = self.tasks.iter()
            .enumerate()
            .filter(|(_, task)| task.tags.contains(&tag.to_string()))
            .collect();

        if filtered_task.is_empty() {
            println!("No tasks found with tag '{}'", tag);
            return;
        }

        for (id, task) in filtered_task {
            let due_date_display = match task.due_date {
                Some(date) => format!("Due: {}", date),
                None => "No due date".to_string()
            };
            println!(
                "#{}: [{}] {} (Priority: {}, {}, Tags: {})",
                id,
                if task.completed { "✓" } else { " " },
                task.description,
                task.priority,
                due_date_display,
                task.tags.join(", ")
            );
        }
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
