# 📝 Rust To-Do List Manager

A simple command-line To-Do List Manager written in Rust. This program allows you to add, list, and mark tasks as complete, storing them in a `tasks.txt` file.

---

## 🚀 Features
- ✅ Add new tasks
- 📋 List all tasks with completion status
- ✏️ Mark tasks as complete
- 📂 Stores tasks persistently in a text file

---

## 🛠 Installation & Setup
### Prerequisites
- Rust installed ([Install Rust](https://www.rust-lang.org/tools/install))

### Clone the Repository
```sh
git clone https://github.com/Webrowse/rust-cli-todo.git
cd rust-todo-list
```

### Build the Project
```sh
cargo build --release
```

### Run the Program
```sh
cargo run
```

---

## 📌 Usage
Once you run the program, follow the on-screen menu options:

1️⃣ **Add a Task** – Enter a task description to add it to the list.
```sh
Enter task description: Buy groceries
✅ Task added successfully!
```

2️⃣ **List Tasks** – Displays all tasks with completion status.
```sh
📌 Task List:
1: ❌ Buy groceries
2: ❌ Finish Rust project
```

3️⃣ **Mark Task as Complete** – Enter the task number to mark it as done.
```sh
Enter task number to mark complete: 1
Task 1 marked as complete!
```

4️⃣ **Exit** – Quit the application.

---

## 📂 Data Storage
- Tasks are stored in `tasks.txt` in the format: `completed|description`
- Example:
```
0|Buy groceries
1|Finish Rust project
```
- `0` → Incomplete, `1` → Completed

---

## 📜 License
This project is open-source and available under the MIT License.

Happy coding! 🚀

