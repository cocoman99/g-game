use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("+++ Guess THE Number +++");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Input your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line"); // handling potential failure

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
            
        println!("YOU guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("YOU WIN!");
                break;
            }
        }
    }
}