
use std::fs;
use std::io::{self, Write};

//Step 1 - Data Strructure
#[derive(Debug, Clone)]
struct Task {
    description: String,
    completed: bool,
}

// fn load_tasks_from_file(path: &str) -> io::Result<Vec<Task>> {
//     let content = fs::read_to_string(path).unwrap_or_else(|_| "".to_string());
//     let mut tasks = Vec::new();

//     for line in content.lines() {
//         let parts: Vec<&str> = line.split('|').collect();
//         if parts.len() == 2 {
//             let completed = parts[0] == "1";
//             let description = parts[1].to_string();
//             tasks.push(Task { description, completed });
//         }
//     }

//     Ok(tasks)
// }

// fn save_tasks_to_file(path: &str, tasks: &Vec<Task>) -> io::Result<()> {
//     let content = tasks
//         .iter()
//         .map(|task| format!("{}|{}", task.completed as u8, task.description))
//         .collect::<Vec<String>>()
//         .join("\n");

//     fs::write(path, content)?;

//     Ok(())
// }

// fn read_tasks_from_file(path: &str) -> io::Result<Vec<Task>> {
//     let content = fs::read_to_string(path)?;

//     if content.is_empty() {
//         return Ok(vec![]); // Return an empty list if file is empty
//     }

//     let tasks = content
//         .lines()
//         .map(|line| {
//             let parts: Vec<&str> = line.split('|').collect();
//             Task {
//                 completed: parts[0] == "1",
//                 description: parts[1].to_string(),
//             }
//         })
//         .collect();

//     Ok(tasks)
// }

fn mark_task_complete(path: &str, task_number: usize) -> io::Result<()> {
    let content = fs::read_to_string(path)?;
    let mut tasks: Vec<Task> = content
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() == 2 {
                Some(Task {
                    completed: parts[0] == "1",
                    description: parts[1].to_string(),
                })
            } else {
                None
            }
        })
        .collect();

    // Check if the task_number is valid
    if task_number == 0 || task_number > tasks.len() {
        println!("Invalid task number!");
        return Ok(());
    }

    // Mark the task as complete (using 1-based indexing)
    tasks[task_number - 1].completed = true;

    // Convert back to string format and write to file
    let new_content: String = tasks
        .iter()
        .map(|task| format!("{}|{}", if task.completed { "1" } else { "0" }, task.description))
        .collect::<Vec<_>>()
        .join("\n");

    fs::write(path, new_content)?;
    println!("Task {} marked as complete!", task_number);

    Ok(())
}

fn list_tasks(path: &str) -> io::Result<()> {
    let content = fs::read_to_string(path)?;
    let tasks: Vec<Task> = content
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() == 2 {
                Some(Task {
                    completed: parts[0] == "1",
                    description: parts[1].to_string(),
                })
            } else {
                None
            }
        })
        .collect();

    if tasks.is_empty() {
        println!("No tasks found!");
    } else {
        println!("📌 Task List:");
        for (index, task) in tasks.iter().enumerate() {
            let status = if task.completed { "✅" } else { "❌" };
            println!("{}: {} - {}", index + 1, status, task.description);
        }
    }

    Ok(())
}

fn add_task(path: &str, description: String) -> io::Result<()> {
    let mut tasks = fs::read_to_string(path).unwrap_or_else(|_| String::new());

    if !tasks.is_empty() {
        tasks.push('\n'); // Ensure tasks are separated by new lines
    }

    tasks.push_str(&format!("0|{}", description)); // `0` means task is incomplete

    fs::write(path, tasks)?;
    println!("✅ Task added successfully!");

    Ok(())
}



fn main() {
    let path = "tasks.txt";

    loop {
        println!("\n📝 To-Do List Manager");
        println!("1️⃣ Add a Task");
        println!("2️⃣ List Tasks");
        println!("3️⃣ Mark Task as Complete");
        println!("4️⃣ Exit");
        print!("👉 Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                print!("Enter task description: ");
                io::stdout().flush().unwrap();
                let mut description = String::new();
                io::stdin().read_line(&mut description).unwrap();
                let description = description.trim().to_string();

                add_task(path, description).unwrap();
            }
            "2" => {
                list_tasks(path).unwrap();
            }
            "3" => {
                print!("Enter task number to mark complete: ");
                io::stdout().flush().unwrap();
                let mut task_number = String::new();
                io::stdin().read_line(&mut task_number).unwrap();
                let task_number: usize = task_number.trim().parse().unwrap_or(0);

                mark_task_complete(path, task_number).unwrap();
            }
            "4" => {
                println!("👋 Exiting...");
                break;
            }
            _ => {
                println!("⚠ Invalid choice, please try again.");
            }
        }
    }
}



