use std::io; 
use rand::Rng; 
use std::cmp::Ordering;
fn main() {
    /*** Dynamic guessing game  */
    let secret_number =  rand::rng().random_range(1..=100); //Values can take on between 0 and 100 inclusive 
    println!("Secret number {}", secret_number);
    println!("This is a secret guessing game. Values are between 1 and 100. ");
    loop {
        let mut guess = String::new(); 
        io::stdin().read_line(&mut guess).expect("Error reading lines.");
        let num_guess: u8 = guess.trim().parse().expect("Error parsing number.");
        match num_guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess is smaller."),
            Ordering::Greater => println!("Your guess is greater."),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}