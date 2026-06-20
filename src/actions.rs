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
    if target_name.is_empty(){
        println!("Cancelled");
        return;
    }
    let Some(found_task)  = tasks.iter_mut().find(|task| task.name == target_name)
    else{
        println!("No task found; cancelled.");
        return;
    };
    print!("(Tap enter to skip renaming) Change name to: ");
    _ = std::io::stdout().flush();
    buffer.clear();
    stdin().read_line(&mut buffer).expect("Failed to read line");
    let name_changed_to: String = buffer.trim().to_string();
    if name_changed_to.is_empty(){
        println!("Skipped renaming");
    }
    else {
        found_task.name = name_changed_to;
    }
    println!("(Tap enter to skip) Change body to: ");
    buffer.clear();
    stdin().read_line(&mut buffer).expect("Failed to read line");
    let body_changed_to: String = buffer.trim().to_string();
    if body_changed_to.is_empty()
    {
        println!("Skipped");
        return;
    }
    found_task.body = body_changed_to;
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
    let found_task = tasks.iter().find(|task| task.name == target_name);
    match found_task {
        Some(task) => {
            println!("Deleted task: {}", task.name);
        }
        None => {
            println!("Could not find task to delete.");
        }
    }
    tasks.retain(|task| task.name != target_name);

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

