mod input;
mod task;
mod actions;

use input::*;
use task::*;

use crate::actions::{create_task, list_all_tasks, open_task};
 
fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    let help_message: String = String::from(
"Commands:
    Help: Get commands.
    List: List every task.
    Open: Open a task.
    Edit: Edit a task.
    Delete: Delete a task.
    Create: Create a task.
    ExitApp: Exit the application.");
    loop{
        let command: InputTypes = input::get_input();
        match command{
            InputTypes::Help => println!("{}", help_message),
            InputTypes::List => println!("{}", list_all_tasks(&mut tasks)),
            InputTypes::Open => open_task(&mut tasks),
            InputTypes::Edit => println!("editing stuff"),
            InputTypes::Delete => println!("deleting stuff"),
            InputTypes::Create => create_task(&mut tasks),
            InputTypes::ExitApp => break,
            _ => println!("Unknown command. Type 'help' for info. "),
        }
    }
}
