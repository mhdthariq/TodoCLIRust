use serde::{Deserialize, Serialize};
use std::result::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub description: String,
    pub completed: bool,
}

#[derive(Serialize, Deserialize, Default)]
pub struct TaskList {
    tasks: Vec<Task>,
}

impl TaskList {
    pub fn add_task(&mut self, description: String) -> usize {
        let id = self.tasks.len();
        self.tasks.push(Task {
            description,
            completed: false,
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
            println!(
                "#{}: [{}] {}",
                id,
                if task.completed { "✓" } else { " " },
                task.description
            );
        }
    }

    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
}
