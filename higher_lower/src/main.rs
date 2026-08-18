use std::io::{self, Write}; // Write trait contains flush()
// The rand crate provides higher level functionality, for example generation of 
// floating-point values, uniform ranged sampling and shuffling sequences. In 
// particular, rand::RngExt is an extension trait over Rng providing many of the 
// methods one might expect to be able to use on an RNG.
//     https://docs.rs/rand/latest/rand/trait.Rng.html
//     https://docs.rs/rand/latest/rand/trait.RngExt.html
use rand::RngExt;

fn main() {
    let mut rng = rand::rng(); // you can't explict type this without out first importing ThreadRng
    let target: i32 = rng.random_range(0..100);

    print_starting_text();
    game_loop(target);
    print_ending_text();
}

fn print_starting_text() -> () {
    println!("Welcome to Higher-Lower!");
    println!("Try to guess the number that I'm thinking of.");
    println!("");
}

fn print_ending_text() -> () {
    println!("You won!");
    println!("Please re-load the game to play again.");
}

fn get_user_input() -> String {
    let mut return_val: String = String::new();
    print!("Enter a number [1-100]: ");
    io::stdout().flush().expect("flushed");
    io::stdin()
        .read_line(&mut return_val)
        .expect("Failed to read line");
    return_val = return_val.trim().to_string();
    return return_val;
}

fn game_loop(target: i32) -> () {
    loop {
        let user_input = get_user_input();
        let guess = user_input.parse::<i32>().unwrap();
        println!("{user_input}");

        match evaluate_guess(target, guess) {
            0 => println!("Too Low!"),
            1 => println!("Too High!"),
            2 => break,
            _ => println!("What have you done?!"),
        }
    }
}

// If guess is...
//   Too low, return 0
//   Too high, return 1
//   Correct, return 3
fn evaluate_guess(target: i32, guess: i32) -> i32 {
    if target > guess {
        return 0;
    } else if target < guess {
        return 1;
    } else {
        return 2;
    }
}
