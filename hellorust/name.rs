use std::io;
use std::io::Write;

fn main() {
    let mut user_input = String::new();
    print!("What's your name? ");
    io::stdout().flush().expect("Error flushing");
    let _ =io::stdin().read_line(&mut user_input);
    println!("Hello, {}!", user_input.trim());
}
