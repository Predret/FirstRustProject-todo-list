mod input;
mod task;
mod list;

use input::*;
fn main() {
    let help_message: String = String::from(
"Commands:
    Help: Get commands.
    List: List every task.
    Open: Open a file.
    Exit: Exit the application.");
    loop{
        let command: InputTypes = input::get_input();
        match command{
            InputTypes::Help => println!("{}", help_message),
            InputTypes::List => println!("listing stuff"),
            InputTypes::Open => println!("opening stuff"),
            InputTypes::Edit => println!("editing stuff"),
            InputTypes::Create => println!("creating stuff"),
            InputTypes::ExitApp => break,
            _ => println!("Unknown command. Type 'help' for info. "),
        }
    }
}

