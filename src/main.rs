extern crate rand;

use std::cmp::Ordering;
use std::io;
use std::process;
use rand::Rng;

fn game_init() -> Result<u32, String> {

    println!("Choose top range");

    let mut top_range = String::new();
    io::stdin().read_line(&mut top_range).expect(
        "Failed to read line",
    );

    let top_range = top_range.trim().parse::<u32>().map_err(
        |err| err.to_string(),
    )?;

    println!("Guess the number from 1 to {}", top_range);

    let secret_number = rand::thread_rng().gen_range(1, top_range);

    println!("The secret number is: {}", secret_number);

    return Ok(secret_number);
}

fn game_loop(secret_number: u32) {
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect(
            "Failed to read line",
        );

        let parsed_guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match parsed_guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn main() {
    let secret_number = game_init().unwrap_or_else(|err| {
        println!("Error innitialising the game: {}", err);
        process::exit(1);
    });

    game_loop(secret_number);
}
