use std::io;
use std::io::Write; // the trait that flush() is in

fn main() {
    // setup vars
    let target: i32 = 76;

    print_starting_text();
    game_loop();
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
        .unwrap();
    return_val = return_val.trim().to_string();
    return return_val;
}

fn game_loop () -> () {
    loop {
        let user_input = get_user_input();
        println!("{user_input}");
        break;
    }
}
