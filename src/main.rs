extern crate rand;

use std::cmp::Ordering;
use std::io;
use std::process;
use rand::Rng;

struct Config {
    secret_number: u32,
    top_range: u32,
}

impl Config {
    fn new() -> Result<Config, String> {
        println!("Choose top range");

        let mut top_range = String::new();
        io::stdin().read_line(&mut top_range).expect(
            "Failed to read line",
        );

        let top_range = top_range.trim().parse::<u32>().map_err(
            |err| err.to_string(),
        )?;

        let secret_number = rand::thread_rng().gen_range(1, top_range);

        Ok(Config{secret_number: secret_number, top_range: top_range})
    }
}

fn game_loop(game_config: Config) {
    println!("Guess the number from 1 to {}", game_config.top_range);

    println!("The secret number is: {}", game_config.secret_number);

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

        match parsed_guess.cmp(&game_config.secret_number) {
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
    let game_config = Config::new().unwrap_or_else(|err| {
        println!("Error innitialising the game: {}", err);
        process::exit(1);
    });
    game_loop(game_config);
}
