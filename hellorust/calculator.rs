use std::io; 

fn main() {

    let mut user_input_1: i32; 
    let mut user_input_2: i32; 
    let mut method = String::new();

    println!("What is the first number?"); 
    // There are several issues in this selection:
    // 1. io::stdout().flush.expect("Error"); is incorrect. 'flush' is a method, not a property, so it needs parentheses: flush().
    // 2. io::stdi() is a typo, it should be io::stdin().
    // 3. read_line(&mut user_input_1).parse(); is not correct usage. read_line expects a &mut String, and then the string should be parsed to i32.
    // 4. user_input_1 and user_input_2 are declared as i32 but read_line reads into String.
    // 5. There's a semicolon inside println!("What is the second number?"; ) that should not be there.
    // 6. Method is never read from user input or assigned.

    use std::io::Write; // flush() needs this trait

    let mut input1 = String::new();
    io::stdout().flush().expect("Error");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let user_input_1: i32 = input1.trim().parse().expect("Please enter a number");

    println!("What is the second number?");
    io::stdout().flush().expect("Error");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let user_input_2: i32 = input2.trim().parse().expect("Please enter a number");

    println!("Which method? (add, multiply, divide, subtract)");
    io::stdout().flush().expect("Error");
    method.clear();
    io::stdin().read_line(&mut method).expect("Failed to read input");
    let method = method.trim();
    if method == "add" {
        println!("{}", user_input_1 + user_input_2);
    } else if method == "multiply" {
        println!("{}", user_input_1 * user_input_2); 
    } else if method == "divide" { 
        println!("{}", user_input_1 / user_input_2); 
    } else if method == "subtract" { 
        println!("{}", user_input_1 - user_input_2)
    }

}