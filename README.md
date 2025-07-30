# Task Manager CLI in Rust

A simple command-line Task Manager application built using Rust. It allows you to add, delete, update, and list tasks, all stored persistently in a local JSON file.

---

## 🧰 Features

- Add new tasks with auto-incremented IDs
- Delete tasks by ID
- Update the status of existing tasks
- Automatically remove tasks when marked as "complete"
- View all tasks in a neatly formatted list
- Persistent storage using JSON
- Colored terminal output for clarity (without emojis)

---

## 📦 Dependencies

Add these to your `Cargo.toml`:

```toml
[dependencies]
clap = { version = "4.0", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
colored = "2.0"
