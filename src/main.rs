use std::io;
use rand::Rng;

fn main() {
    println!("+++ Guess THE Number +++");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");
    
    println!("Input your guess: ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read the line"); // handling potential failure

    println!("YOU guessed: {}", guess);
}