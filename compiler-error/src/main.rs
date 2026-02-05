use std::io::{self, Write};

fn main() {
    println!("Enter integers, one per line. Empty line to finish."); // add a semicolon to indicate end of subfunction

    let mut nums: Vec<i32> = Vec::new(); // add a semicolumn to indicate end of this declarative statement ; convert it to a mutable

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() { return; }

        let trimmed = input.trim();

        if trimmed.is_empty() {  // have to use curly braces for everything nested under the if statement
          break; 
        }
        match trimmed.parse::<i32>() {
            Ok(n) => nums.push(n),
            Err(_) => println!("Please enter a valid integer."),
        }
    }

    if nums.is_empty() {
        println!("No numbers entered.");
    } else {
        let sum: i32 = nums.iter().sum();
        let avg = sum as f64 / nums.len() as f64;
        println!("Count = {}, Sum = {}, Average = {:.2}", nums.len(), sum, avg); // Print requires specifying parameters after the string value (substitutes curly braces)
    }
}