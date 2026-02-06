use std::io;
use rand::Rng; // from rand import rng 
use std::cmp::Ordering; 

fn main() {
    let secret_number = rand::rng().random_range(1..=100);
    
    
    println!("Guess the number between 1 and 100!");
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let num_guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;  // Skip to next loop iteration
            }
        };
       

        match num_guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;},
        };
    };

}