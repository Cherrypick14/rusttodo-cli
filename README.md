# 📝 Rusty To-Do CLI

A minimal command-line To-Do list manager written in Rust — because your tasks deserve a bit of systems programming!

## 🚀 Features

- ✅ Add new tasks
- 📋 View your current to-do list
- 🗂️ Persistent storage using a simple `todo.txt` file
- ⚡ Fast, lightweight, and dependency-free

---

## 🛠️ Getting Started

### 1. **Clone the repo**
```bash
git clone https://github.com/Cherrypick14/rusty-todo.git
cd rusty-todo
```

### 2. **Build the project**
```bash
cargo build --release
```

### 3. **Run it**
```bash
# Add a new task
cargo run -- add "Write blog post about Rust"

# List tasks
cargo run -- list
```

> The tasks will be saved in a `todo.txt` file in the same directory.

---

## 🧪 Example Output

```bash
$ cargo run -- add "Buy milk"
✅ Added task: Buy milk

$ cargo run -- list
📝 Your tasks:
1. Buy milk
```

---

## 📁 File Structure

```
.
├── src
│   └── main.rs        # Main CLI logic
├── todo.txt           # Generated at runtime to store tasks
└── Cargo.toml         # Rust project config
```

---

## 🧠 Why This Project?

This project is ideal for:

- Practicing Rust’s file and argument handling
- Learning how to build simple CLI tools
- Keeping track of your personal tasks like a true Rustacean 🦀

---

## ✨ Future Improvements

- [ ] Mark tasks as complete
- [ ] Delete tasks
- [ ] Add timestamps or priorities
- [ ] Improve error handling & UX

---

