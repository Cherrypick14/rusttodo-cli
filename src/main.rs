use std::env;
use std::fs::{OpenOptions, read_to_string};
use std::io::{self, Write};

fn main() {
    // Collect command-line arguments into a vector
    let args: Vec<String> = env::args().collect();

    // If no command is provided, display usage instructions
    if args.len() < 2 {
        eprintln!("Usage:");
        eprintln!("  add <task>     - Add a new task");
        eprintln!("  list           - List all tasks");
        return;
    }

    // Read the command (e.g., "add" or "list")
    let command = &args[1];

    match command.as_str() {
        "add" => {
            // Check if task text is provided
            if args.len() < 3 {
                eprintln!("Please provide a task to add.");
                return;
            }

            // Join the rest of the args into one task string
            let task = &args[2..].join(" ");
            add_task(task);
        }
        "list" => {
            list_tasks();
        }
        _ => {
            // Handle unknown commands
            eprintln!("Unknown command: {}", command);
            eprintln!("Available commands: add, list");
        }
    }
}

// Appends a new task to the "todo.txt" file
fn add_task(task: &str) {
    let mut file = OpenOptions::new()
        .create(true)   // Create the file if it doesn't exist
        .append(true)   // Append to the file if it exists
        .open("todo.txt")
        .expect("Could not open file");

    writeln!(file, "{}", task).expect("Could not write to file");
    println!("✅ Added task: {}", task);
}

// Reads and lists all tasks from "todo.txt"
fn list_tasks() {
    let content = read_to_string("todo.txt");

    match content {
        Ok(data) => {
            if data.trim().is_empty() {
                println!("📭 No tasks yet!");
            } else {
                println!("📝 Your tasks:");
                for (i, line) in data.lines().enumerate() {
                    println!("{}. {}", i + 1, line);
                }
            }
        }
        Err(_) => {
            // Handle case where file doesn't exist or can't be read
            println!("🗂️ No tasks found. Start by adding one!");
        }
    }
}
