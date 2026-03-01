use::std::io; 

fn main() {
    let secret_number: u8 = 200; 
    let mut guess = String::new(); 
    println!("This is a secret guessing game. Values are between 0 and 255. ");

    io::stdin().read_line(&mut guess).expect("Error reading lines.");
    let guess_number: u8 = guess.trim().parse().expect("Error parsing number.");

    if guess_number == secret_number {
        println!("You have correctly guessed that the secret number is {}", guess_number);
    } else {
        println!("You have incorrectly guessed the secret number.");
    }
}