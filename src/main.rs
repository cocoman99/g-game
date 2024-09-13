use std::io;

fn main() {
    println!("+++ Guess THE Number +++");
    println!("Input your guess: ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read the line");

    println!("YOU guessed: {}", guess);
}