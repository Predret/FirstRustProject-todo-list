use std::io::{self};

fn main() {

    let x: i32 = 5;

    println!("Hello, world! {}", x);

    println!("Please input something here: ");
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("Your input: {}", input);

}
