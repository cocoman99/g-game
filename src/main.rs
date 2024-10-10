use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("\n+ + +   G U E S S   T H E   N U M B E R   + + +\n");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut count = 0;
    let prize = 1000000;

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
        count += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("\n\nTHE LUCKY NUMBER IS {secret_number}.");                
                break;
            }
        }
    }
    
    println!("\nYOU WON AFTER {count} TRIES AND YOUR PRIZE IS {prize} $$$$$$$");
    println!("\nPRESS ENTER TO EXIT");
    let _ = io::stdin().read_line(&mut String::new());
}