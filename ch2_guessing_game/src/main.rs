use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    let debug: bool = false;

    // Prompt the user for input

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    if debug {
        println!("The secret number is: {secret_number}");
    }

    // Continue prompting user until they figure it out
    loop {

        println!("Please input your guess.");

        let mut guess = String::new();

        // Process user input

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // Convert user input to u32
        // Shadows previous `guess` var with a new one
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };

        // Return user input

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
} 
