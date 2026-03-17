fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    
    // Task 1: Calculate sum without taking ownership
    let total = calculate_sum(&numbers);
    
    // Task 2: Double each number in the vector
    double_values(&mut numbers);
    
    // Task 3: Print both the original and doubled values
    println!("Original sum: {}", total);
    println!("Doubled values: {:?}", numbers);
    
    // Task 4: Add new numbers to the vector
    add_numbers(&mut numbers, vec![6, 7, 8]);
    println!("After adding: {:?}", numbers);
}

fn calculate_sum(v: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for num in v {
        sum += num;
    }
    sum
}

fn double_values(v: &mut Vec<i32>) {
    for num in v {
        *num *= 2;
    }
}

fn add_numbers(v: &mut Vec<i32>, new_nums: Vec<i32>) {
    for num in new_nums {
        v.push(num);
    }
}