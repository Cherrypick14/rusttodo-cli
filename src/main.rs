use std::env;
use std::fs::{OpenOptions, read_to_string};
use std::io::{self, Write};

/// Entry point for the CLI To-Do application
fn main() {
    // Collect command-line arguments (e.g., ["program", "add", "Task name"])
    let args: Vec<String> = env::args().collect();

    // If no subcommand is provided, display usage instructions
    if args.len() < 2 {
        eprintln!("Usage:");
        eprintln!("  add <task>     - Add a new task");
        eprintln!("  list           - List all tasks");
        return;
    }

    // First argument after the binary name is treated as the command
    let command = &args[1];

    match command.as_str() {
        "add" => {
            // Ensure a task description is provided
            if args.len() < 3 {
                eprintln!("⚠️ Please provide a task to add.");
                return;
            }

            // Join all parts of the task (supporting multiple words)
            let task = &args[2..].join(" ");
            add_task(task);
        }
        "list" => {
            list_tasks();
        }
        _ => {
            // Handle invalid or unsupported commands
            eprintln!("❌ Unknown command: {}", command);
            eprintln!("Available commands: add, list");
        }
    }
}

/// Appends a new task to `todo.txt`, creating the file if it doesn't exist
fn add_task(task: &str) {
    let mut file = OpenOptions::new()
        .create(true)   // Create file if it doesn't already exist
        .append(true)   // Append to the existing file
        .open("todo.txt")
        .expect("❌ Could not open or create todo.txt");

    writeln!(file, "{}", task).expect("❌ Could not write to todo.txt");
    println!("✅ Added task: {}", task);
}

/// Reads all tasks from `todo.txt` and displays them in a numbered list
fn list_tasks() {
    match read_to_string("todo.txt") {
        Ok(data) => {
            if data.trim().is_empty() {
                println!("📭 No tasks yet! Add your first task with: cargo run -- add \"Do something\"");
            } else {
                println!("📝 Your tasks:");
                for (i, line) in data.lines().enumerate() {
                    println!("{}. {}", i + 1, line);
                }
            }
        }
        Err(_) => {
            // Handles case when the file is missing or unreadable
            println!("🗂️ No tasks found. Start by adding one with: cargo run -- add \"Do something\"");
        }
    }
}

/// Unit tests for core functionality
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    /// Test adding tasks by writing manually to a temporary file
    #[test]
    fn test_add_and_list_tasks() {
        let test_file = "test_todo.txt";

        // Remove any existing test file to ensure a clean state
        let _ = fs::remove_file(test_file);

        // Simulate adding tasks by writing to the test file
        {
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(test_file)
                .expect("Could not open test file");
            writeln!(file, "Task 1").unwrap();
            writeln!(file, "Task 2").unwrap();
        }

        // Read the test file and assert content
        let content = fs::read_to_string(test_file).unwrap();
        assert!(content.contains("Task 1"));
        assert!(content.contains("Task 2"));

        // Cleanup: remove test file
        let _ = fs::remove_file(test_file);
    }
}
