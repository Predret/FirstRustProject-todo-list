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
    print!("(Tap enter to cancel) Task name: ");
    _ = std::io::stdout().flush();
    let mut buffer: String = String::new();
    stdin().read_line(&mut buffer).expect("Failed to read line");
    let target_name: String = buffer.trim().to_string();
    if target_name.is_empty()
    {
        println!("Cancelled");
        return;
    }
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

pub fn edit_task(tasks: &mut Vec<Task>){
    print!("(Tap enter to cancel) Task to edit: ");
    _ = std::io::stdout().flush();
    let mut buffer: String = String::new();
    stdin().read_line(&mut buffer).expect("Failed to read line");
    let target_name: String = buffer.trim().to_string();
    if target_name.is_empty()
    {
        println!("Cancelled");
        return;
    }
}

pub fn delete_task(tasks: &mut Vec<Task>){
    print!("(Tap enter to cancel) Task to delete: ");
    _ = std::io::stdout().flush();
    let mut buffer: String = String::new();
    stdin().read_line(&mut buffer).expect("Failed to read line");
    let target_name: String = buffer.trim().to_string();
    if target_name.is_empty()
    {
        println!("Cancelled deletion.");
        return;
    }
    tasks.retain(|task| task.name != target_name);
    let found_task = tasks.iter().find(|task| task.name == target_name);
    match found_task {
        Some(task) => {
            println!("Deleted task: {}", task.name);
        }
        None => {
            println!("Could not find task to delete.");
        }
    }

}

pub fn create_task(tasks: &mut Vec<Task>){
    let mut buffer: String = String::new();
    print!("(Tap enter to cancel) Task name: ");
    _ = std::io::stdout().flush();
    stdin().read_line(&mut buffer).expect("Failed to read line");
    let name: String = buffer.trim().to_string();
    if name.is_empty()
    {
        println!("Cancelled");
        return;
    }
    let found_task = tasks.iter().find(|task| task.name == name);
    if found_task.is_some() {
        println!("Task already exists!");
        return;
    }
    buffer.clear();
    println!("Task body: ");
    stdin().read_line(&mut buffer).expect("Failed to read lines");
    let body: String = buffer.trim().to_string();
    let new_task: Task = Task::new(name, body);
    tasks.push(new_task);
    
}

