use std::io::{Write, stdin};

use crate::task::{Task};

pub fn list_all_tasks(tasks: &mut Vec<Task>) -> String{
    let mut tasks_string: String = "Tasks: \n".to_string();
    for task in tasks
    {
        tasks_string.push_str(&format!("\t{}\n", &task.name));
    }
    tasks_string.trim().to_string()
}

pub fn open_task(tasks: &mut Vec<Task>){
    print!("Task name: ");
    _ = std::io::stdout().flush();
    let mut buffer: String = String::new();
    stdin().read_line(&mut buffer).expect("Failed to read line");
    let target_name = buffer.trim();

    let found_task = tasks.iter().find(|task| task.name == target_name);
    match found_task {
        Some(task) => {
            println!("Task found: \n {}\n{}", task.name, task.body);
        }
        None => {
            println!("No task found");
        }
    }
}

pub fn create_task(tasks: &mut Vec<Task>){
    let mut buffer: String = String::new();
    print!("Task name: ");
    _ = std::io::stdout().flush();
    stdin().read_line(&mut buffer).expect("Failed to read line");
    let name: String = buffer.trim().to_string();
    buffer.clear();
    println!("Task body: ");
    stdin().read_line(&mut buffer).expect("Failed to read lines");
    let body: String = buffer.trim().to_string();
    let new_task: Task = Task::new(name, body);
    tasks.push(new_task);
    
}

