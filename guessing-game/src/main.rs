use std::io; // imports a library

fn main() {
    println!("This is the start of a guessing game!");
    println!("Enter a guess number");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read lines.");
    let num_guess: i32 = guess.trim().parse().expect("Expect a number");

    println!("You guessed: {}", num_guess);
}

