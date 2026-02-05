use std::io; // imports a library

fn main() {
    println!("This is the start of a guessing game!");
    println!("Enter a guess number");
    let trial_total = 5; 
    let true_guess = 21; 
    let mut trial_num = 0; 
    let mut guessed_values: Vec<i32> = Vec::new(); 

    while trial_num <= trial_total {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read lines.");
        let num_guess: i32 = guess.trim().parse().expect("Expect a number");
           
        println!("You guessed: {}", num_guess);
        println!("Your previous guesses are {:?}", guessed_values);
        if num_guess == true_guess {
            println!("You have successfully guessed the number {}", true_guess);
            break;
        } else {
            guessed_values.push(num_guess);
            println!("You have guessed an incorrect number {}", num_guess);
        }

        trial_num = trial_num + 1
    };

}

