
use std::fs;
use std::io;

//Step 1 - Data Strructure
#[derive(Debug, Clone)]
struct Task {
    description: String,
    completed: bool,
}

fn load_tasks_from_file(path: &str) -> io::Result<Vec<Task>> {
    let content = fs::read_to_string(path).unwrap_or_else(|_| "".to_string());
    let mut tasks = Vec::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() == 2 {
            let completed = parts[0] == "1";
            let description = parts[1].to_string();
            tasks.push(Task { description, completed });
        }
    }

    Ok(tasks)
}

fn save_tasks_to_file(path: &str, tasks: &Vec<Task>) -> io::Result<()> {
    let content = tasks
        .iter()
        .map(|task| format!("{}|{}", task.completed as u8, task.description))
        .collect::<Vec<String>>()
        .join("\n");

    fs::write(path, content)?;

    Ok(())
}

fn read_tasks_from_file(path: &str) -> io::Result<Vec<Task>> {
    let content = fs::read_to_string(path)?;

    if content.is_empty() {
        return Ok(vec![]); // Return an empty list if file is empty
    }

    let tasks = content
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split('|').collect();
            Task {
                completed: parts[0] == "1",
                description: parts[1].to_string(),
            }
        })
        .collect();

    Ok(tasks)
}

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



fn main() {
    println!("Hello, world!");
}
