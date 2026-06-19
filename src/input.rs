use std::io::{self, Write};
pub enum InputTypes {
    Help,
    List,
    Open,
    Edit,
    Create,
    ExitApp,
    Unknown,
}

pub fn get_input() -> InputTypes {
    let mut input: String = String::new();
    print!("Command: ");
    _ = std::io::stdout().flush();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let trimmed_input: &str = &str::to_lowercase(input.trim());

    match trimmed_input{
        "help"   => InputTypes::Help,
        "list"   => InputTypes::List,
        "open"   => InputTypes::Open,
        "edit"   => InputTypes::Edit,
        "create" => InputTypes::Create,
        "exit"   => InputTypes::ExitApp,
        _        => InputTypes::Unknown,
    }
}