// TODO: Implement this function
fn find_first_even(numbers: &Vec<i32>) -> Option<i32> {
    for number in numbers { 
        if number % 2 == 0 {
            return Some(*number);  
        } 
    }
    None
}

fn main() {
    let numbers1 = vec![1, 3, 5, 7];
    let numbers2 = vec![1, 3, 6, 7];

    match find_first_even(&numbers1) {
        Some(n) => println!("Found even number: {}", n),
        None => println!("No even numbers found"),
    };

    let res2 = find_first_even(&numbers2); 
    // TODO: Use unwrap_or() to print the result with a default value of -1
    println!("First even in numbers2: {}", res2.unwrap_or(-1));
}