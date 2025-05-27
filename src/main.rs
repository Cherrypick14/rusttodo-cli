use std::env;
use std::fs::{OpenOptions, read_to_string};
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage:");
        eprintln!("  add <task>     - Add a new task");
        eprintln!("  list           - List all tasks");
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "add" => {
            if args.len() < 3 {
                eprintln!("Please provide a task to add.");
                return;
            }

            let task = &args[2..].join(" ");
            add_task(task);
        }
        "list" => {
            list_tasks();
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            eprintln!("Available commands: add, list");
        }
    }
}

fn add_task(task: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("todo.txt")
        .expect("Could not open file");

    writeln!(file, "{}", task).expect("Could not write to file");
    println!("✅ Added task: {}", task);
}

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
            println!("🗂️ No tasks found. Start by adding one!");
        }
    }
}
