
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






fn main() {
    println!("Hello, world!");
}
