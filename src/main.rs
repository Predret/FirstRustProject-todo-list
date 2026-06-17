use std::io::{self};
use std::cmp::Ordering;

fn guess(number: u8) -> bool
{
    println!("What is your guess?\n");
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input_number: u8 = match input.trim().parse()
    {
        Ok(numb) => numb,
        Err(_) =>
        {
            println!("That's not a number.");
            return false;
        }
    };
    
    match input_number.cmp(&number)
    {
        Ordering::Less =>
        {
            println!("Your number is too low.");
            false
        }
        Ordering::Greater =>
        {
            println!("Your number is too high.");
            false
        }
        Ordering::Equal =>
        {
            true
        }
    }
}

fn main() {

    let number: u8 = rand::random_range(1..=100);
    println!("{}", number);
    let mut tries: u16 = 1;

    while !guess(number) {tries += 1};

    println!("Ey nice job! It took you {}", tries);

    let _ = io::stdin();
}
