# TodoCLIRust

A simple command-line todo list application written in Rust. This CLI tool helps you manage your tasks with basic operations like adding, completing, removing, and listing tasks.

## Features

- Add new tasks
- Mark tasks as completed
- Remove tasks
- List all tasks
- Persistent storage using JSON
- Simple and intuitive command-line interface

## Installation

### Prerequisites

- Rust and Cargo (Latest stable version)

### Building from source

1. Clone the repository
```bash
git clone https://github.com/mhdthariq/TodoCLIRust.git
cd TodoCLIRust
```

2. Build the project
```bash
cargo build --release
```

The executable will be available in `target/release/todo_rust_cli`

## Usage

### Basic Commands

1. Add a new task:
```bash
cargo run -- add "Your task description" (without due date)
cargo run -- add "Your task description" --due-date "YYYY-MM-DD" (with due date)
```

2. List all tasks:
```bash
cargo run -- list
```

3. Mark a task as completed:
```bash
cargo run -- complete <task-id>
```

4. Remove a task:
```bash
cargo run -- remove <task-id>
```

### Examples

```bash
# Add a new task
$ cargo run -- add "Buy groceries"
Task #0 added: Buy groceries

# Add another task with due date
$ cargo run -- add "Learn English" --due-date "2025-02-13"
Task #1 added: Call mom

# List all tasks
$ cargo run -- list
#0: [ ] Buy groceries
#1: [ ] Call mom

# Complete a task
$ cargo run -- complete 0
Task #0 marked as completed!

# List tasks again
$ cargo run -- list
#0: [✓] Buy groceries
#1: [ ] Call mom

# Remove a task
$ cargo run -- remove 1
Task #1 removed!
```

## File Storage

Tasks are automatically saved to a `tasks.json` file in the current directory. This file is created automatically if it doesn't exist and is updated after each operation.

## Project Structure

```
TodoCLIRust/
├── src/
│   ├── main.rs        # Application entry point
│   ├── cli.rs         # CLI argument parsing
│   ├── task.rs        # Task and TaskList implementations
│   ├── storage.rs     # File I/O operations
│   └── error.rs       # Error handling
├── Cargo.toml         # Project dependencies and metadata
└── README.md          # Project documentation
```

## Dependencies

- clap: Command line argument parsing
- serde: Serialization/Deserialization
- serde_json: JSON handling
- thiserror: Error handling

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Future Improvements

- [x] Add due dates for tasks
- [x] Add priority levels
- [ ] Add categories/tags
- [ ] Add search functionality
- [ ] Add task editing
- [ ] Add task sorting
- [ ] Add multi-user support
- [ ] Add data backup

## Support

If you encounter any problems or have suggestions, please open an issue on the GitHub repository.
